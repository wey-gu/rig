//! Shared JSON-schema sanitizer for providers with strict structured-output
//! restrictions (OpenAI, Anthropic). The common core enforces
//! `additionalProperties: false`, all-properties-required, recursion into
//! `$defs`/`properties`/`items`/combinators, and the oneOf→anyOf merge;
//! [`SanitizeOptions`] flags carry the per-provider deltas.

/// Per-provider deltas applied by [`sanitize_schema`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct SanitizeOptions {
    /// OpenAI does not allow sibling keywords next to `$ref`; strip everything
    /// except `$ref` and stop descending into that node.
    pub(crate) strip_ref_siblings: bool,
    /// OpenAI requires `properties` on all object schemas, even empty ones.
    pub(crate) inject_empty_properties: bool,
    /// Anthropic does not support numerical constraints on integer/number
    /// types (`minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`,
    /// `multipleOf`).
    pub(crate) strip_numeric_constraints: bool,
}

/// Recursively rewrites `schema` in place to respect a provider's structured
/// output restrictions. See [`SanitizeOptions`] for the provider-specific
/// behavior.
pub(crate) fn sanitize_schema(schema: &mut serde_json::Value, options: SanitizeOptions) {
    use serde_json::Value;

    if let Value::Object(obj) = schema {
        if options.strip_ref_siblings && obj.contains_key("$ref") {
            obj.retain(|k, _| k == "$ref");
            return;
        }

        if let Some(additional_properties) = obj.remove("additional_properties") {
            obj.entry("additionalProperties".to_string())
                .or_insert(additional_properties);
        }
        let is_object_schema = declares_object_type(obj.get("type"))
            || obj.contains_key("properties")
            || obj.contains_key("additionalProperties");

        if is_object_schema {
            obj.entry("type".to_string())
                .or_insert_with(|| Value::String("object".to_string()));
            if options.inject_empty_properties {
                obj.entry("properties".to_string())
                    .or_insert_with(|| Value::Object(Default::default()));
            }
            obj.insert("additionalProperties".to_string(), Value::Bool(false));
        }

        if let Some(Value::Object(properties)) = obj.get("properties") {
            let prop_keys = properties.keys().cloned().map(Value::String).collect();
            obj.insert("required".to_string(), Value::Array(prop_keys));
        }

        if options.strip_numeric_constraints {
            let is_numeric_schema = obj.get("type") == Some(&Value::String("integer".to_string()))
                || obj.get("type") == Some(&Value::String("number".to_string()));

            if is_numeric_schema {
                for key in [
                    "minimum",
                    "maximum",
                    "exclusiveMinimum",
                    "exclusiveMaximum",
                    "multipleOf",
                ] {
                    obj.remove(key);
                }
            }
        }

        if let Some(defs) = obj.get_mut("$defs")
            && let Value::Object(defs_obj) = defs
        {
            for (_, def_schema) in defs_obj.iter_mut() {
                sanitize_schema(def_schema, options);
            }
        }

        if let Some(properties) = obj.get_mut("properties")
            && let Value::Object(props) = properties
        {
            for (_, prop_value) in props.iter_mut() {
                sanitize_schema(prop_value, options);
            }
        }

        if let Some(items) = obj.get_mut("items") {
            sanitize_schema(items, options);
        }

        // Neither provider supports oneOf; convert to anyOf, merging into an
        // existing anyOf array if present.
        if let Some(one_of) = obj.remove("oneOf") {
            match obj.get_mut("anyOf") {
                Some(Value::Array(existing)) => {
                    if let Value::Array(mut incoming) = one_of {
                        existing.append(&mut incoming);
                    }
                }
                _ => {
                    obj.insert("anyOf".to_string(), one_of);
                }
            }
        }

        for key in ["anyOf", "allOf"] {
            if let Some(variants) = obj.get_mut(key)
                && let Value::Array(variants_array) = variants
            {
                // A constraint-only branch (`{"required": ["id"]}`, the usual
                // "one of these selectors" idiom) is not a schema the strict
                // subset accepts: it has no type, so it can never carry
                // `additionalProperties: false`, and every property is
                // already required above. Drop it rather than send a branch
                // the provider rejects (`In context=('anyOf', '0'),
                // 'additionalProperties' is required`).
                variants_array.retain(|variant| !is_constraint_only(variant));
                for variant in variants_array.iter_mut() {
                    sanitize_schema(variant, options);
                }
            }
            if matches!(obj.get(key), Some(Value::Array(variants)) if variants.is_empty()) {
                obj.remove(key);
            }
        }
    }
}

/// Whether a `type` keyword admits objects (`"object"` or a type list that
/// contains it, e.g. `["object", "null"]`).
fn declares_object_type(kind: Option<&serde_json::Value>) -> bool {
    match kind {
        Some(serde_json::Value::String(kind)) => kind == "object",
        Some(serde_json::Value::Array(kinds)) => kinds.iter().any(|kind| kind == "object"),
        _ => false,
    }
}

