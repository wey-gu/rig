//! Completion endpoint routing for GitHub Copilot.
//!
//! Copilot serves each completion model on a declared subset of endpoints:
//! `GET /models` lists them per model as `supported_endpoints` (for example
//! `["/chat/completions", "/v1/messages"]` or `["/responses"]`). A request sent
//! to an endpoint the model does not serve fails with HTTP 400 and the error
//! code `unsupported_api_for_model`.
//!
//! Routing therefore resolves, in order:
//!
//! 1. a route learned at runtime from such a rejection (the model was
//!    rejected on one route and answered on the other);
//! 2. the route the model declares in the catalog, preferring the
//!    historical choice when it declares both;
//! 3. the model-name rule (`codex` models on `/responses`, everything else on
//!    `/chat/completions`), used only when the catalog is unavailable or does
//!    not list the model.
//!
//! The catalog and the learned routes live in a [`CopilotRouteCatalog`], which
//! a client owns by default. Callers that build many short-lived clients can
//! share one catalog through
//! [`ClientBuilder::route_catalog`](super::ClientBuilder::route_catalog) so the
//! catalog is fetched once per TTL rather than once per client.

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use crate::completion::CompletionError;

/// Error code Copilot returns when a model is called on an endpoint it does
/// not serve.
pub(crate) const UNSUPPORTED_API_FOR_MODEL: &str = "unsupported_api_for_model";

const CHAT_COMPLETIONS_ENDPOINT: &str = "/chat/completions";
const RESPONSES_ENDPOINT: &str = "/responses";

/// How long a successfully fetched catalog is trusted before it is refetched.
pub const DEFAULT_CATALOG_TTL: Duration = Duration::from_secs(10 * 60);
/// How long a failed catalog fetch suppresses further fetch attempts.
pub const DEFAULT_CATALOG_FAILURE_BACKOFF: Duration = Duration::from_secs(60);
/// Upper bound on one catalog fetch, so a slow `/models` cannot stall
/// completions; on timeout the request falls back to the name rule.
pub(crate) const CATALOG_FETCH_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CompletionRoute {
    ChatCompletions,
    Responses,
}

impl CompletionRoute {
    pub(crate) fn path(self) -> &'static str {
        match self {
            Self::ChatCompletions => CHAT_COMPLETIONS_ENDPOINT,
            Self::Responses => RESPONSES_ENDPOINT,
        }
    }

    pub(crate) fn alternate(self) -> Self {
        match self {
            Self::ChatCompletions => Self::Responses,
            Self::Responses => Self::ChatCompletions,
        }
    }
}

/// The name rule: the fallback when the catalog says nothing about a model.
pub(crate) fn route_for_model(model: &str) -> CompletionRoute {
    if model.to_ascii_lowercase().contains("codex") {
        CompletionRoute::Responses
    } else {
        CompletionRoute::ChatCompletions
    }
}

/// The route a model's declared `supported_endpoints` allows.
///
/// When both completion endpoints are declared, the name rule breaks the tie so
/// a model keeps the route it has always used. Returns `None` when neither
/// completion endpoint is declared (for example a `/v1/messages`-only model),
/// leaving the decision to the name rule.
pub(crate) fn declared_route(model: &str, endpoints: &[String]) -> Option<CompletionRoute> {
    let declares = |endpoint: &str| endpoints.iter().any(|e| e.trim() == endpoint);
    match (
        declares(CHAT_COMPLETIONS_ENDPOINT),
        declares(RESPONSES_ENDPOINT),
    ) {
        (true, true) => Some(route_for_model(model)),
        (true, false) => Some(CompletionRoute::ChatCompletions),
        (false, true) => Some(CompletionRoute::Responses),
        (false, false) => None,
    }
}

/// Whether `error` is Copilot refusing this model on the endpoint it was sent
/// to (`400` with code `unsupported_api_for_model`).
///
/// The structured code is read from `error.code` (or a top-level `code`).
/// When a transport layer has decorated the body so it no longer parses as
/// JSON, the quoted code string is matched instead.
pub fn is_unsupported_api_for_model(error: &CompletionError) -> bool {
    if error.provider_response_status() != Some(http::StatusCode::BAD_REQUEST) {
        return false;
    }
    let Some(body) = error.provider_response_body() else {
        return false;
    };
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json) => {
            json.pointer("/error/code")
                .or_else(|| json.get("code"))
                .and_then(serde_json::Value::as_str)
                == Some(UNSUPPORTED_API_FOR_MODEL)
        }
        Err(_) => body.contains(&format!("\"{UNSUPPORTED_API_FOR_MODEL}\"")),
    }
}

type Stamp = Option<std::time::Instant>;

/// `None` on browser wasm, where `Instant::now` is unavailable: a stamp-less
/// entry never expires, so wasm clients fetch the catalog at most once.
fn now() -> Stamp {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        Some(std::time::Instant::now())
    }
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        None
    }
}

fn is_within(stamp: Stamp, window: Duration) -> bool {
    match stamp {
        Some(at) => at.elapsed() < window,
        None => true,
    }
}

