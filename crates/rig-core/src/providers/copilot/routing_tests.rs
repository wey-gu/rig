//! Route-selection regression tests for Copilot completion models.
//!
//! These drive a real [`super::CompletionModel`] over a scripted HTTP double
//! that answers per path, so they observe which endpoint each request hits.
//! The failure they guard against: a Responses-only model (declared
//! `supported_endpoints: ["/responses"]`, e.g. `gpt-6-luna`) was sent to
//! `/chat/completions` because routing guessed from the model name, and every
//! call failed with `400 unsupported_api_for_model`.

use std::future::{self, Future};
use std::sync::{Arc, Mutex, MutexGuard};

use bytes::Bytes;
use futures::StreamExt;

use super::Client;
use crate::client::CompletionClient;
use crate::completion::{CompletionModel as _, ToolDefinition};
use crate::http_client::{
    self, HttpClientExt, LazyBody, MultipartForm, Request, Response, StreamingResponse,
};
use crate::providers::internal::openai_chat_completions_compatible::test_support::{
    sse_bytes_from_data_lines, sse_bytes_from_json_events,
};
use crate::streaming::StreamedAssistantContent;
use crate::wasm_compat::WasmCompatSend;

const UNSUPPORTED_ON_CHAT: &str = r#"{"error":{"message":"model \"gpt-6-luna\" is not accessible via the /chat/completions endpoint","code":"unsupported_api_for_model"}}"#;
const UNSUPPORTED_ON_RESPONSES: &str = r#"{"error":{"message":"model \"chat-only\" is not accessible via the /responses endpoint","code":"unsupported_api_for_model"}}"#;

