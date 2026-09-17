//! Typed classification for ChatGPT provider response envelopes.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ErrorEnvelope {
    code: Option<String>,
    error: Option<ErrorBody>,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    code: Option<String>,
}

pub(super) fn is_expired_token_response(status: http::StatusCode, body: &str) -> bool {
    if status != http::StatusCode::UNAUTHORIZED {
        return false;
    }
    let Ok(payload) = serde_json::from_str::<ErrorEnvelope>(body) else {
        return false;
    };
    payload.code.as_deref() == Some("token_expired")
        || payload.error.and_then(|error| error.code).as_deref() == Some("token_expired")
}