/// Keywords that make a combinator branch describe a value, not only
/// constrain the enclosing one.
const SHAPE_KEYWORDS: &[&str] = &[
    "type",
    "properties",
    "additionalProperties",
    "items",
    "prefixItems",
    "$ref",
    "enum",
    "const",
    "anyOf",
    "oneOf",
    "allOf",
];

fn is_constraint_only(schema: &serde_json::Value) -> bool {
    match schema {
        serde_json::Value::Object(map) => !SHAPE_KEYWORDS.iter().any(|key| map.contains_key(*key)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const OPENAI: SanitizeOptions = SanitizeOptions {
        strip_ref_siblings: true,
        inject_empty_properties: true,
        strip_numeric_constraints: false,
    };

    /// Every object-like node is closed and requires all of its properties,
    /// and no combinator keeps a branch that is not a schema.
    fn assert_strict(schema: &serde_json::Value, path: &str) {
        let Some(obj) = schema.as_object() else {
            return;
        };
        if declares_object_type(obj.get("type")) || obj.contains_key("properties") {
            assert_eq!(
                obj.get("additionalProperties"),
                Some(&json!(false)),
                "{path}"
            );
            let mut keys: Vec<_> = obj["properties"].as_object().unwrap().keys().collect();
            let mut required: Vec<_> = obj["required"]
                .as_array()
                .unwrap()
                .iter()
                .map(|key| key.as_str().unwrap())
                .collect();
            keys.sort();
            required.sort();
            assert_eq!(keys, required, "{path}");
        }
        for (key, value) in obj {
            match value {
                serde_json::Value::Object(_) => assert_strict(value, &format!("{path}/{key}")),
                serde_json::Value::Array(items) => {
                    for (index, item) in items.iter().enumerate() {
                        if matches!(key.as_str(), "anyOf" | "allOf" | "oneOf") {
                            assert!(!is_constraint_only(item), "{path}/{key}/{index}");
                        }
                        assert_strict(item, &format!("{path}/{key}/{index}"));
                    }
                }
                _ => {}
            }
        }
    }

    /// Mem's `thread_fetch_messages` shape: a root `anyOf` of selector
    /// constraints. Copilot's Responses endpoint rejected it with
    /// `In context=('anyOf', '0'), 'additionalProperties' is required`.
    #[test]
    fn constraint_only_selector_branches_are_dropped() {
        let mut schema = json!({
            "type": "object",
            "properties": {
                "thread_id": {"type": "string"},
                "thread_reference": {"type": "string"},
                "limit": {"type": "integer"}
            },
            "required": [],
            "anyOf": [{"required": ["thread_id"]}, {"required": ["thread_reference"]}]
        });
        sanitize_schema(&mut schema, OPENAI);
        assert!(schema.get("anyOf").is_none(), "{schema}");
        assert_strict(&schema, "");
    }

    #[test]
    fn typed_branches_survive_and_are_closed_at_every_depth() {
        let mut schema = json!({
            "type": "object",
            "properties": {
                "filter": {
                    "anyOf": [
                        {"type": "object", "properties": {"ids": {"type": "array", "items": {
                            "type": "object", "properties": {"id": {"type": "string"}}
                        }}}},
                        {"required": ["ids"]},
                        {"type": "null"}
                    ]
                },
                "nullable_object": {"type": ["object", "null"], "properties": {"a": {"type": "string"}}},
                "choice": {"oneOf": [{"type": "object", "properties": {"x": {"type": "integer"}}}, {"type": "string"}]},
                "ref": {"$ref": "#/$defs/Inner"}
            },
            "$defs": {
                "Inner": {"allOf": [{"type": "object", "properties": {"deep": {"type": "object"}}}, {"required": ["deep"]}]}
            }
        });
        sanitize_schema(&mut schema, OPENAI);
        assert_eq!(
            schema["properties"]["filter"]["anyOf"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            schema["properties"]["choice"]["anyOf"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            schema["$defs"]["Inner"]["allOf"].as_array().unwrap().len(),
            1
        );
        assert_strict(&schema, "");
    }

    #[test]
    fn strict_object_schemas_normalize_additional_properties() {
        for key in ["additionalProperties", "additional_properties"] {
            let mut schema = json!({"type": "object", "properties": {
                "filters": {key: {"type": "string"}}
            }, "additionalProperties": true});
            sanitize_schema(
                &mut schema,
                SanitizeOptions {
                    strip_ref_siblings: true,
                    inject_empty_properties: true,
                    strip_numeric_constraints: false,
                },
            );
            assert_eq!(schema["additionalProperties"], false);
            assert_eq!(
                schema["properties"]["filters"],
                json!({
                    "type": "object", "properties": {}, "required": [],
                    "additionalProperties": false
                })
            );
        }
    }
}
