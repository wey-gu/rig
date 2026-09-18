//! Native ChatGPT OAuth and token cache implementation.

use super::{AuthContext, AuthError, DeviceCodeHandler, DeviceCodePrompt};
use crate::providers::internal::device_auth::{
    emit_device_code_prompt, read_json_record, token_expired, write_json_record,
};
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

const CHATGPT_AUTH_BASE: &str = "https://auth.openai.com";
const CHATGPT_DEVICE_CODE_URL: &str = "https://auth.openai.com/api/accounts/deviceauth/usercode";
const CHATGPT_DEVICE_TOKEN_URL: &str = "https://auth.openai.com/api/accounts/deviceauth/token";
const CHATGPT_OAUTH_TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const CHATGPT_DEVICE_VERIFY_URL: &str = "https://auth.openai.com/codex/device";
const CHATGPT_CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const CHATGPT_BROWSER_CALLBACK_HOST: &str = "127.0.0.1";
const CHATGPT_BROWSER_CALLBACK_PORTS: [u16; 2] = [1455, 1457];
const CHATGPT_BROWSER_CALLBACK_PATH: &str = "/auth/callback";
const CHATGPT_BROWSER_SCOPE: &str =
    "openid profile email offline_access api.connectors.read api.connectors.invoke";
const CHATGPT_BROWSER_ORIGINATOR: &str = "codex_cli_rs";
const CHATGPT_BROWSER_CALLBACK_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const CHATGPT_BROWSER_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
const CHATGPT_BROWSER_MAX_HEADER_BYTES: usize = 8192;
const TOKEN_EXPIRY_SKEW_SECONDS: i64 = 60;
const DEVICE_CODE_TIMEOUT_SECONDS: i64 = 15 * 60;
const DEVICE_CODE_POLL_SLEEP_SECONDS: u64 = 5;