enum CatalogSnapshot {
    Loaded {
        at: Stamp,
        endpoints: HashMap<String, Vec<String>>,
    },
    Failed {
        at: Stamp,
    },
}

struct CatalogInner {
    ttl: Duration,
    failure_backoff: Duration,
    /// Declared endpoints per runtime base URL. The async lock is held across
    /// a fetch so concurrent first requests share one `/models` call.
    snapshots: tokio::sync::Mutex<HashMap<String, CatalogSnapshot>>,
    /// Routes learned from `unsupported_api_for_model` rejections, keyed by
    /// `(runtime base URL, model)`. Kept for the catalog's lifetime: runtime
    /// evidence outranks the declared catalog.
    learned: Mutex<HashMap<(String, String), CompletionRoute>>,
}

/// Shared record of which completion endpoint each Copilot model must use.
///
/// Cheap to clone; clones share state. See the [module docs](self) for the
/// resolution order.
#[derive(Clone)]
pub struct CopilotRouteCatalog {
    inner: Arc<CatalogInner>,
}

impl Default for CopilotRouteCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CopilotRouteCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CopilotRouteCatalog")
            .field("ttl", &self.inner.ttl)
            .field("failure_backoff", &self.inner.failure_backoff)
            .finish_non_exhaustive()
    }
}

impl CopilotRouteCatalog {
    /// A catalog with [`DEFAULT_CATALOG_TTL`] and
    /// [`DEFAULT_CATALOG_FAILURE_BACKOFF`].
    pub fn new() -> Self {
        Self::with_ttl(DEFAULT_CATALOG_TTL, DEFAULT_CATALOG_FAILURE_BACKOFF)
    }

    /// A catalog that trusts a fetched model list for `ttl` and, after a failed
    /// fetch, waits `failure_backoff` before trying again.
    pub fn with_ttl(ttl: Duration, failure_backoff: Duration) -> Self {
        Self {
            inner: Arc::new(CatalogInner {
                ttl,
                failure_backoff,
                snapshots: tokio::sync::Mutex::new(HashMap::new()),
                learned: Mutex::new(HashMap::new()),
            }),
        }
    }

    fn learned_guard(&self) -> MutexGuard<'_, HashMap<(String, String), CompletionRoute>> {
        match self.inner.learned.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub(crate) fn learned_route(&self, base: &str, model: &str) -> Option<CompletionRoute> {
        self.learned_guard()
            .get(&(base.to_owned(), model.to_owned()))
            .copied()
    }

    pub(crate) fn learn_route(&self, base: &str, model: &str, route: CompletionRoute) {
        self.learned_guard()
            .insert((base.to_owned(), model.to_owned()), route);
    }

    pub(crate) fn forget_route(&self, base: &str, model: &str) {
        self.learned_guard()
            .remove(&(base.to_owned(), model.to_owned()));
    }

    /// The declared endpoints for `model` on `base`, loading the catalog with
    /// `load` when there is no fresh snapshot and no recent failed fetch.
    ///
    /// `Some(endpoints)` means the catalog lists the model; `None` means the
    /// catalog is unavailable or does not list it.
    pub(crate) async fn declared_endpoints<F, Fut>(
        &self,
        base: &str,
        model: &str,
        load: F,
    ) -> Option<Vec<String>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<HashMap<String, Vec<String>>, String>>,
    {
        let mut snapshots = self.inner.snapshots.lock().await;
        let fresh = match snapshots.get(base) {
            Some(CatalogSnapshot::Loaded { at, .. }) => is_within(*at, self.inner.ttl),
            Some(CatalogSnapshot::Failed { at }) => is_within(*at, self.inner.failure_backoff),
            None => false,
        };

        if !fresh {
            let snapshot = match load().await {
                Ok(endpoints) => CatalogSnapshot::Loaded {
                    at: now(),
                    endpoints,
                },
                Err(reason) => {
                    tracing::debug!(
                        target: "rig::completions",
                        %reason,
                        "Copilot model catalog unavailable; routing by model name"
                    );
                    CatalogSnapshot::Failed { at: now() }
                }
            };
            snapshots.insert(base.to_owned(), snapshot);
        }

        match snapshots.get(base) {
            Some(CatalogSnapshot::Loaded { endpoints, .. }) => endpoints.get(model).cloned(),
            _ => None,
        }
    }
}