#[derive(Clone, Debug)]
enum Reply {
    /// A unary JSON body with this status.
    Json(http::StatusCode, &'static str),
    /// A successful `text/event-stream` body.
    Sse(Bytes),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Seen {
    method: http::Method,
    path: String,
    body: String,
}

/// HTTP double that answers by request path suffix and records every request.
///
/// Non-success replies are returned as
/// [`http_client::Error::InvalidStatusCodeWithDetails`], exactly what the
/// bundled `reqwest` client produces for a non-2xx response.
#[derive(Clone, Debug, Default)]
struct PathScriptedHttpClient {
    replies: Arc<Mutex<Vec<(&'static str, Reply)>>>,
    seen: Arc<Mutex<Vec<Seen>>>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

impl PathScriptedHttpClient {
    fn new(replies: Vec<(&'static str, Reply)>) -> Self {
        Self {
            replies: Arc::new(Mutex::new(replies)),
            seen: Arc::default(),
        }
    }

    fn reply_for(&self, path: &str) -> Reply {
        lock(&self.replies)
            .iter()
            .find(|(suffix, _)| path.ends_with(suffix))
            .map(|(_, reply)| reply.clone())
            .unwrap_or(Reply::Json(http::StatusCode::NOT_FOUND, "{}"))
    }

    fn record<T: Into<Bytes>>(&self, req: Request<T>) -> String {
        let (parts, body) = req.into_parts();
        let path = parts.uri.path().to_string();
        lock(&self.seen).push(Seen {
            method: parts.method,
            path: path.clone(),
            body: String::from_utf8_lossy(&body.into()).into_owned(),
        });
        path
    }

    fn seen(&self) -> Vec<Seen> {
        lock(&self.seen).clone()
    }

    /// Paths of completion requests (everything except the catalog fetch).
    fn completion_paths(&self) -> Vec<String> {
        self.seen()
            .into_iter()
            .filter(|seen| seen.method == http::Method::POST)
            .map(|seen| seen.path)
            .collect()
    }

    fn catalog_fetches(&self) -> usize {
        self.seen()
            .iter()
            .filter(|seen| seen.method == http::Method::GET && seen.path.ends_with("/models"))
            .count()
    }

    fn status_error(status: http::StatusCode, body: &str) -> http_client::Error {
        http_client::Error::InvalidStatusCodeWithDetails {
            status,
            body: body.to_string(),
            headers: Box::default(),
        }
    }
}

impl HttpClientExt for PathScriptedHttpClient {
    fn send<T, U>(
        &self,
        req: Request<T>,
    ) -> impl Future<Output = http_client::Result<Response<LazyBody<U>>>> + WasmCompatSend + 'static
    where
        T: Into<Bytes> + WasmCompatSend,
        U: From<Bytes> + WasmCompatSend + 'static,
    {
        let path = self.record(req);
        let reply = self.reply_for(&path);
        async move {
            match reply {
                Reply::Json(status, body) if status.is_success() => {
                    let body: LazyBody<U> =
                        Box::pin(async move { Ok(U::from(Bytes::from_static(body.as_bytes()))) });
                    Response::builder()
                        .status(status)
                        .body(body)
                        .map_err(http_client::Error::Protocol)
                }
                Reply::Json(status, body) => Err(Self::status_error(status, body)),
                Reply::Sse(_) => Err(Self::status_error(
                    http::StatusCode::NOT_IMPLEMENTED,
                    "scripted SSE reply on a unary request",
                )),
            }
        }
    }

    fn send_multipart<U>(
        &self,
        _req: Request<MultipartForm>,
    ) -> impl Future<Output = http_client::Result<Response<LazyBody<U>>>> + WasmCompatSend + 'static
    where
        U: From<Bytes> + WasmCompatSend + 'static,
    {
        future::ready(Err(http_client::Error::InvalidStatusCode(
            http::StatusCode::NOT_IMPLEMENTED,
        )))
    }

    fn send_streaming<T>(
        &self,
        req: Request<T>,
    ) -> impl Future<Output = http_client::Result<StreamingResponse>> + WasmCompatSend
    where
        T: Into<Bytes> + WasmCompatSend,
    {
        let path = self.record(req);
        let reply = self.reply_for(&path);
        async move {
            match reply {
                Reply::Sse(bytes) => {
                    let stream =
                        futures::stream::iter(vec![Ok::<Bytes, http_client::Error>(bytes)]);
                    let boxed: http_client::sse::BoxedStream = Box::pin(stream);
                    Response::builder()
                        .status(http::StatusCode::OK)
                        .header(http::header::CONTENT_TYPE, "text/event-stream")
                        .body(boxed)
                        .map_err(http_client::Error::Protocol)
                }
                Reply::Json(status, body) => Err(Self::status_error(status, body)),
            }
        }
    }
}

const CATALOG: &str = r#"{"data":[
    {"id":"gpt-6-luna","capabilities":{"type":"chat"},"supported_endpoints":["/responses","ws:/responses"]},
    {"id":"chat-only","capabilities":{"type":"chat"},"supported_endpoints":["/v1/messages","/chat/completions"]},
    {"id":"gpt-5.4","capabilities":{"type":"chat"},"supported_endpoints":["/responses","/chat/completions"]},
    {"id":"gpt-4o","capabilities":{"type":"chat"}}
]}"#;

const CHAT_BODY: &str = r#"{
    "id": "chatcmpl-1",
    "model": "chat-only",
    "choices": [{"index": 0, "message": {"role": "assistant", "content": "from chat"}, "finish_reason": "stop"}],
    "usage": {"prompt_tokens": 4, "total_tokens": 7}
}"#;

const RESPONSES_BODY: &str = r#"{
    "id": "resp_1",
    "object": "response",
    "created_at": 1700000000,
    "status": "completed",
    "error": null,
    "incomplete_details": null,
    "instructions": null,
    "max_output_tokens": null,
    "model": "gpt-6-luna",
    "usage": {
        "input_tokens": 4,
        "input_tokens_details": {"cached_tokens": 0},
        "output_tokens": 3,
        "output_tokens_details": {"reasoning_tokens": 0},
        "total_tokens": 7
    },
    "output": [{
        "type": "message",
        "id": "msg_1",
        "role": "assistant",
        "status": "completed",
        "content": [{"type": "output_text", "text": "from responses", "annotations": []}]
    }],
    "tools": []
}"#;

fn responses_sse() -> Bytes {
    let delta = serde_json::json!({
        "type": "response.output_text.delta",
        "sequence_number": 1,
        "item_id": "msg_1",
        "output_index": 0,
        "content_index": 0,
        "delta": "streamed from responses"
    });
    let completed = serde_json::json!({
        "type": "response.completed",
        "sequence_number": 2,
        "response": serde_json::from_str::<serde_json::Value>(RESPONSES_BODY).expect("body")
    });
    sse_bytes_from_json_events(&[delta, completed])
}

fn chat_sse() -> Bytes {
    sse_bytes_from_data_lines([
        r#"{"id":"chatcmpl-1","model":"chat-only","choices":[{"index":0,"delta":{"content":"streamed from chat"},"finish_reason":null}],"usage":null}"#,
        r#"{"id":"chatcmpl-1","model":"chat-only","choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"usage":{"prompt_tokens":4,"completion_tokens":3,"total_tokens":7}}"#,
        "[DONE]",
    ])
}

fn client(http: &PathScriptedHttpClient) -> Client<PathScriptedHttpClient> {
    Client::builder()
        .api_key("copilot-token")
        .http_client(http.clone())
        .build()
        .expect("build client")
}

async fn complete_text(
    model: &super::CompletionModel<PathScriptedHttpClient>,
) -> Result<String, crate::completion::CompletionError> {
    let response = model
        .completion(model.completion_request("hello").build())
        .await?;
    Ok(response
        .choice
        .iter()
        .filter_map(|content| match content {
            crate::completion::AssistantContent::Text(text) => Some(text.text.clone()),
            _ => None,
        })
        .collect())
}

async fn stream_text(
    model: &super::CompletionModel<PathScriptedHttpClient>,
) -> Result<String, crate::completion::CompletionError> {
    let mut stream = model
        .stream(model.completion_request("hello").build())
        .await?;
    let mut text = String::new();
    while let Some(item) = stream.next().await {
        if let StreamedAssistantContent::Text(delta) = item? {
            text.push_str(&delta.text);
        }
    }
    Ok(text)
}

#[tokio::test]
async fn responses_only_model_routes_by_declared_endpoints() {
    let http = PathScriptedHttpClient::new(vec![
        ("/models", Reply::Json(http::StatusCode::OK, CATALOG)),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");

    let text = complete_text(&model).await.expect("completion succeeds");

    assert_eq!(text, "from responses");
    // The catalog decided the route up front: no rejected chat attempt.
    assert_eq!(http.completion_paths(), vec!["/responses"]);
}

#[tokio::test]
async fn declared_chat_model_without_codex_in_name_stays_on_chat() {
    let http = PathScriptedHttpClient::new(vec![
        ("/models", Reply::Json(http::StatusCode::OK, CATALOG)),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::OK, CHAT_BODY),
        ),
    ]);
    let client = client(&http);

    complete_text(&client.completion_model("chat-only"))
        .await
        .expect("chat-only model");
    // Declares both endpoints: keeps its historical chat route.
    complete_text(&client.completion_model("gpt-5.4"))
        .await
        .expect("dual-endpoint model");
    // Not declared at all: the name rule decides.
    complete_text(&client.completion_model("gpt-4o"))
        .await
        .expect("undeclared model");

    assert_eq!(
        http.completion_paths(),
        vec![
            "/chat/completions",
            "/chat/completions",
            "/chat/completions"
        ]
    );
    assert_eq!(
        http.catalog_fetches(),
        1,
        "the catalog is cached per client"
    );
}

#[tokio::test]
async fn chat_rejection_retries_once_on_responses_and_remembers_the_route() {
    // Catalog unavailable: routing falls back to the name rule (chat), and
    // the self-healing guard must recover.
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");

    assert_eq!(
        complete_text(&model).await.expect("retried on responses"),
        "from responses"
    );
    assert_eq!(
        http.completion_paths(),
        vec!["/chat/completions", "/responses"]
    );

    // The learned route is used directly next time.
    complete_text(&model).await.expect("learned route");
    assert_eq!(
        http.completion_paths(),
        vec!["/chat/completions", "/responses", "/responses"]
    );
}

#[tokio::test]
async fn responses_rejection_retries_once_on_chat() {
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::OK, CHAT_BODY),
        ),
        (
            "/responses",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_RESPONSES),
        ),
    ]);
    // The name rule sends codex-named models to /responses first.
    let model = client(&http).completion_model("legacy-codex-chat");

    assert_eq!(
        complete_text(&model).await.expect("retried on chat"),
        "from chat"
    );
    assert_eq!(
        http.completion_paths(),
        vec!["/responses", "/chat/completions"]
    );
}