const DEFAULT_OAUTH_REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const DEFAULT_OAUTH_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Debug, Clone)]
pub(super) struct PlatformAuthenticator {
    auth_file: Option<PathBuf>,
    http_client: Result<reqwest::Client, String>,
    device_code_handler: DeviceCodeHandler,
    allow_device_flow: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub(super) struct AuthRecord {
    access_token: Option<String>,
    refresh_token: Option<String>,
    id_token: Option<String>,
    expires_at: Option<i64>,
    account_id: Option<String>,
    #[serde(default)]
    reauth_required: bool,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_auth_id: String,
    #[serde(alias = "usercode")]
    user_code: String,
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    interval: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct DeviceTokenResponse {
    authorization_code: String,
    code_verifier: String,
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    id_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthErrorResponse {
    error: Option<OAuthError>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OAuthError {
    Code(String),
    Details {
        code: Option<String>,
        message: Option<String>,
    },
}

impl OAuthErrorResponse {
    fn code(&self) -> Option<&str> {
        match self.error.as_ref()? {
            OAuthError::Code(code) => Some(code),
            OAuthError::Details { code, .. } => code.as_deref(),
        }
    }

    fn description(&self) -> Option<&str> {
        self.error_description
            .as_deref()
            .or_else(|| match self.error.as_ref()? {
                OAuthError::Details { message, .. } => message.as_deref(),
                OAuthError::Code(_) => None,
            })
    }
}

enum RefreshTokensError {
    Reauthenticate,
    Auth(AuthError),
}

impl PlatformAuthenticator {
    pub(super) fn new(
        auth_file: Option<PathBuf>,
        oauth_http_client: Option<reqwest::Client>,
        device_code_handler: DeviceCodeHandler,
        allow_device_flow: bool,
    ) -> Self {
        Self {
            auth_file,
            http_client: oauth_http_client
                .map(Ok)
                .unwrap_or_else(default_oauth_http_client),
            device_code_handler,
            allow_device_flow,
        }
    }

    pub(super) fn device_flow_allowed(&self) -> bool {
        self.allow_device_flow
    }

    pub(super) async fn cached_or_refreshed_context(
        &self,
    ) -> Result<Option<AuthContext>, AuthError> {
        let mut record: AuthRecord = read_json_record(self.auth_file.as_deref())?;

        if record.reauth_required {
            return Ok(None);
        }

        if let Some(access_token) = record.access_token.clone()
            && !token_expired(record.expires_at, TOKEN_EXPIRY_SKEW_SECONDS)
        {
            let account_id = record
                .account_id
                .clone()
                .or_else(|| extract_account_id(record.id_token.as_deref()))
                .or_else(|| extract_account_id(Some(&access_token)));
            if account_id != record.account_id {
                record.account_id = account_id.clone();
                write_json_record(self.auth_file.as_deref(), &record)?;
            }
            return Ok(Some(AuthContext {
                access_token,
                account_id,
            }));
        }

        if let Some(refresh_token) = record.refresh_token.clone() {
            match self.refresh_tokens(&refresh_token).await {
                Ok(refreshed) => {
                    write_json_record(self.auth_file.as_deref(), &refreshed)?;
                    return Ok(Some(AuthContext {
                        access_token: refreshed.access_token.unwrap_or_default(),
                        account_id: refreshed.account_id,
                    }));
                }
                Err(RefreshTokensError::Reauthenticate) => {
                    if let Some(context) = self.mark_reauth_required(
                        record.access_token.as_deref(),
                        Some(&refresh_token),
                    )? {
                        return Ok(Some(context));
                    }
                }
                Err(RefreshTokensError::Auth(err)) => return Err(err),
            }
        }

        Ok(None)
    }

    pub(super) async fn login_device_flow(&self) -> Result<AuthRecord, AuthError> {
        self.login_with_fallback_at(
            CHATGPT_DEVICE_CODE_URL,
            CHATGPT_DEVICE_TOKEN_URL,
            CHATGPT_OAUTH_TOKEN_URL,
            CHATGPT_AUTH_BASE,
            CHATGPT_OAUTH_TOKEN_URL,
            &CHATGPT_BROWSER_CALLBACK_PORTS,
        )
        .await
    }

    async fn login_with_fallback_at(
        &self,
        device_code_url: &str,
        device_token_url: &str,
        device_oauth_token_url: &str,
        browser_issuer: &str,
        browser_oauth_token_url: &str,
        browser_callback_ports: &[u16],
    ) -> Result<AuthRecord, AuthError> {
        match self
            .login_device_flow_at(device_code_url, device_token_url, device_oauth_token_url)
            .await
        {
            Ok(record) => Ok(record),
            Err(device_error) if browser_fallback_candidate(&device_error) => self
                .login_browser_flow_at(
                    browser_issuer,
                    browser_oauth_token_url,
                    browser_callback_ports,
                )
                .await
                .map_err(|browser_error| {
                    AuthError::Message(format!(
                        "{device_error} Browser sign-in fallback also failed: {browser_error}"
                    ))
                }),
            Err(error) => Err(error),
        }
    }

    pub(super) fn persist_device_flow(&self, fresh: AuthRecord) -> Result<AuthContext, AuthError> {
        write_json_record(self.auth_file.as_deref(), &fresh)?;
        Ok(AuthContext {
            access_token: fresh.access_token.unwrap_or_default(),
            account_id: fresh.account_id,
        })
    }

    pub(super) async fn refresh_after_rejection(
        &self,
        rejected_access_token: &str,
    ) -> Result<AuthContext, AuthError> {
        self.refresh_after_rejection_at(CHATGPT_OAUTH_TOKEN_URL, rejected_access_token)
            .await
    }

    async fn refresh_after_rejection_at(
        &self,
        oauth_token_url: &str,
        rejected_access_token: &str,
    ) -> Result<AuthContext, AuthError> {
        let record: AuthRecord = read_json_record(self.auth_file.as_deref())?;

        // Another client may have refreshed while this request was in flight.
        // Reuse that persisted credential instead of rotating the refresh token
        // a second time.
        if record.access_token.as_deref() != Some(rejected_access_token)
            && let Some(access_token) = record.access_token.clone()
            && !token_expired(record.expires_at, TOKEN_EXPIRY_SKEW_SECONDS)
        {
            return Ok(AuthContext {
                account_id: record
                    .account_id
                    .or_else(|| extract_account_id(record.id_token.as_deref())),
                access_token,
            });
        }

        let Some(refresh_token) = record.refresh_token else {
            if let Some(context) = self.mark_reauth_required(Some(rejected_access_token), None)? {
                return Ok(context);
            }
            return Err(sign_in_required());
        };
        match self
            .refresh_tokens_at(oauth_token_url, &refresh_token)
            .await
        {
            Ok(refreshed) => {
                write_json_record(self.auth_file.as_deref(), &refreshed)?;
                Ok(AuthContext {
                    access_token: refreshed.access_token.unwrap_or_default(),
                    account_id: refreshed.account_id,
                })
            }
            Err(RefreshTokensError::Reauthenticate) => {
                if let Some(context) =
                    self.mark_reauth_required(Some(rejected_access_token), Some(&refresh_token))?
                {
                    return Ok(context);
                }
                Err(sign_in_required())
            }
            Err(RefreshTokensError::Auth(error)) => Err(error),
        }
    }

    fn mark_reauth_required(
        &self,
        expected_access_token: Option<&str>,
        expected_refresh_token: Option<&str>,
    ) -> Result<Option<AuthContext>, AuthError> {
        let current: AuthRecord = read_json_record(self.auth_file.as_deref())?;
        if current.access_token.as_deref() != expected_access_token
            || current.refresh_token.as_deref() != expected_refresh_token
        {
            if let Some(access_token) = current.access_token.clone()
                && !current.reauth_required
                && !token_expired(current.expires_at, TOKEN_EXPIRY_SKEW_SECONDS)
            {
                return Ok(Some(AuthContext {
                    account_id: current
                        .account_id
                        .or_else(|| extract_account_id(current.id_token.as_deref())),
                    access_token,
                }));
            }
            // A different writer changed the credential. Never erase its state
            // based on this request's stale invalid_grant result; the next call
            // can evaluate the complete record it left behind.
            return Ok(None);
        }
        write_json_record(
            self.auth_file.as_deref(),
            &AuthRecord {
                reauth_required: true,
                ..AuthRecord::default()
            },
        )?;
        Ok(None)
    }

    async fn login_device_flow_at(
        &self,
        device_code_url: &str,
        device_token_url: &str,
        oauth_token_url: &str,
    ) -> Result<AuthRecord, AuthError> {
        let http_client = self
            .http_client
            .as_ref()
            .map_err(|error| AuthError::Message(error.clone()))?;
        let device = http_client
            .post(device_code_url)
            .json(&serde_json::json!({ "client_id": CHATGPT_CLIENT_ID }))
            .send()
            .await
            .map_err(|error| auth_request_error("device-code request", error))?;
        if !device.status().is_success() {
            return Err(device_code_request_error(device.status(), device.headers()));
        }
        let device = device.json::<DeviceCodeResponse>().await?;

        emit_device_code_prompt(
            self.device_code_handler.0.as_ref(),
            DeviceCodePrompt {
                verification_uri: CHATGPT_DEVICE_VERIFY_URL.to_string(),
                user_code: device.user_code.clone(),
            },
            &format!(
                "Sign in with ChatGPT:\n1) Visit {CHATGPT_DEVICE_VERIFY_URL}\n2) Enter code: {}\nDo not share this device code.",
                device.user_code
            ),
        );

        let interval = device.interval.unwrap_or(DEVICE_CODE_POLL_SLEEP_SECONDS);
        let start = std::time::Instant::now();
        let code = loop {
            if start.elapsed().as_secs() as i64 >= DEVICE_CODE_TIMEOUT_SECONDS {
                return Err(AuthError::Message(
                    "Timed out waiting for ChatGPT device authorization".into(),
                ));
            }

            let response = http_client
                .post(device_token_url)
                .json(&serde_json::json!({
                    "device_auth_id": device.device_auth_id,
                    "user_code": device.user_code,
                }))
                .send()
                .await
                .map_err(|error| auth_request_error("device-token polling", error))?;

            if response.status().is_success() {
                let token_response = response.json::<DeviceTokenResponse>().await?;
                break token_response;
            }

            let status = response.status();
            if status.as_u16() == 403 || status.as_u16() == 404 {
                tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
                continue;
            }

            let text = response.text().await.unwrap_or_default();
            return Err(AuthError::Message(format!(
                "ChatGPT device authorization failed: {status} {text}"
            )));
        };

        let redirect_uri = format!("{CHATGPT_AUTH_BASE}/deviceauth/callback");
        let form = [
            ("grant_type", "authorization_code"),
            ("code", code.authorization_code.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("client_id", CHATGPT_CLIENT_ID),
            ("code_verifier", code.code_verifier.as_str()),
        ];
        let body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(form)
            .finish();

        let tokens = http_client
            .post(oauth_token_url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(body)
            .send()
            .await
            .map_err(|error| auth_request_error("token exchange", error))?
            .error_for_status()?
            .json::<OAuthTokenResponse>()
            .await?;

        Ok(build_auth_record(tokens, None))
    }

    async fn login_browser_flow_at(
        &self,
        issuer: &str,
        oauth_token_url: &str,
        callback_ports: &[u16],
    ) -> Result<AuthRecord, AuthError> {
        let http_client = self
            .http_client
            .as_ref()
            .map_err(|error| AuthError::Message(error.clone()))?;
        let verifier = random_urlsafe(64)?;
        let challenge = pkce_challenge(&verifier);
        let state = random_urlsafe(32)?;
        let listener = bind_browser_callback_listener(callback_ports).await?;
        let port = listener.local_addr()?.port();
        let redirect_uri = format!("http://localhost:{port}{CHATGPT_BROWSER_CALLBACK_PATH}");
        let authorize_url = build_browser_authorize_url(issuer, &redirect_uri, &challenge, &state);
        emit_device_code_prompt(
            self.device_code_handler.0.as_ref(),
            DeviceCodePrompt {
                verification_uri: authorize_url.clone(),
                user_code: String::new(),
            },
            &format!("Open {authorize_url} to sign in with ChatGPT."),
        );

        let code = wait_for_browser_callback(listener, &state).await?;
        let body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs([
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                ("redirect_uri", redirect_uri.as_str()),
                ("client_id", CHATGPT_CLIENT_ID),
                ("code_verifier", verifier.as_str()),
            ])
            .finish();
        let response = http_client
            .post(oauth_token_url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(body)
            .send()
            .await
            .map_err(|error| auth_request_error("browser token exchange", error))?;
        let status = response.status();
        if !status.is_success() {
            let payload = response.json::<OAuthErrorResponse>().await.ok();
            return Err(AuthError::Message(format_browser_token_error(
                status,
                payload.as_ref(),
            )));
        }
        let tokens = response.json::<OAuthTokenResponse>().await?;
        if tokens
            .refresh_token
            .as_deref()
            .is_none_or(|token| token.trim().is_empty())
        {
            return Err(AuthError::Message(
                "ChatGPT browser token exchange did not return a refresh token".into(),
            ));
        }
        Ok(build_auth_record(tokens, None))
    }

    async fn refresh_tokens(&self, refresh_token: &str) -> Result<AuthRecord, RefreshTokensError> {
        self.refresh_tokens_at(CHATGPT_OAUTH_TOKEN_URL, refresh_token)
            .await
    }

    async fn refresh_tokens_at(
        &self,
        oauth_token_url: &str,
        refresh_token: &str,
    ) -> Result<AuthRecord, RefreshTokensError> {
        let http_client = self
            .http_client
            .as_ref()
            .map_err(|error| RefreshTokensError::Auth(AuthError::Message(error.clone())))?;
        let form = [
            ("client_id", CHATGPT_CLIENT_ID),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("scope", "openid profile email"),
        ];

        let body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(form)
            .finish();

        let response = http_client
            .post(oauth_token_url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(body)
            .send()
            .await
            .map_err(|error| auth_request_error("token refresh", error))
            .map_err(RefreshTokensError::Auth)?;

        let status = response.status();
        if status.is_success() {
            let tokens = response
                .json::<OAuthTokenResponse>()
                .await
                .map_err(AuthError::from)
                .map_err(RefreshTokensError::Auth)?;
            return Ok(build_auth_record(tokens, Some(refresh_token.to_owned())));
        }

        let body = response.text().await.unwrap_or_default();
        let oauth_error = serde_json::from_str::<OAuthErrorResponse>(&body).ok();
        if should_reauthenticate_after_refresh(
            status,
            oauth_error.as_ref().and_then(OAuthErrorResponse::code),
        ) {
            return Err(RefreshTokensError::Reauthenticate);
        }

        Err(RefreshTokensError::Auth(AuthError::Message(
            format_refresh_error(status, oauth_error.as_ref(), &body),
        )))
    }
}

fn browser_fallback_candidate(error: &AuthError) -> bool {
    let message = error.to_string();
    message.contains("device-code sign-in was blocked by a network security challenge")
        || message.contains("denied the ChatGPT device-code request before sign-in began")
}

async fn bind_browser_callback_listener(
    callback_ports: &[u16],
) -> Result<tokio::net::TcpListener, AuthError> {
    let mut failures = Vec::new();
    for port in callback_ports {
        match tokio::net::TcpListener::bind((CHATGPT_BROWSER_CALLBACK_HOST, *port)).await {
            Ok(listener) => return Ok(listener),
            Err(error) => failures.push(format!("{port}: {error}")),
        }
    }
    Err(AuthError::Message(format!(
        "ChatGPT browser callback could not bind a registered loopback port ({})",
        failures.join("; ")
    )))
}

async fn wait_for_browser_callback(
    listener: tokio::net::TcpListener,
    expected_state: &str,
) -> Result<String, AuthError> {
    tokio::time::timeout(CHATGPT_BROWSER_CALLBACK_TIMEOUT, async move {
        let mut connections = tokio::task::JoinSet::new();
        loop {
            let accepted = if connections.is_empty() {
                Some(listener.accept().await)
            } else {
                match futures::future::select(
                    Box::pin(listener.accept()),
                    Box::pin(connections.join_next()),
                )
                .await
                {
                    futures::future::Either::Left((accepted, _)) => Some(accepted),
                    futures::future::Either::Right((completed, _)) => {
                        match completed {
                            Some(Ok(Some(Ok(code)))) => return Ok(code),
                            Some(Ok(Some(Err(error)))) => return Err(error),
                            Some(Ok(None) | Err(_)) | None => {}
                        }
                        None
                    }
                }
            };

            let Some(accepted) = accepted else {
                continue;
            };
            let (mut stream, peer) = accepted?;
            if !peer.ip().is_loopback() {
                write_browser_callback_response(&mut stream, 403, "Forbidden").await;
                continue;
            }
            let expected_state = expected_state.to_string();
            connections.spawn(async move {
                handle_browser_callback_connection(&mut stream, &expected_state).await
            });
        }
    })
    .await
    .map_err(|_| AuthError::Message("Timed out waiting for ChatGPT browser sign-in".into()))?
}

async fn handle_browser_callback_connection(
    stream: &mut tokio::net::TcpStream,
    expected_state: &str,
) -> Option<Result<String, AuthError>> {
    match tokio::time::timeout(
        CHATGPT_BROWSER_CONNECTION_TIMEOUT,
        handle_browser_callback_connection_inner(stream, expected_state),
    )
    .await
    {
        Ok(outcome) => outcome,
        Err(_) => {
            let _ = tokio::time::timeout(
                Duration::from_secs(1),
                write_browser_callback_response(stream, 408, "Callback request timed out"),
            )
            .await;
            None
        }
    }
}

async fn handle_browser_callback_connection_inner(
    stream: &mut tokio::net::TcpStream,
    expected_state: &str,
) -> Option<Result<String, AuthError>> {
    use tokio::io::AsyncReadExt as _;

    let mut request = Vec::with_capacity(1024);
    loop {
        if request.len() >= CHATGPT_BROWSER_MAX_HEADER_BYTES {
            write_browser_callback_response(stream, 431, "Callback request headers are too large")
                .await;
            return None;
        }
        let remaining = CHATGPT_BROWSER_MAX_HEADER_BYTES - request.len();
        let mut chunk = [0_u8; 1024];
        let read_limit = remaining.min(chunk.len());
        let read_buffer = chunk.get_mut(..read_limit)?;
        let count = match stream.read(read_buffer).await {
            Ok(count) => count,
            Err(_) => return None,
        };
        if count == 0 {
            write_browser_callback_response(stream, 400, "Incomplete callback request").await;
            return None;
        }
        let bytes = chunk.get(..count)?;
        request.extend_from_slice(bytes);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    let Some(first_line_end) = request.windows(2).position(|window| window == b"\r\n") else {
        write_browser_callback_response(stream, 400, "Invalid callback").await;
        return None;
    };
    let first_line_bytes = request.get(..first_line_end)?;
    let Ok(first_line) = std::str::from_utf8(first_line_bytes) else {
        write_browser_callback_response(stream, 400, "Invalid callback").await;
        return None;
    };
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or_default();
    if method != "GET" {
        write_browser_callback_response(stream, 405, "Method not allowed").await;
        return None;
    }
    let Ok(url) = url::Url::parse(&format!("http://{CHATGPT_BROWSER_CALLBACK_HOST}{target}"))
    else {
        write_browser_callback_response(stream, 400, "Invalid callback").await;
        return None;
    };
    if url.path() != CHATGPT_BROWSER_CALLBACK_PATH {
        write_browser_callback_response(stream, 404, "Not found").await;
        return None;
    }
    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();
    if params.get("state").map(String::as_str) != Some(expected_state) {
        write_browser_callback_response(stream, 400, "Sign-in state mismatch").await;
        return None;
    }
    if let Some(error) = params.get("error") {
        write_browser_callback_response(
            stream,
            400,
            "ChatGPT sign-in was not completed. You can close this tab.",
        )
        .await;
        let description = params
            .get("error_description")
            .map(String::as_str)
            .unwrap_or(error);
        return Some(Err(AuthError::Message(format!(
            "ChatGPT browser authorization failed: {description}"
        ))));
    }
    let Some(code) = params
        .get("code")
        .filter(|code| !code.trim().is_empty())
        .cloned()
    else {
        write_browser_callback_response(stream, 400, "Missing authorization code").await;
        return None;
    };
    write_browser_callback_response(
        stream,
        200,
        "ChatGPT sign-in received. You can close this tab and return to Nowledge Mem.",
    )
    .await;
    Some(Ok(code))
}

async fn write_browser_callback_response(
    stream: &mut tokio::net::TcpStream,
    status: u16,
    message: &str,
) {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        431 => "Request Header Fields Too Large",
        _ => "OK",
    };
    let body = format!("<html><body><h1>{message}</h1></body></html>");
    let response = format!(
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = tokio::io::AsyncWriteExt::write_all(stream, response.as_bytes()).await;
    let _ = tokio::io::AsyncWriteExt::shutdown(stream).await;
}

fn build_browser_authorize_url(
    issuer: &str,
    redirect_uri: &str,
    challenge: &str,
    state: &str,
) -> String {
    let mut query = url::form_urlencoded::Serializer::new(String::new());
    query
        .append_pair("response_type", "code")
        .append_pair("client_id", CHATGPT_CLIENT_ID)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", CHATGPT_BROWSER_SCOPE)
        .append_pair("code_challenge", challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("id_token_add_organizations", "true")
        .append_pair("codex_cli_simplified_flow", "true")
        .append_pair("state", state)
        .append_pair("originator", CHATGPT_BROWSER_ORIGINATOR);
    format!(
        "{}/oauth/authorize?{}",
        issuer.trim_end_matches('/'),
        query.finish()
    )
}

fn pkce_challenge(verifier: &str) -> String {
    use sha2::Digest as _;
    BASE64_URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(verifier.as_bytes()))
}

fn random_urlsafe(length: usize) -> Result<String, AuthError> {
    let mut bytes = vec![0_u8; length];
    getrandom::fill(&mut bytes)
        .map_err(|error| AuthError::Message(format!("secure random generation failed: {error}")))?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(bytes))
}

fn format_browser_token_error(
    status: reqwest::StatusCode,
    error: Option<&OAuthErrorResponse>,
) -> String {
    let code = error
        .and_then(|error| error.error.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let description = error
        .and_then(|error| error.error_description.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty());
    match (code, description) {
        (Some(code), Some(description)) => {
            format!("ChatGPT browser token exchange failed: {status} {code} ({description})")
        }
        (Some(code), None) => {
            format!("ChatGPT browser token exchange failed: {status} {code}")
        }
        _ => format!("ChatGPT browser token exchange failed: {status}"),
    }
}

fn device_code_request_error(
    status: reqwest::StatusCode,
    headers: &reqwest::header::HeaderMap,
) -> AuthError {
    let cloudflare_challenge = headers
        .get("cf-mitigated")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("challenge"));

    let message = if status == reqwest::StatusCode::FORBIDDEN && cloudflare_challenge {
        "ChatGPT device-code sign-in was blocked by a network security challenge (HTTP 403, Cloudflare). Try another network or disable a VPN, proxy, or content filter, then retry. The request failed before ChatGPT could check an account or plan."
            .to_string()
    } else if status == reqwest::StatusCode::FORBIDDEN {
        "OpenAI denied the ChatGPT device-code request before sign-in began (HTTP 403). Retry later; if it persists, try another network and confirm device-code login is enabled in ChatGPT security settings. ChatGPT did not check an account or plan yet."
            .to_string()
    } else if status == reqwest::StatusCode::NOT_FOUND {
        "ChatGPT device-code login is not available at this authentication server (HTTP 404). Verify the server URL or use a supported sign-in method."
            .to_string()
    } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        "OpenAI temporarily rate-limited the ChatGPT device-code request (HTTP 429). Wait a moment, then retry Authenticate once."
            .to_string()
    } else if status.is_server_error() {
        format!(
            "OpenAI's authentication service is temporarily unavailable ({status}). Retry later; no ChatGPT account or plan was checked."
        )
    } else {
        format!(
            "ChatGPT device-code request failed before sign-in began ({status}). No ChatGPT account or plan was checked."
        )
    };

    AuthError::Message(message)
}

fn default_oauth_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .connect_timeout(DEFAULT_OAUTH_CONNECT_TIMEOUT)
        .timeout(DEFAULT_OAUTH_REQUEST_TIMEOUT)
        .build()
        .map_err(|error| {
            format!("static ChatGPT OAuth HTTP client configuration must build: {error}")
        })
}

fn auth_request_error(stage: &str, error: reqwest::Error) -> AuthError {
    let retry = if error.is_timeout() {
        " The authentication service did not respond before the configured network deadline. Check connectivity to auth.openai.com and try again."
    } else {
        " Check connectivity to auth.openai.com and try again."
    };
    AuthError::Message(format!("ChatGPT {stage} failed: {error}.{retry}"))
}

fn build_auth_record(
    tokens: OAuthTokenResponse,
    previous_refresh_token: Option<String>,
) -> AuthRecord {
    let access_token = Some(tokens.access_token);
    let id_token = tokens.id_token;
    AuthRecord {
        expires_at: access_token
            .as_deref()
            .and_then(extract_expiration_timestamp),
        account_id: extract_account_id(id_token.as_deref()).or_else(|| {
            access_token
                .as_deref()
                .and_then(|token| extract_account_id(Some(token)))
        }),
        access_token,
        refresh_token: tokens.refresh_token.or(previous_refresh_token),
        id_token,
        reauth_required: false,
    }
}

fn sign_in_required() -> AuthError {
    AuthError::Message(
        "ChatGPT sign-in required. Reconnect ChatGPT in Settings before using this provider."
            .into(),
    )
}

fn extract_expiration_timestamp(token: &str) -> Option<i64> {
    decode_jwt_claims(token)
        .get("exp")
        .and_then(|value| value.as_i64().or_else(|| value.as_u64().map(|v| v as i64)))
}

fn extract_account_id(token: Option<&str>) -> Option<String> {
    let claims = decode_jwt_claims(token?);
    claims
        .get("https://api.openai.com/auth")
        .and_then(|value| value.as_object())
        .and_then(|map| map.get("chatgpt_account_id"))
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
}

fn decode_jwt_claims(token: &str) -> serde_json::Value {
    let payload = token.split('.').nth(1).unwrap_or_default();
    let decoded = BASE64_URL_SAFE_NO_PAD.decode(payload.as_bytes());
    decoded
        .ok()
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
        .unwrap_or(serde_json::Value::Null)
}

fn should_reauthenticate_after_refresh(
    status: reqwest::StatusCode,
    error_code: Option<&str>,
) -> bool {
    matches!(
        status,
        reqwest::StatusCode::BAD_REQUEST | reqwest::StatusCode::UNAUTHORIZED
    ) && matches!(error_code, Some("invalid_grant"))
        // The refresh endpoint also returns OpenAI's nested error envelope for
        // credentials it cannot validate. Do not broaden this to generic 401s.
        || (status == reqwest::StatusCode::UNAUTHORIZED
            && error_code == Some("token_expired"))
}

fn format_refresh_error(
    status: reqwest::StatusCode,
    oauth_error: Option<&OAuthErrorResponse>,
    body: &str,
) -> String {
    let error_code = oauth_error.and_then(OAuthErrorResponse::code);
    let description = oauth_error.and_then(OAuthErrorResponse::description);

    if let Some(description) = description
        .map(str::trim)
        .filter(|description| !description.is_empty())
    {
        return format!(
            "ChatGPT token refresh failed: {status} {} ({description})",
            error_code.unwrap_or("unknown_error")
        );
    }

    if let Some(error_code) = error_code {
        return format!("ChatGPT token refresh failed: {status} {error_code}");
    }

    if !body.trim().is_empty() {
        return format!("ChatGPT token refresh failed: {status} {body}");
    }

    format!("ChatGPT token refresh failed: {status}")
}

fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U64OrString {
        U64(u64),
        String(String),
    }

    let value = Option::<U64OrString>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(U64OrString::U64(value)) => Ok(Some(value)),
        Some(U64OrString::String(value)) => {
            let value = value.trim();
            if value.is_empty() {
                Ok(None)
            } else {
                value
                    .parse::<u64>()
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DeviceCodeHandler, DeviceCodeResponse, OAuthErrorResponse, OAuthTokenResponse,
        PlatformAuthenticator, RefreshTokensError, build_auth_record, build_browser_authorize_url,
        device_code_request_error, format_browser_token_error, format_refresh_error,
        should_reauthenticate_after_refresh, wait_for_browser_callback,
    };
    use reqwest::StatusCode;
    use std::time::Duration;

    fn jwt_with_exp(exp: i64) -> String {
        use base64::Engine as _;
        use base64::prelude::BASE64_URL_SAFE_NO_PAD;
        let payload = BASE64_URL_SAFE_NO_PAD.encode(serde_json::json!({"exp": exp}).to_string());
        format!("header.{payload}.signature")
    }

    async fn one_response_server(
        status: &str,
        body: String,
    ) -> (String, tokio::task::JoinHandle<()>) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind fixture server");
        let address = listener.local_addr().expect("fixture address");
        let status = status.to_string();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept request");
            let mut request = vec![0_u8; 4096];
            let _ = socket.read(&mut request).await.expect("read request");
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket
                .write_all(response.as_bytes())
                .await
                .expect("write response");
        });
        (format!("http://{address}/token"), server)
    }

    #[test]
    fn device_code_response_accepts_numeric_interval() {
        let response: DeviceCodeResponse = serde_json::from_str(
            r#"{
                "device_auth_id": "deviceauth_123",
                "user_code": "ABCD-EFGH",
                "interval": 5
            }"#,
        )
        .expect("device code response");

        assert_eq!(response.interval, Some(5));
    }

    #[test]
    fn device_code_response_accepts_string_interval() {
        let response: DeviceCodeResponse = serde_json::from_str(
            r#"{
                "device_auth_id": "deviceauth_123",
                "user_code": "ABCD-EFGH",
                "interval": "5"
            }"#,
        )
        .expect("device code response");

        assert_eq!(response.interval, Some(5));
    }

    #[test]
    fn device_code_denials_have_stable_actionable_classes() {
        let empty = reqwest::header::HeaderMap::new();

        let denied = device_code_request_error(StatusCode::FORBIDDEN, &empty).to_string();
        assert!(denied.contains("before sign-in began"), "{denied}");
        assert!(
            denied.contains("did not check an account or plan"),
            "{denied}"
        );

        let unavailable = device_code_request_error(StatusCode::NOT_FOUND, &empty).to_string();
        assert!(unavailable.contains("not available"), "{unavailable}");

        let limited = device_code_request_error(StatusCode::TOO_MANY_REQUESTS, &empty).to_string();
        assert!(limited.contains("rate-limited"), "{limited}");

        let upstream =
            device_code_request_error(StatusCode::SERVICE_UNAVAILABLE, &empty).to_string();
        assert!(upstream.contains("temporarily unavailable"), "{upstream}");
    }

    #[test]
    fn browser_authorize_url_matches_first_party_pkce_contract() {
        let url = url::Url::parse(&build_browser_authorize_url(
            "https://auth.openai.com/",
            "http://localhost:1455/auth/callback",
            "challenge-value",
            "state-value",
        ))
        .expect("valid authorize URL");
        let params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();

        assert_eq!(
            url.as_str().split('?').next(),
            Some("https://auth.openai.com/oauth/authorize")
        );
        assert_eq!(
            params.get("response_type").map(String::as_str),
            Some("code")
        );
        assert_eq!(
            params.get("client_id").map(String::as_str),
            Some("app_EMoamEEZ73f0CkXaXp7hrann")
        );
        assert_eq!(
            params.get("redirect_uri").map(String::as_str),
            Some("http://localhost:1455/auth/callback")
        );
        assert_eq!(
            params.get("code_challenge_method").map(String::as_str),
            Some("S256")
        );
        assert_eq!(
            params.get("id_token_add_organizations").map(String::as_str),
            Some("true")
        );
        assert_eq!(
            params.get("codex_cli_simplified_flow").map(String::as_str),
            Some("true")
        );
        assert_eq!(
            params.get("originator").map(String::as_str),
            Some("codex_cli_rs")
        );
        assert_eq!(params.get("state").map(String::as_str), Some("state-value"));
        assert_eq!(
            params.get("code_challenge").map(String::as_str),
            Some("challenge-value")
        );
    }

    #[tokio::test]
    async fn browser_callback_rejects_wrong_state_without_consuming_attempt() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind callback fixture");
        let address = listener.local_addr().expect("callback address");
        let callback =
            tokio::spawn(
                async move { wait_for_browser_callback(listener, "expected-state").await },
            );

        let mut wrong = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect wrong-state callback");
        wrong
            .write_all(
                b"GET /auth/callback?state=wrong&code=must-not-win HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
            )
            .await
            .expect("send wrong callback");
        let mut wrong_response = String::new();
        wrong
            .read_to_string(&mut wrong_response)
            .await
            .expect("read wrong-state response");
        assert!(
            wrong_response.starts_with("HTTP/1.1 400"),
            "{wrong_response}"
        );

        let mut valid = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect valid callback");
        valid
            .write_all(
                b"GET /auth/callback?state=expected-state&code=authorization-code HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
            )
            .await
            .expect("send valid callback");
        let mut valid_response = String::new();
        valid
            .read_to_string(&mut valid_response)
            .await
            .expect("read valid response");
        assert!(
            valid_response.starts_with("HTTP/1.1 200"),
            "{valid_response}"
        );

        assert_eq!(
            callback
                .await
                .expect("callback task")
                .expect("valid callback"),
            "authorization-code"
        );
    }