impl fmt::Display for CompletionRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn endpoints(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn declared_route_follows_the_model_declaration() {
        assert_eq!(
            declared_route("gpt-6-luna", &endpoints(&["/responses"])),
            Some(CompletionRoute::Responses)
        );
        assert_eq!(
            declared_route("gpt-5.5", &endpoints(&["/responses", "ws:/responses"])),
            Some(CompletionRoute::Responses)
        );
        assert_eq!(
            declared_route(
                "claude-opus-4.7",
                &endpoints(&["/v1/messages", "/chat/completions"])
            ),
            Some(CompletionRoute::ChatCompletions)
        );
        // A codex-named model that only serves chat still goes to chat.
        assert_eq!(
            declared_route("odd-codex", &endpoints(&["/chat/completions"])),
            Some(CompletionRoute::ChatCompletions)
        );
    }

    #[test]
    fn declared_route_keeps_the_historical_choice_when_both_are_declared() {
        let both = endpoints(&["/responses", "/chat/completions", "ws:/responses"]);
        assert_eq!(
            declared_route("gpt-5.4", &both),
            Some(CompletionRoute::ChatCompletions)
        );
        assert_eq!(
            declared_route("gpt-5.1-codex", &both),
            Some(CompletionRoute::Responses)
        );
    }

    #[test]
    fn declared_route_defers_when_no_completion_endpoint_is_declared() {
        assert_eq!(declared_route("m", &endpoints(&["/v1/messages"])), None);
        assert_eq!(declared_route("m", &[]), None);
    }

    fn http_400(body: &str) -> CompletionError {
        CompletionError::from_http_response(http::StatusCode::BAD_REQUEST, body)
    }

    #[test]
    fn detects_unsupported_api_for_model_rejections() {
        let live = r#"{"error":{"message":"model \"gpt-6-luna\" is not accessible via the /chat/completions endpoint","code":"unsupported_api_for_model"}}"#;
        assert!(is_unsupported_api_for_model(&http_400(live)));
        assert!(is_unsupported_api_for_model(&http_400(
            r#"{"code":"unsupported_api_for_model"}"#
        )));
        // A transport layer prefixed the body; the code is still recognized.
        assert!(is_unsupported_api_for_model(&http_400(&format!(
            "capability marker: {live}"
        ))));
        // The preserved-response variant carries the same evidence.
        assert!(is_unsupported_api_for_model(
            &CompletionError::ProviderResponse(
                crate::provider_response::ProviderResponseError::new(
                    http::StatusCode::BAD_REQUEST,
                    live,
                )
            )
        ));
    }

    #[test]
    fn ignores_other_rejections() {
        assert!(!is_unsupported_api_for_model(&http_400(
            r#"{"error":{"message":"bad","code":"invalid_request_error"}}"#
        )));
        assert!(!is_unsupported_api_for_model(
            &CompletionError::from_http_response(
                http::StatusCode::UNAUTHORIZED,
                r#"{"error":{"code":"unsupported_api_for_model"}}"#,
            )
        ));
        assert!(!is_unsupported_api_for_model(
            &CompletionError::ProviderError("unsupported_api_for_model".into())
        ));
    }

    #[tokio::test]
    async fn catalog_is_fetched_once_within_ttl_and_refetched_after() {
        let catalog = CopilotRouteCatalog::with_ttl(Duration::from_secs(600), Duration::ZERO);
        let loads = std::sync::atomic::AtomicUsize::new(0);
        let load = || async {
            loads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(HashMap::from([(
                "gpt-6-luna".to_string(),
                endpoints(&["/responses"]),
            )]))
        };

        assert_eq!(
            catalog.declared_endpoints("base", "gpt-6-luna", load).await,
            Some(endpoints(&["/responses"]))
        );
        assert_eq!(
            catalog.declared_endpoints("base", "other", load).await,
            None
        );
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 1);

        // A different base URL is a different catalog.
        catalog
            .declared_endpoints("other-base", "gpt-6-luna", load)
            .await;
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 2);

        // An expired catalog is fetched again.
        let expiring = CopilotRouteCatalog::with_ttl(Duration::ZERO, Duration::ZERO);
        expiring
            .declared_endpoints("base", "gpt-6-luna", load)
            .await;
        expiring
            .declared_endpoints("base", "gpt-6-luna", load)
            .await;
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn failed_fetch_backs_off_before_retrying() {
        let catalog =
            CopilotRouteCatalog::with_ttl(Duration::from_secs(600), Duration::from_secs(600));
        let loads = std::sync::atomic::AtomicUsize::new(0);
        let failing = || async {
            loads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Err::<HashMap<String, Vec<String>>, _>("503".to_string())
        };

        assert_eq!(catalog.declared_endpoints("base", "m", failing).await, None);
        assert_eq!(catalog.declared_endpoints("base", "m", failing).await, None);
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 1);

        let no_backoff = CopilotRouteCatalog::with_ttl(Duration::from_secs(600), Duration::ZERO);
        no_backoff.declared_endpoints("base", "m", failing).await;
        no_backoff.declared_endpoints("base", "m", failing).await;
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[test]
    fn learned_routes_are_scoped_to_base_and_model() {
        let catalog = CopilotRouteCatalog::new();
        catalog.learn_route("base", "gpt-6-luna", CompletionRoute::Responses);
        assert_eq!(
            catalog.learned_route("base", "gpt-6-luna"),
            Some(CompletionRoute::Responses)
        );
        assert_eq!(catalog.learned_route("other", "gpt-6-luna"), None);
        assert_eq!(catalog.learned_route("base", "gpt-4o"), None);
        // Clones share state.
        let clone = catalog.clone();
        clone.forget_route("base", "gpt-6-luna");
        assert_eq!(catalog.learned_route("base", "gpt-6-luna"), None);
    }
}