#[tokio::test]
async fn rejection_on_both_routes_surfaces_the_provider_error_after_one_retry() {
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
        (
            "/responses",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_RESPONSES),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");

    let error = complete_text(&model).await.expect_err("no route works");

    assert_eq!(
        error.provider_response_status(),
        Some(http::StatusCode::BAD_REQUEST)
    );
    let body = error.provider_response_json().expect("json").expect("body");
    assert_eq!(body["error"]["code"], "unsupported_api_for_model");
    assert_eq!(
        http.completion_paths(),
        vec!["/chat/completions", "/responses"]
    );
}

#[tokio::test]
async fn other_rejections_are_not_retried() {
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        (
            "/chat/completions",
            Reply::Json(
                http::StatusCode::BAD_REQUEST,
                r#"{"error":{"message":"bad input","code":"invalid_request_body"}}"#,
            ),
        ),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");

    complete_text(&model).await.expect_err("not a route error");
    assert_eq!(http.completion_paths(), vec!["/chat/completions"]);
}

#[tokio::test]
async fn streaming_routes_by_declared_endpoints() {
    let http = PathScriptedHttpClient::new(vec![
        ("/models", Reply::Json(http::StatusCode::OK, CATALOG)),
        ("/responses", Reply::Sse(responses_sse())),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");

    let text = stream_text(&model).await.expect("stream succeeds");

    assert_eq!(text, "streamed from responses");
    assert_eq!(http.completion_paths(), vec!["/responses"]);
}

#[tokio::test]
async fn streaming_chat_rejection_falls_back_to_responses_and_remembers() {
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        ("/responses", Reply::Sse(responses_sse())),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");

    let text = stream_text(&model).await.expect("stream recovers");
    assert_eq!(text, "streamed from responses");
    assert_eq!(
        http.completion_paths(),
        vec!["/chat/completions", "/responses"]
    );

    stream_text(&model).await.expect("learned route");
    assert_eq!(
        http.completion_paths(),
        vec!["/chat/completions", "/responses", "/responses"]
    );
}

#[tokio::test]
async fn streaming_responses_rejection_falls_back_to_chat() {
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        ("/chat/completions", Reply::Sse(chat_sse())),
        (
            "/responses",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_RESPONSES),
        ),
    ]);
    let model = client(&http).completion_model("legacy-codex-chat");

    let text = stream_text(&model).await.expect("stream recovers");
    assert_eq!(text, "streamed from chat");
    assert_eq!(
        http.completion_paths(),
        vec!["/responses", "/chat/completions"]
    );
}