    #[tokio::test]
    async fn idle_loopback_connection_cannot_block_a_valid_callback() {
        use tokio::io::AsyncWriteExt as _;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind callback fixture");
        let address = listener.local_addr().expect("callback address");
        let callback =
            tokio::spawn(
                async move { wait_for_browser_callback(listener, "expected-state").await },
            );

        let _idle = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect idle socket");
        tokio::task::yield_now().await;
        let mut valid = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect valid callback");
        valid
            .write_all(
                b"GET /auth/callback?state=expected-state&code=valid-code HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
            )
            .await
            .expect("send valid callback");

        assert_eq!(
            tokio::time::timeout(Duration::from_millis(300), callback)
                .await
                .expect("idle connection must not block valid callback")
                .expect("callback task")
                .expect("valid callback"),
            "valid-code"
        );
    }

    #[tokio::test]
    async fn cancelling_browser_callback_wait_releases_the_registered_port() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind callback fixture");
        let address = listener.local_addr().expect("callback address");
        let callback =
            tokio::spawn(async move { wait_for_browser_callback(listener, "state").await });

        callback.abort();
        let cancelled = callback.await.expect_err("callback wait must be cancelled");
        assert!(cancelled.is_cancelled());

        let rebound = tokio::net::TcpListener::bind(address)
            .await
            .expect("cancelled callback wait must release its loopback port");
        assert_eq!(rebound.local_addr().expect("rebound address"), address);
    }

    #[tokio::test]
    async fn fragmented_state_waits_for_complete_http_headers() {
        use tokio::io::AsyncWriteExt as _;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind callback fixture");
        let address = listener.local_addr().expect("callback address");
        let callback =
            tokio::spawn(
                async move { wait_for_browser_callback(listener, "expected-state").await },
            );
        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect callback");
        stream
            .write_all(b"GET /auth/callback?sta")
            .await
            .expect("write first fragment");
        tokio::time::sleep(Duration::from_millis(25)).await;
        assert!(
            !callback.is_finished(),
            "partial state consumed the attempt"
        );
        stream
            .write_all(
                b"te=expected-state&code=fragmented-code HTTP/1.1\r\nHost: localhost\r\n\r\n",
            )
            .await
            .expect("write remaining callback");

        assert_eq!(
            callback
                .await
                .expect("callback task")
                .expect("valid fragmented callback"),
            "fragmented-code"
        );
    }

    #[tokio::test]
    async fn fragmented_code_is_not_consumed_before_request_completion() {
        use tokio::io::AsyncWriteExt as _;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind callback fixture");
        let address = listener.local_addr().expect("callback address");
        let callback =
            tokio::spawn(
                async move { wait_for_browser_callback(listener, "expected-state").await },
            );
        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect callback");
        stream
            .write_all(b"GET /auth/callback?state=expected-state&code=partial")
            .await
            .expect("write first fragment");
        tokio::time::sleep(Duration::from_millis(25)).await;
        assert!(!callback.is_finished(), "partial code consumed the attempt");
        stream
            .write_all(b"-complete HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .expect("write remaining callback");

        assert_eq!(
            callback
                .await
                .expect("callback task")
                .expect("valid fragmented callback"),
            "partial-complete"
        );
    }

    #[test]
    fn browser_token_error_does_not_echo_unparsed_payload() {
        assert_eq!(
            format_browser_token_error(StatusCode::BAD_GATEWAY, None),
            "ChatGPT browser token exchange failed: 502 Bad Gateway"
        );
        let parsed = OAuthErrorResponse {
            error: Some("access_denied".into()),
            error_description: Some("workspace access is unavailable".into()),
        };
        assert_eq!(
            format_browser_token_error(StatusCode::FORBIDDEN, Some(&parsed)),
            "ChatGPT browser token exchange failed: 403 Forbidden access_denied (workspace access is unavailable)"
        );
    }

    #[tokio::test]
    async fn cloudflare_device_code_challenge_is_classified_without_echoing_body() {
        use tokio::io::AsyncWriteExt;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind fixture server");
        let address = listener.local_addr().expect("fixture address");
        let secret_body = "sensitive-upstream-body-must-not-leak";
        let response = format!(
            "HTTP/1.1 403 Forbidden\r\nCf-Mitigated: challenge\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{secret_body}",
            secret_body.len()
        );
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept request");
            socket
                .write_all(response.as_bytes())
                .await
                .expect("write fixture response");
        });
        let auth = PlatformAuthenticator::new(
            None,
            Some(reqwest::Client::new()),
            DeviceCodeHandler::default(),
            true,
        );
        let endpoint = format!("http://{address}/device-code");

        let error = auth
            .login_device_flow_at(&endpoint, &endpoint, &endpoint)
            .await
            .expect_err("Cloudflare challenge must stop before polling")
            .to_string();

        server.await.expect("fixture server exits");
        assert!(error.contains("network security challenge"), "{error}");
        assert!(error.contains("Cloudflare"), "{error}");
        assert!(
            error.contains("before ChatGPT could check an account or plan"),
            "{error}"
        );
        assert!(
            !error.contains(secret_body),
            "upstream response body leaked: {error}"
        );
    }

    #[tokio::test]
    async fn cloudflare_device_denial_falls_back_to_browser_pkce_and_returns_refreshable_auth() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind OAuth fixture");
        let address = listener.local_addr().expect("OAuth fixture address");
        let access_token = jwt_with_exp(i64::MAX - 2);
        let token_body = serde_json::json!({
            "access_token": access_token,
            "refresh_token": "refresh-from-browser",
            "id_token": null,
        })
        .to_string();
        let server = tokio::spawn(async move {
            let (mut device, _) = listener.accept().await.expect("accept device request");
            let mut request = vec![0_u8; 8192];
            let count = device
                .read(&mut request)
                .await
                .expect("read device request");
            assert!(
                String::from_utf8_lossy(request.get(..count).unwrap_or_default())
                    .starts_with("POST /device-code ")
            );
            device
                .write_all(
                    b"HTTP/1.1 403 Forbidden\r\nCf-Mitigated: challenge\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .await
                .expect("write device denial");

            let (mut token, _) = listener.accept().await.expect("accept token exchange");
            let count = token.read(&mut request).await.expect("read token exchange");
            let request = String::from_utf8_lossy(request.get(..count).unwrap_or_default());
            assert!(request.starts_with("POST /token "), "{request}");
            assert!(
                request.contains("grant_type=authorization_code"),
                "{request}"
            );
            assert!(request.contains("code=browser-code"), "{request}");
            assert!(request.contains("code_verifier="), "{request}");
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{token_body}",
                token_body.len()
            );
            token
                .write_all(response.as_bytes())
                .await
                .expect("write token response");
        });

        let (prompt_tx, mut prompt_rx) = tokio::sync::mpsc::unbounded_channel();
        let auth = PlatformAuthenticator::new(
            None,
            Some(reqwest::Client::new()),
            DeviceCodeHandler::new(move |prompt| {
                let _ = prompt_tx.send(prompt);
            }),
            true,
        );
        let device_url = format!("http://{address}/device-code");
        let token_url = format!("http://{address}/token");
        let issuer = format!("http://{address}");
        let login = tokio::spawn(async move {
            auth.login_with_fallback_at(
                &device_url,
                &device_url,
                &token_url,
                &issuer,
                &token_url,
                &[0],
            )
            .await
        });

        let prompt = prompt_rx.recv().await.expect("browser prompt");
        assert!(prompt.user_code.is_empty());
        let authorize_url = url::Url::parse(&prompt.verification_uri).expect("authorize URL");
        let params: std::collections::HashMap<_, _> =
            authorize_url.query_pairs().into_owned().collect();
        let redirect_uri = params.get("redirect_uri").expect("redirect URI");
        let state = params.get("state").expect("OAuth state");
        let redirect = url::Url::parse(redirect_uri).expect("redirect URL");
        let callback_address = format!(
            "{}:{}",
            redirect.host_str().expect("redirect host"),
            redirect.port().expect("redirect port")
        );
        let mut callback = tokio::net::TcpStream::connect(callback_address)
            .await
            .expect("connect browser callback");
        callback
            .write_all(
                format!(
                    "GET /auth/callback?state={state}&code=browser-code HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
                )
                .as_bytes(),
            )
            .await
            .expect("send browser callback");
        let mut callback_response = String::new();
        callback
            .read_to_string(&mut callback_response)
            .await
            .expect("read browser callback response");
        assert!(callback_response.starts_with("HTTP/1.1 200"));

        let record = login
            .await
            .expect("login task")
            .expect("browser fallback login");
        server.await.expect("OAuth fixture exits");
        assert_eq!(record.access_token.as_deref(), Some(access_token.as_str()));
        assert_eq!(
            record.refresh_token.as_deref(),
            Some("refresh-from-browser")
        );
        assert_eq!(record.expires_at, Some(i64::MAX - 2));
        assert!(!record.reauth_required);
    }

    #[test]
    fn refresh_reauth_only_on_terminal_credential_errors() {
        assert!(should_reauthenticate_after_refresh(
            StatusCode::BAD_REQUEST,
            Some("invalid_grant")
        ));
        assert!(should_reauthenticate_after_refresh(
            StatusCode::UNAUTHORIZED,
            Some("invalid_grant")
        ));
        assert!(!should_reauthenticate_after_refresh(
            StatusCode::BAD_GATEWAY,
            Some("invalid_grant")
        ));
        assert!(!should_reauthenticate_after_refresh(
            StatusCode::BAD_REQUEST,
            Some("invalid_request")
        ));
        assert!(!should_reauthenticate_after_refresh(
            StatusCode::UNAUTHORIZED,
            None
        ));
    }

    #[tokio::test]
    async fn noninteractive_oauth_requires_sign_in_instead_of_device_flow() {
        let auth = PlatformAuthenticator::new(None, None, DeviceCodeHandler::default(), false);
        let context = auth
            .cached_or_refreshed_context()
            .await
            .expect("missing cached auth is not a transport failure");
        assert!(
            context.is_none(),
            "missing cached auth must request sign-in without starting device flow"
        );
    }

    #[tokio::test]
    async fn device_code_request_obeys_injected_http_deadline() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind blackhole server");
        let address = listener.local_addr().expect("blackhole address");
        let server = tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.expect("accept OAuth request");
            std::future::pending::<()>().await;
        });
        let http = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_millis(100))
            .timeout(std::time::Duration::from_millis(150))
            .build()
            .expect("bounded OAuth client");
        let auth = PlatformAuthenticator::new(None, Some(http), DeviceCodeHandler::default(), true);
        let endpoint = format!("http://{address}/device-code");
        let started = std::time::Instant::now();

        let error = auth
            .login_device_flow_at(&endpoint, &endpoint, &endpoint)
            .await
            .expect_err("an unresponsive authentication service must time out")
            .to_string();

        server.abort();
        assert!(
            started.elapsed() < std::time::Duration::from_secs(1),
            "injected request deadline was not honored: {:?}",
            started.elapsed()
        );
        assert!(error.contains("device-code request failed"), "{error}");
        assert!(error.contains("configured network deadline"), "{error}");
        assert!(error.contains("auth.openai.com"), "{error}");
    }

    #[tokio::test]
    async fn refresh_request_obeys_injected_deadline_without_exposing_token() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind blackhole server");
        let address = listener.local_addr().expect("blackhole address");
        let server = tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.expect("accept OAuth request");
            std::future::pending::<()>().await;
        });
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(150))
            .build()
            .expect("bounded OAuth client");
        let auth = PlatformAuthenticator::new(None, Some(http), DeviceCodeHandler::default(), true);
        let endpoint = format!("http://{address}/token");
        let refresh_token = "refresh-token-must-stay-secret";

        let error = match auth.refresh_tokens_at(&endpoint, refresh_token).await {
            Err(RefreshTokensError::Auth(error)) => error.to_string(),
            Err(RefreshTokensError::Reauthenticate) => {
                panic!("an unreachable provider is transient, not terminal auth rejection")
            }
            Ok(_) => panic!("an unresponsive authentication service must time out"),
        };

        server.abort();
        assert!(error.contains("token refresh failed"), "{error}");
        assert!(error.contains("configured network deadline"), "{error}");
        assert!(!error.contains(refresh_token), "{error}");
    }

    #[test]
    fn refresh_error_uses_oauth_description_when_present() {
        let oauth_error = OAuthErrorResponse {
            error: Some(super::OAuthError::Code("temporarily_unavailable".into())),
            error_description: Some("please retry".into()),
        };

        assert_eq!(
            format_refresh_error(StatusCode::BAD_GATEWAY, Some(&oauth_error), ""),
            "ChatGPT token refresh failed: 502 Bad Gateway temporarily_unavailable (please retry)"
        );
    }

    #[test]
    fn build_auth_record_preserves_existing_refresh_token_when_refresh_omits_one() {
        let record = build_auth_record(
            OAuthTokenResponse {
                access_token: "access-token".into(),
                refresh_token: None,
                id_token: None,
            },
            Some("cached-refresh-token".into()),
        );

        assert_eq!(
            record.refresh_token.as_deref(),
            Some("cached-refresh-token")
        );
    }

    #[tokio::test]
    async fn rejected_access_token_refreshes_once_and_persists_the_replacement() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let auth_file = temp.path().join("auth.json");
        let old_access = jwt_with_exp(i64::MAX);
        std::fs::write(
            &auth_file,
            serde_json::to_vec_pretty(&serde_json::json!({
                "access_token": old_access,
                "refresh_token": "refresh-1",
                "expires_at": i64::MAX,
            }))
            .expect("serialize seed auth"),
        )
        .expect("write seed auth");
        let new_access = jwt_with_exp(i64::MAX - 1);
        let (endpoint, server) = one_response_server(
            "200 OK",
            serde_json::json!({
                "access_token": new_access,
                "refresh_token": "refresh-2",
            })
            .to_string(),
        )
        .await;
        let auth = PlatformAuthenticator::new(
            Some(auth_file.clone()),
            Some(reqwest::Client::new()),
            DeviceCodeHandler::default(),
            false,
        );

        let refreshed = auth
            .refresh_after_rejection_at(&endpoint, &old_access)
            .await
            .expect("rejected token should refresh");
        server.await.expect("fixture server exits");
        assert_eq!(refreshed.access_token, new_access);

        let persisted: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&auth_file).expect("read persisted auth"))
                .expect("parse persisted auth");
        assert_eq!(persisted["access_token"], new_access);
        assert_eq!(persisted["refresh_token"], "refresh-2");
        assert_eq!(persisted["reauth_required"], false);
    }

    #[tokio::test]
    async fn terminal_refresh_failure_persists_reauthentication_state() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let auth_file = temp.path().join("auth.json");
        let old_access = jwt_with_exp(i64::MAX);
        std::fs::write(
            &auth_file,
            serde_json::to_vec_pretty(&serde_json::json!({
                "access_token": old_access,
                "refresh_token": "refresh-secret",
                "expires_at": i64::MAX,
            }))
            .expect("serialize seed auth"),
        )
        .expect("write seed auth");
        let (endpoint, server) = one_response_server(
            "400 Bad Request",
            serde_json::json!({"error": "invalid_grant"}).to_string(),
        )
        .await;
        let auth = PlatformAuthenticator::new(
            Some(auth_file.clone()),
            Some(reqwest::Client::new()),
            DeviceCodeHandler::default(),
            false,
        );

        let error = auth
            .refresh_after_rejection_at(&endpoint, &old_access)
            .await
            .expect_err("invalid refresh grant requires sign-in")
            .to_string();
        server.await.expect("fixture server exits");
        assert!(error.contains("ChatGPT sign-in required"), "{error}");

        let persisted: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&auth_file).expect("read terminal auth state"))
                .expect("parse terminal auth state");
        assert_eq!(persisted["reauth_required"], true);
        assert!(persisted["access_token"].is_null());
        assert!(persisted["refresh_token"].is_null());
        assert!(!persisted.to_string().contains("refresh-secret"));
    }

    // Replay the real OAuth refresh response observed in native Mem acceptance.
    // This private auth seam also verifies credential persistence, outside completion cassettes.
    #[tokio::test]
    async fn nested_token_expired_persists_reauthentication_state() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let auth_file = temp.path().join("auth.json");
        let old_access = jwt_with_exp(i64::MAX);
        std::fs::write(
            &auth_file,
            serde_json::to_vec_pretty(&serde_json::json!({
                "access_token": old_access,
                "refresh_token": "refresh-secret",
                "expires_at": i64::MAX,
            }))
            .expect("serialize seed auth"),
        )
        .expect("write seed auth");
        let (endpoint, server) = one_response_server(
            "401 Unauthorized",
            serde_json::json!({"error": {
                "message": "Could not validate your token. Please try signing in again.",
                "type": "invalid_request_error",
                "param": null,
                "code": "token_expired"
            }})
            .to_string(),
        )
        .await;
        let auth = PlatformAuthenticator::new(
            Some(auth_file.clone()),
            Some(reqwest::Client::new()),
            DeviceCodeHandler::default(),
            false,
        );

        let error = auth
            .refresh_after_rejection_at(&endpoint, &old_access)
            .await
            .expect_err("expired refresh credential requires sign-in")
            .to_string();
        server.await.expect("fixture server exits");
        assert!(error.contains("ChatGPT sign-in required"), "{error}");

        let persisted: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&auth_file).expect("read terminal auth state"))
                .expect("parse terminal auth state");
        assert_eq!(persisted["reauth_required"], true);
        assert!(persisted["access_token"].is_null());
        assert!(persisted["refresh_token"].is_null());
        assert!(!persisted.to_string().contains("refresh-secret"));
    }

    // Exercise the private OAuth HTTP seam: completion cassettes do not cover refresh.
    #[tokio::test]
    async fn nonterminal_refresh_responses_preserve_credentials() {
        for (status, body) in [
            (
                "401 Unauthorized",
                r#"{"error":{"code":"invalid_request_error"}}"#,
            ),
            (
                "401 Unauthorized",
                r#"{"error":{"message":"token_expired"}}"#,
            ),
            ("401 Unauthorized", "<html>Access denied</html>"),
            ("403 Forbidden", r#"{"error":{"code":"token_expired"}}"#),
            (
                "429 Too Many Requests",
                r#"{"error":{"code":"token_expired"}}"#,
            ),
            ("502 Bad Gateway", r#"{"error":{"code":"token_expired"}}"#),
            ("400 Bad Request", r#"{"error":{"code":"token_expired"}}"#),
        ] {
            let temp = assert_fs::TempDir::new().expect("temp auth directory");
            let auth_file = temp.path().join("auth.json");
            let access = jwt_with_exp(i64::MAX);
            let seed = serde_json::to_vec(&serde_json::json!({
                "access_token": access,
                "refresh_token": "valid-refresh",
                "expires_at": i64::MAX,
            }))
            .expect("serialize auth");
            std::fs::write(&auth_file, &seed).expect("seed auth");
            let (endpoint, server) = one_response_server(status, body.into()).await;
            let auth = PlatformAuthenticator::new(
                Some(auth_file.clone()),
                Some(reqwest::Client::new()),
                DeviceCodeHandler::default(),
                false,
            );
            let error = auth
                .refresh_after_rejection_at(&endpoint, &access)
                .await
                .expect_err("nonterminal failure must surface")
                .to_string();
            server.await.expect("fixture exits");
            assert!(
                !error.contains("ChatGPT sign-in required"),
                "{status}: {error}"
            );
            assert_eq!(
                std::fs::read(&auth_file).expect("read auth"),
                seed,
                "{status}"
            );
        }
    }

    #[test]
    fn stale_invalid_grant_cannot_erase_a_rotated_credential() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let auth_file = temp.path().join("auth.json");
        let replacement = jwt_with_exp(i64::MAX);
        std::fs::write(
            &auth_file,
            serde_json::to_vec_pretty(&serde_json::json!({
                "access_token": replacement,
                "refresh_token": "refresh-rotated",
                "expires_at": i64::MAX,
            }))
            .expect("serialize replacement auth"),
        )
        .expect("write replacement auth");
        let auth = PlatformAuthenticator::new(
            Some(auth_file.clone()),
            Some(reqwest::Client::new()),
            DeviceCodeHandler::default(),
            false,
        );

        let context = auth
            .mark_reauth_required(Some("access-rejected"), Some("refresh-stale"))
            .expect("stale rejection should re-read the credential")
            .expect("fresh rotated access token should be reused");
        assert_eq!(context.access_token, replacement);

        let persisted: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&auth_file).expect("read persisted auth"))
                .expect("parse persisted auth");
        assert_eq!(persisted["access_token"], replacement);
        assert_eq!(persisted["refresh_token"], "refresh-rotated");
        assert_ne!(persisted["reauth_required"], true);
    }

    #[tokio::test]
    async fn rejected_request_reuses_a_newer_persisted_token_without_refreshing_again() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let auth_file = temp.path().join("auth.json");
        let replacement = jwt_with_exp(i64::MAX);
        std::fs::write(
            &auth_file,
            serde_json::to_vec_pretty(&serde_json::json!({
                "access_token": replacement,
                "refresh_token": "refresh-must-not-be-used",
                "expires_at": i64::MAX,
            }))
            .expect("serialize replacement auth"),
        )
        .expect("write replacement auth");
        let auth = PlatformAuthenticator::new(
            Some(auth_file),
            Some(reqwest::Client::new()),
            DeviceCodeHandler::default(),
            false,
        );

        let recovered = auth
            .refresh_after_rejection_at("http://127.0.0.1:1/must-not-run", "rejected-token")
            .await
            .expect("a concurrent refresh result should be reused");
        assert_eq!(recovered.access_token, replacement);
    }
}