#[tokio::test]
async fn fallback_request_keeps_tools_and_structured_output_on_the_responses_wire() {
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
    ]);
    let model = client(&http).completion_model("gpt-6-luna");
    let schema: schemars::Schema = serde_json::from_value(serde_json::json!({
        "type": "object",
        "properties": {"answer": {"type": "string"}},
        "required": ["answer"],
        "additionalProperties": false
    }))
    .expect("schema");
    let request = model
        .completion_request("hello")
        .tool(ToolDefinition {
            name: "lookup".into(),
            description: "Look something up".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {"q": {"type": "string"}},
                "required": ["q"]
            }),
        })
        .output_schema(schema)
        .additional_params(serde_json::json!({"reasoning": {"effort": "low"}}))
        .build();

    model
        .completion(request)
        .await
        .expect("retried on responses");

    let retried = http
        .seen()
        .into_iter()
        .find(|seen| seen.path == "/responses")
        .expect("responses request");
    let body: serde_json::Value = serde_json::from_str(&retried.body).expect("json body");
    // The retry is built by the Responses request builder, not replayed chat JSON.
    assert!(body.get("input").is_some(), "responses wire uses `input`");
    assert!(body.get("messages").is_none());
    assert_eq!(body["tools"][0]["name"], "lookup");
    assert_eq!(body["tools"][0]["type"], "function");
    assert_eq!(body["text"]["format"]["type"], "json_schema");
    assert_eq!(body["reasoning"]["effort"], "low");
}

fn shared_client(
    http: &PathScriptedHttpClient,
    catalog: &super::CopilotRouteCatalog,
) -> Client<PathScriptedHttpClient> {
    Client::builder()
        .api_key("copilot-token")
        .http_client(http.clone())
        .route_catalog(catalog.clone())
        .build()
        .expect("build client")
}

#[tokio::test]
async fn clients_sharing_a_route_catalog_fetch_it_once() {
    let catalog = super::CopilotRouteCatalog::new();
    let http = PathScriptedHttpClient::new(vec![
        ("/models", Reply::Json(http::StatusCode::OK, CATALOG)),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::OK, CHAT_BODY),
        ),
    ]);

    for _ in 0..2 {
        complete_text(&shared_client(&http, &catalog).completion_model("gpt-6-luna"))
            .await
            .expect("completion");
    }

    assert_eq!(http.catalog_fetches(), 1, "one fetch serves every client");
    assert_eq!(http.completion_paths(), vec!["/responses", "/responses"]);
}

#[tokio::test]
async fn a_learned_route_outlives_the_client_that_learned_it() {
    let catalog = super::CopilotRouteCatalog::new();
    let http = PathScriptedHttpClient::new(vec![
        (
            "/models",
            Reply::Json(http::StatusCode::SERVICE_UNAVAILABLE, "{}"),
        ),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
        (
            "/chat/completions",
            Reply::Json(http::StatusCode::BAD_REQUEST, UNSUPPORTED_ON_CHAT),
        ),
    ]);

    for _ in 0..2 {
        complete_text(&shared_client(&http, &catalog).completion_model("gpt-6-luna"))
            .await
            .expect("completion");
    }

    assert_eq!(
        http.completion_paths(),
        vec!["/chat/completions", "/responses", "/responses"]
    );
}

#[tokio::test]
async fn completion_endpoint_reports_the_route_a_request_would_take() {
    let http = PathScriptedHttpClient::new(vec![(
        "/models",
        Reply::Json(http::StatusCode::OK, CATALOG),
    )]);
    let client = client(&http);

    assert_eq!(
        client
            .completion_model("gpt-6-luna")
            .completion_endpoint()
            .await,
        "/responses"
    );
    assert_eq!(
        client
            .completion_model("chat-only")
            .completion_endpoint()
            .await,
        "/chat/completions"
    );
    // Not in the catalog: the name rule.
    assert_eq!(
        client
            .completion_model("unlisted-codex")
            .completion_endpoint()
            .await,
        "/responses"
    );
    assert!(http.completion_paths().is_empty(), "no completion is sent");
}

#[tokio::test]
async fn an_expired_catalog_is_fetched_again() {
    let catalog =
        super::CopilotRouteCatalog::with_ttl(std::time::Duration::ZERO, std::time::Duration::ZERO);
    let http = PathScriptedHttpClient::new(vec![
        ("/models", Reply::Json(http::StatusCode::OK, CATALOG)),
        (
            "/responses",
            Reply::Json(http::StatusCode::OK, RESPONSES_BODY),
        ),
    ]);
    let model = shared_client(&http, &catalog).completion_model("gpt-6-luna");

    complete_text(&model).await.expect("first");
    complete_text(&model).await.expect("second");

    assert_eq!(http.catalog_fetches(), 2);
}
