use super::{AuthContext, AuthError, DeviceCodeHandler, DeviceCodePrompt};
use crate::providers::internal::device_auth::{
    emit_device_code_prompt, ensure_parent_dir, read_json_record, token_expired, write_json_record,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

// Keep generations after deletion: an empty cache can still have a revoked
// device login waiting to write its first token.
static AUTH_CACHE_LOCK: Mutex<BTreeMap<PathBuf, u64>> = Mutex::new(BTreeMap::new());
const REAUTH_REQUIRED: &str =
    "GitHub Copilot sign-in required. Reconnect Copilot in Settings before using this provider.";

const GITHUB_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";
const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_API_KEY_URL: &str = "https://api.github.com/copilot_internal/v2/token";
const DEVICE_CODE_POLL_SLEEP_SECONDS: u64 = 5;
const DEVICE_CODE_TIMEOUT_SECONDS: u64 = 15 * 60;
const DEVICE_CODE_SLOW_DOWN_SECONDS: u64 = 5;

fn advance_cache_generation(
    generations: &mut BTreeMap<PathBuf, u64>,
    access_token: &std::path::Path,
) -> std::io::Result<u64> {
    let generation = generations.entry(access_token.to_path_buf()).or_default();
    *generation = generation
        .checked_add(1)
        .ok_or_else(|| std::io::Error::other("GitHub Copilot cache generation exhausted"))?;
    Ok(*generation)
}

/// Revoke pending device logins and disconnect under the auth commit lock.
/// Revocation applies even when cache files are absent or cannot be removed.
pub fn clear_oauth_cache(
    access_token: &std::path::Path,
    api_key: &std::path::Path,
) -> std::io::Result<usize> {
    let mut generations = AUTH_CACHE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    advance_cache_generation(&mut generations, access_token)?;
    let mut removed = 0;
    for path in [access_token, api_key] {
        match std::fs::remove_file(path) {
            Ok(()) => removed += 1,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(removed)
}

#[derive(Debug, Clone)]
pub(super) struct PlatformAuthenticator {
    access_token_file: Option<PathBuf>,
    api_key_file: Option<PathBuf>,
    device_code_handler: DeviceCodeHandler,
    allow_device_flow: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
struct ApiKeyRecord {
    token: Option<String>,
    expires_at: Option<i64>,
    endpoints: Option<ApiKeyEndpoints>,
    bootstrap_token_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    reauth_required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
struct ApiKeyEndpoints {
    api: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    interval: Option<u64>,
    expires_in: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AccessTokenState {
    token: String,
    from_cache: bool,
}

impl PlatformAuthenticator {
    pub(super) fn new(
        access_token_file: Option<PathBuf>,
        api_key_file: Option<PathBuf>,
        device_code_handler: DeviceCodeHandler,
        allow_device_flow: bool,
    ) -> Self {
        Self {
            access_token_file,
            api_key_file,
            device_code_handler,
            allow_device_flow,
        }
    }

    pub(super) async fn auth_context_oauth(&self) -> Result<AuthContext, AuthError> {
        self.auth_context_oauth_at(GITHUB_API_KEY_URL).await
    }

    async fn auth_context_oauth_at(&self, endpoint: &str) -> Result<AuthContext, AuthError> {
        let record: ApiKeyRecord = read_json_record(self.api_key_file.as_deref())?;
        let expected_record = record.clone();
        let cached_access_token = self.read_access_token().ok().flatten();
        let api_base = record.api_base();
        if record.reauth_required
            && cached_access_token
                .as_deref()
                .is_some_and(|token| record.matches_bootstrap_token(token))
            && !self.allow_device_flow
        {
            return Err(AuthError::Message(REAUTH_REQUIRED.into()));
        }
        if record.can_reuse_for_oauth(cached_access_token.as_deref())
            && let Some(token) = record.token
        {
            return Ok(AuthContext {
                api_key: token,
                api_base,
            });
        }

        let access_token = if let Some(token) = cached_access_token {
            AccessTokenState {
                token,
                from_cache: true,
            }
        } else {
            self.access_token().await?
        };
        let (record, bootstrap) = match self.refresh_api_key_at(&access_token.token, endpoint).await
        {
            Ok(refreshed) => (
                refreshed.bind_to_bootstrap_token(&access_token.token),
                access_token.token,
            ),
            Err(err) if access_token.from_cache && should_retry_with_fresh_access_token(&err) => {
                if !self.allow_device_flow {
                    self.retain_terminal_rejection(&access_token.token, &record)?;
                    return Err(AuthError::Message(REAUTH_REQUIRED.into()));
                }
                let fresh_access_token = self.reauthenticate_access_token().await?;
                (
                    self.refresh_api_key_at(&fresh_access_token, endpoint)
                        .await?
                        .bind_to_bootstrap_token(&fresh_access_token),
                    fresh_access_token,
                )
            }
            Err(err) => return Err(err),
        };
        let api_base = record.api_base();
        self.write_record_if_current(&bootstrap, &record, Some(&expected_record))?;
        Ok(AuthContext {
            api_key: record.token.unwrap_or_default(),
            api_base,
        })
    }

    pub(super) async fn auth_context_with_github_access_token(
        &self,
        access_token: &str,
    ) -> Result<AuthContext, AuthError> {
        self.auth_context_with_github_access_token_at(access_token, GITHUB_API_KEY_URL)
            .await
    }

    async fn auth_context_with_github_access_token_at(
        &self,
        access_token: &str,
        endpoint: &str,
    ) -> Result<AuthContext, AuthError> {
        let record: ApiKeyRecord = read_json_record(self.api_key_file.as_deref())?;
        let expected_record = record.clone();
        let expected_bootstrap = self.read_access_token()?;
        let api_base = record.api_base();
        if record.can_reuse_for_bootstrap_token(access_token)
            && let Some(token) = record.token
        {
            return Ok(AuthContext {
                api_key: token,
                api_base,
            });
        }

        let record = self
            .refresh_api_key_at(access_token, endpoint)
            .await?
            .bind_to_bootstrap_token(access_token);
        let api_base = record.api_base();
        {
            let _guard = AUTH_CACHE_LOCK
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            let current_record: ApiKeyRecord = read_json_record(self.api_key_file.as_deref())?;
            if current_record != expected_record || self.read_access_token()? != expected_bootstrap
            {
                return Err(AuthError::Message(
                    "GitHub Copilot credentials changed during refresh. Try again.".into(),
                ));
            }
            write_json_record(self.api_key_file.as_deref(), &record)?;
        }
        Ok(AuthContext {
            api_key: record.token.unwrap_or_default(),
            api_base,
        })
    }

    async fn access_token(&self) -> Result<AccessTokenState, AuthError> {
        if let Some(token) = self.read_access_token()? {
            return Ok(AccessTokenState {
                token,
                from_cache: true,
            });
        }

        self.reauthenticate_access_token()
            .await
            .map(|token| AccessTokenState {
                token,
                from_cache: false,
            })
    }

    async fn login_device_flow(&self) -> Result<String, AuthError> {
        let client = reqwest::Client::new();
        let body = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", GITHUB_CLIENT_ID)
            .append_pair("scope", "read:user")
            .finish();

        let device = client
            .post(GITHUB_DEVICE_CODE_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(body)
            .send()
            .await?
            .error_for_status()?
            .json::<DeviceCodeResponse>()
            .await?;

        emit_device_code_prompt(
            self.device_code_handler.0.as_ref(),
            DeviceCodePrompt {
                verification_uri: device.verification_uri.clone(),
                user_code: device.user_code.clone(),
            },
            &format!(
                "Sign in with GitHub Copilot:\n1) Visit {}\n2) Enter code: {}",
                device.verification_uri, device.user_code
            ),
        );

        let deadline = std::time::Instant::now()
            + std::time::Duration::from_secs(
                device.expires_in.unwrap_or(DEVICE_CODE_TIMEOUT_SECONDS),
            );
        let mut interval = normalize_poll_interval_seconds(device.interval);

        while std::time::Instant::now() < deadline {
            let body = url::form_urlencoded::Serializer::new(String::new())
                .append_pair("client_id", GITHUB_CLIENT_ID)
                .append_pair("device_code", &device.device_code)
                .append_pair("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
                .finish();

            let response = client
                .post(GITHUB_ACCESS_TOKEN_URL)
                .header(reqwest::header::ACCEPT, "application/json")
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(body)
                .send()
                .await?
                .error_for_status()?
                .json::<AccessTokenResponse>()
                .await?;

            if let Some(access_token) = response.access_token {
                return Ok(access_token);
            }

            interval = next_poll_interval_seconds(
                interval,
                response.error.as_deref(),
                response.error_description.as_deref(),
            )?;
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
        }

        Err(AuthError::Message(
            "Timed out waiting for GitHub Copilot device authorization".into(),
        ))
    }

    async fn refresh_api_key_at(
        &self,
        access_token: &str,
        endpoint: &str,
    ) -> Result<ApiKeyRecord, AuthError> {
        let client = reqwest::Client::new();
        let response = client
            .get(endpoint)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("editor-version", super::super::EDITOR_VERSION)
            .header("editor-plugin-version", super::super::EDITOR_PLUGIN_VERSION)
            .header("user-agent", super::super::USER_AGENT)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("token {access_token}"),
            )
            .send()
            .await?
            .error_for_status()?
            .json::<ApiKeyRecord>()
            .await?;

        if response.token.is_none() {
            return Err(AuthError::Message(
                "GitHub Copilot API key response did not include a token".into(),
            ));
        }

        Ok(response)
    }

    fn read_access_token(&self) -> Result<Option<String>, AuthError> {
        let Some(path) = &self.access_token_file else {
            return Ok(None);
        };

        match std::fs::read_to_string(path) {
            Ok(token) => {
                let token = token.trim();
                if token.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(token.to_owned()))
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn begin_access_token_login(&self) -> Result<Option<u64>, AuthError> {
        let mut generations = AUTH_CACHE_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        // A newer interactive login owns the eventual write, including when
        // its predecessor completes before the new device flow finishes.
        self.access_token_file
            .as_ref()
            .map(|path| advance_cache_generation(&mut generations, path))
            .transpose()
            .map_err(Into::into)
    }

    #[cfg(test)]
    fn write_access_token(&self, token: &str) -> Result<(), AuthError> {
        self.write_access_token_if_current(token, self.begin_access_token_login()?)
    }

    fn write_access_token_if_current(
        &self,
        token: &str,
        expected_generation: Option<u64>,
    ) -> Result<(), AuthError> {
        let generations = AUTH_CACHE_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let Some(path) = &self.access_token_file else {
            return Ok(());
        };
        if Some(generations.get(path).copied().unwrap_or_default()) != expected_generation {
            return Err(AuthError::Message(
                "GitHub Copilot credentials changed during sign-in. Try again.".into(),
            ));
        }

        ensure_parent_dir(path)?;
        std::fs::write(path, token.as_bytes())?;
        Ok(())
    }

    fn write_record_if_current(
        &self,
        bootstrap: &str,
        record: &ApiKeyRecord,
        expected: Option<&ApiKeyRecord>,
    ) -> Result<(), AuthError> {
        let _guard = AUTH_CACHE_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if self.read_access_token()?.as_deref() != Some(bootstrap) {
            return Err(AuthError::Message(
                "GitHub Copilot credentials changed during refresh. Try again.".into(),
            ));
        }
        if let Some(expected) = expected {
            let current: ApiKeyRecord = read_json_record(self.api_key_file.as_deref())?;
            if &current != expected {
                return Err(AuthError::Message(
                    "GitHub Copilot credentials changed during refresh. Try again.".into(),
                ));
            }
        }
        write_json_record(self.api_key_file.as_deref(), record)
    }

    fn retain_terminal_rejection(
        &self,
        bootstrap: &str,
        expected: &ApiKeyRecord,
    ) -> Result<(), AuthError> {
        self.write_record_if_current(
            bootstrap,
            &ApiKeyRecord {
                reauth_required: true,
                bootstrap_token_fingerprint: Some(bootstrap_token_fingerprint(bootstrap)),
                ..ApiKeyRecord::default()
            },
            Some(expected),
        )
    }

    async fn reauthenticate_access_token(&self) -> Result<String, AuthError> {
        self.reauthenticate_access_token_with(self.login_device_flow())
            .await
    }

    async fn reauthenticate_access_token_with(
        &self,
        login: impl std::future::Future<Output = Result<String, AuthError>>,
    ) -> Result<String, AuthError> {
        if !self.allow_device_flow {
            return Err(AuthError::Message(
                "GitHub Copilot sign-in required. Reconnect Copilot in Settings before using this provider."
                    .into(),
            ));
        }
        let generation = self.begin_access_token_login()?;
        let token = login.await?;
        self.write_access_token_if_current(&token, generation)?;
        Ok(token)
    }
}

impl ApiKeyRecord {
    fn api_base(&self) -> Option<String> {
        self.endpoints
            .as_ref()
            .and_then(|endpoints| endpoints.api.as_ref())
            .cloned()
    }

    fn can_reuse_for_oauth(&self, bootstrap_token: Option<&str>) -> bool {
        if !self.has_live_api_key() {
            return false;
        }

        match bootstrap_token {
            Some(bootstrap_token) => self.matches_bootstrap_token(bootstrap_token),
            // Legacy standalone API keys remain usable; a bootstrap-bound key
            // cannot outlive the login that owns it.
            None => self.bootstrap_token_fingerprint.is_none(),
        }
    }

    fn can_reuse_for_bootstrap_token(&self, bootstrap_token: &str) -> bool {
        self.has_live_api_key() && self.matches_bootstrap_token(bootstrap_token)
    }

    fn bind_to_bootstrap_token(mut self, bootstrap_token: &str) -> Self {
        self.bootstrap_token_fingerprint = Some(bootstrap_token_fingerprint(bootstrap_token));
        self
    }

    fn has_live_api_key(&self) -> bool {
        !self.reauth_required
            && self
                .token
                .as_ref()
                .is_some_and(|token| !token.trim().is_empty())
            && !token_expired(self.expires_at, 0)
    }

    fn matches_bootstrap_token(&self, bootstrap_token: &str) -> bool {
        self.bootstrap_token_fingerprint.as_deref()
            == Some(bootstrap_token_fingerprint(bootstrap_token).as_str())
    }
}

fn bootstrap_token_fingerprint(bootstrap_token: &str) -> String {
    format!("{:x}", Sha256::digest(bootstrap_token.as_bytes()))
}

fn normalize_poll_interval_seconds(interval: Option<u64>) -> u64 {
    interval.unwrap_or(DEVICE_CODE_POLL_SLEEP_SECONDS).max(1)
}

fn next_poll_interval_seconds(
    current_interval: u64,
    error: Option<&str>,
    error_description: Option<&str>,
) -> Result<u64, AuthError> {
    match error {
        Some("authorization_pending") => Ok(current_interval),
        Some("slow_down") => Ok(current_interval.saturating_add(DEVICE_CODE_SLOW_DOWN_SECONDS)),
        Some("expired_token") => Err(AuthError::Message(
            "GitHub device authorization expired before it completed".into(),
        )),
        Some("access_denied") => Err(AuthError::Message(
            "GitHub device authorization was denied".into(),
        )),
        Some(other) => Err(AuthError::Message(format_oauth_error(
            "GitHub device authorization failed",
            other,
            error_description,
        ))),
        None => Err(AuthError::Message(
            "GitHub device authorization failed: unknown error".into(),
        )),
    }
}

fn format_oauth_error(prefix: &str, error: &str, description: Option<&str>) -> String {
    match description
        .map(str::trim)
        .filter(|description| !description.is_empty())
    {
        Some(description) => format!("{prefix}: {error} ({description})"),
        None => format!("{prefix}: {error}"),
    }
}

fn should_retry_with_fresh_access_token(err: &AuthError) -> bool {
    match err {
        AuthError::Http(err) => should_retry_with_fresh_access_token_status(err.status()),
        _ => false,
    }
}

fn should_retry_with_fresh_access_token_status(status: Option<reqwest::StatusCode>) -> bool {
    matches!(
        status,
        Some(reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ApiKeyRecord, DeviceCodeHandler, PlatformAuthenticator, bootstrap_token_fingerprint,
        next_poll_interval_seconds, normalize_poll_interval_seconds,
        should_retry_with_fresh_access_token_status,
    };
    use reqwest::StatusCode;

    async fn refresh_reply(
        reply: Vec<u8>,
    ) -> (
        String,
        tokio::sync::oneshot::Receiver<()>,
        tokio::sync::oneshot::Sender<()>,
    ) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/token", listener.local_addr().unwrap());
        let (seen_tx, seen_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                stream.read_exact(&mut byte).await.unwrap();
                request.push(byte[0]);
            }
            seen_tx.send(()).unwrap();
            release_rx.await.unwrap();
            stream.write_all(&reply).await.unwrap();
        });
        (url, seen_rx, release_tx)
    }

    async fn rejected_refresh() -> (
        String,
        tokio::sync::oneshot::Receiver<()>,
        tokio::sync::oneshot::Sender<()>,
    ) {
        refresh_reply(
            b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_vec(),
        )
        .await
    }

    fn isolated_auth(dir: &assert_fs::TempDir) -> PlatformAuthenticator {
        let auth = PlatformAuthenticator::new(
            Some(dir.path().join("access-token")),
            Some(dir.path().join("api-key.json")),
            DeviceCodeHandler::default(),
            false,
        );
        auth.write_access_token("synthetic-bootstrap-a").unwrap();
        auth
    }

    fn isolated_device_auth(dir: &assert_fs::TempDir) -> PlatformAuthenticator {
        PlatformAuthenticator::new(
            Some(dir.path().join("access-token")),
            Some(dir.path().join("api-key.json")),
            DeviceCodeHandler::default(),
            true,
        )
    }

    fn pending_device_login(
        auth: &PlatformAuthenticator,
    ) -> (
        tokio::task::JoinHandle<Result<String, super::AuthError>>,
        tokio::sync::oneshot::Receiver<()>,
        tokio::sync::oneshot::Sender<String>,
    ) {
        let (seen_tx, seen_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = tokio::sync::oneshot::channel();
        let auth = auth.clone();
        let task = tokio::spawn(async move {
            auth.reauthenticate_access_token_with(async move {
                // This future is first polled after the production owner has
                // claimed its cache generation, before any token is returned.
                seen_tx.send(()).unwrap();
                Ok(release_rx.await.unwrap())
            })
            .await
        });
        (task, seen_rx, release_tx)
    }

    #[tokio::test]
    async fn copilot_disconnect_revokes_pending_device_login_with_empty_cache() {
        let dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_device_auth(&dir);
        let (task, seen, release) = pending_device_login(&auth);
        seen.await.unwrap();

        assert_eq!(
            super::clear_oauth_cache(
                auth.access_token_file.as_ref().unwrap(),
                auth.api_key_file.as_ref().unwrap(),
            )
            .unwrap(),
            0
        );
        release.send("synthetic-revoked-bootstrap".into()).unwrap();
        assert!(
            task.await
                .unwrap()
                .unwrap_err()
                .to_string()
                .contains("credentials changed")
        );
        assert!(!auth.access_token_file.as_ref().unwrap().exists());
        assert!(!auth.api_key_file.as_ref().unwrap().exists());
    }

    #[tokio::test]
    async fn copilot_newest_device_login_owns_commit_without_disconnect() {
        let dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_device_auth(&dir);
        let (old_task, old_seen, old_release) = pending_device_login(&auth);
        old_seen.await.unwrap();
        let (new_task, new_seen, new_release) = pending_device_login(&auth);
        new_seen.await.unwrap();

        new_release.send("synthetic-new-bootstrap".into()).unwrap();
        assert_eq!(new_task.await.unwrap().unwrap(), "synthetic-new-bootstrap");
        old_release.send("synthetic-old-bootstrap".into()).unwrap();
        assert!(
            old_task
                .await
                .unwrap()
                .unwrap_err()
                .to_string()
                .contains("credentials changed")
        );
        assert_eq!(
            auth.read_access_token().unwrap().as_deref(),
            Some("synthetic-new-bootstrap")
        );
    }

    #[tokio::test]
    async fn copilot_new_login_claim_revokes_old_completion_before_new_commit() {
        let dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_device_auth(&dir);
        let (old_task, old_seen, old_release) = pending_device_login(&auth);
        old_seen.await.unwrap();
        let (new_task, new_seen, new_release) = pending_device_login(&auth);
        new_seen.await.unwrap();

        old_release.send("synthetic-old-bootstrap".into()).unwrap();
        assert!(old_task.await.unwrap().is_err());
        assert!(auth.read_access_token().unwrap().is_none());
        new_release.send("synthetic-new-bootstrap".into()).unwrap();
        assert_eq!(new_task.await.unwrap().unwrap(), "synthetic-new-bootstrap");
    }

    #[tokio::test]
    async fn copilot_refresh_snapshot_allows_pending_login_but_rejects_fresh_commit() {
        for explicit in [false, true] {
            for refresh_finishes_first in [false, true] {
                let dir = assert_fs::TempDir::new().unwrap();
                let auth = isolated_device_auth(&dir);
                auth.write_access_token("synthetic-bootstrap-a").unwrap();
                let body =
                    r#"{"token":"synthetic-refreshed-key","expires_at":9223372036854775807}"#;
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let (url, seen, release) = refresh_reply(reply.into_bytes()).await;
                let refresh = tokio::spawn({
                    let auth = auth.clone();
                    async move {
                        if explicit {
                            auth.auth_context_with_github_access_token_at(
                                "synthetic-bootstrap-a",
                                &url,
                            )
                            .await
                        } else {
                            auth.auth_context_oauth_at(&url).await
                        }
                    }
                });
                seen.await.unwrap();
                let (login, login_seen, login_release) = pending_device_login(&auth);
                login_seen.await.unwrap();

                if refresh_finishes_first {
                    release.send(()).unwrap();
                    assert_eq!(
                        refresh.await.unwrap().unwrap().api_key,
                        "synthetic-refreshed-key"
                    );
                    login_release.send("synthetic-bootstrap-b".into()).unwrap();
                    assert_eq!(login.await.unwrap().unwrap(), "synthetic-bootstrap-b");
                } else {
                    login_release.send("synthetic-bootstrap-b".into()).unwrap();
                    assert_eq!(login.await.unwrap().unwrap(), "synthetic-bootstrap-b");
                    release.send(()).unwrap();
                    assert!(
                        refresh
                            .await
                            .unwrap()
                            .unwrap_err()
                            .to_string()
                            .contains("credentials changed")
                    );
                    assert!(!auth.api_key_file.as_ref().unwrap().exists());
                }
                assert_eq!(
                    auth.read_access_token().unwrap().as_deref(),
                    Some("synthetic-bootstrap-b")
                );
            }
        }
    }

    #[tokio::test]
    async fn copilot_reconnect_after_disconnect_preserves_new_credentials() {
        let dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_device_auth(&dir);
        let (old_task, seen, release) = pending_device_login(&auth);
        seen.await.unwrap();
        super::clear_oauth_cache(
            auth.access_token_file.as_ref().unwrap(),
            auth.api_key_file.as_ref().unwrap(),
        )
        .unwrap();

        auth.reauthenticate_access_token_with(async { Ok("synthetic-new-bootstrap".into()) })
            .await
            .unwrap();
        let fresh = ApiKeyRecord {
            token: Some("synthetic-new-key".into()),
            expires_at: Some(i64::MAX),
            ..ApiKeyRecord::default()
        }
        .bind_to_bootstrap_token("synthetic-new-bootstrap");
        auth.write_record_if_current("synthetic-new-bootstrap", &fresh, None)
            .unwrap();

        release.send("synthetic-old-bootstrap".into()).unwrap();
        assert!(old_task.await.unwrap().is_err());
        assert_eq!(
            auth.read_access_token().unwrap().as_deref(),
            Some("synthetic-new-bootstrap")
        );
        let record: ApiKeyRecord = super::read_json_record(auth.api_key_file.as_deref()).unwrap();
        assert_eq!(record, fresh);
    }

    #[tokio::test]
    async fn copilot_device_login_generation_is_scoped_to_its_access_cache() {
        let dir = assert_fs::TempDir::new().unwrap();
        let other_dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_device_auth(&dir);
        let other = isolated_device_auth(&other_dir);
        let (task, seen, release) = pending_device_login(&auth);
        seen.await.unwrap();

        super::clear_oauth_cache(
            other.access_token_file.as_ref().unwrap(),
            other.api_key_file.as_ref().unwrap(),
        )
        .unwrap();
        other
            .reauthenticate_access_token_with(async { Ok("synthetic-other-bootstrap".into()) })
            .await
            .unwrap();
        release.send("synthetic-bootstrap".into()).unwrap();
        assert_eq!(task.await.unwrap().unwrap(), "synthetic-bootstrap");
        assert_eq!(
            auth.read_access_token().unwrap().as_deref(),
            Some("synthetic-bootstrap")
        );
        assert_eq!(
            other.read_access_token().unwrap().as_deref(),
            Some("synthetic-other-bootstrap")
        );
    }

    #[tokio::test]
    async fn copilot_failed_cache_clear_still_revokes_pending_device_login() {
        let dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_device_auth(&dir);
        let (task, seen, release) = pending_device_login(&auth);
        seen.await.unwrap();
        // remove_file fails for a directory on every supported native platform.
        std::fs::create_dir(auth.api_key_file.as_ref().unwrap()).unwrap();
        assert!(
            super::clear_oauth_cache(
                auth.access_token_file.as_ref().unwrap(),
                auth.api_key_file.as_ref().unwrap(),
            )
            .is_err()
        );

        release.send("synthetic-revoked-bootstrap".into()).unwrap();
        assert!(task.await.unwrap().is_err());
        assert!(!auth.access_token_file.as_ref().unwrap().exists());
        assert!(auth.api_key_file.as_ref().unwrap().is_dir());
    }

    #[tokio::test]
    async fn copilot_terminal_refresh_retains_evidence_and_a_fresh_login_recovers() {
        let dir = assert_fs::TempDir::new().unwrap();
        let auth = isolated_auth(&dir);
        let (url, seen, release) = rejected_refresh().await;
        let task = tokio::spawn({
            let auth = auth.clone();
            async move { auth.auth_context_oauth_at(&url).await }
        });
        seen.await.unwrap();
        release.send(()).unwrap();
        assert!(
            task.await
                .unwrap()
                .unwrap_err()
                .to_string()
                .contains("sign-in required")
        );
        assert_eq!(
            auth.read_access_token().unwrap().as_deref(),
            Some("synthetic-bootstrap-a")
        );
        let record: ApiKeyRecord = super::read_json_record(auth.api_key_file.as_deref()).unwrap();
        assert!(record.reauth_required);
        assert!(record.matches_bootstrap_token("synthetic-bootstrap-a"));
        let error = auth
            .auth_context_oauth_at("http://127.0.0.1:1/unused")
            .await
            .unwrap_err();
        assert!(error.to_string().contains("sign-in required"));
        auth.write_access_token("synthetic-bootstrap-b").unwrap();
        let fresh = ApiKeyRecord {
            token: Some("synthetic-api-key-b".into()),
            expires_at: Some(i64::MAX),
            ..ApiKeyRecord::default()
        }
        .bind_to_bootstrap_token("synthetic-bootstrap-b");
        auth.write_record_if_current("synthetic-bootstrap-b", &fresh, None)
            .unwrap();
        assert_eq!(
            auth.auth_context_oauth_at("http://127.0.0.1:1/unused")
                .await
                .unwrap()
                .api_key,
            "synthetic-api-key-b"
        );
    }

    #[tokio::test]
    async fn copilot_late_rejection_preserves_new_login_or_disconnect() {
        for change in ["bootstrap", "api-key", "disconnect"] {
            let dir = assert_fs::TempDir::new().unwrap();
            let auth = isolated_auth(&dir);
            let (url, seen, release) = rejected_refresh().await;
            let task = tokio::spawn({
                let auth = auth.clone();
                async move { auth.auth_context_oauth_at(&url).await }
            });
            seen.await.unwrap();
            match change {
                "bootstrap" => auth.write_access_token("synthetic-bootstrap-b").unwrap(),
                "api-key" => {
                    let fresh = ApiKeyRecord {
                        token: Some("synthetic-new-api-key".into()),
                        expires_at: Some(i64::MAX),
                        ..ApiKeyRecord::default()
                    }
                    .bind_to_bootstrap_token("synthetic-bootstrap-a");
                    auth.write_record_if_current("synthetic-bootstrap-a", &fresh, None)
                        .unwrap();
                }
                _ => {
                    super::clear_oauth_cache(
                        auth.access_token_file.as_ref().unwrap(),
                        auth.api_key_file.as_ref().unwrap(),
                    )
                    .unwrap();
                }
            }
            release.send(()).unwrap();
            assert!(
                task.await
                    .unwrap()
                    .unwrap_err()
                    .to_string()
                    .contains("credentials changed")
            );
            let record: ApiKeyRecord =
                super::read_json_record(auth.api_key_file.as_deref()).unwrap();
            assert!(!record.reauth_required);
            if change == "bootstrap" {
                assert_eq!(
                    auth.read_access_token().unwrap().as_deref(),
                    Some("synthetic-bootstrap-b")
                );
            }
            if change == "api-key" {
                assert_eq!(record.token.as_deref(), Some("synthetic-new-api-key"));
            }
            if change == "disconnect" {
                assert!(auth.read_access_token().unwrap().is_none());
            }
        }
    }

    #[test]
    fn api_key_record_parses_dynamic_api_base() {
        let record: ApiKeyRecord = serde_json::from_str(
            r#"{
                "token": "copilot-token",
                "expires_at": 1775791135,
                "endpoints": {
                    "api": "https://api.individual.githubcopilot.com"
                }
            }"#,
        )
        .expect("parse api key record");

        assert_eq!(
            record.api_base().as_deref(),
            Some("https://api.individual.githubcopilot.com")
        );
    }

    #[tokio::test]
    async fn copilot_late_success_preserves_newer_terminal_key_or_disconnect() {
        for explicit in [false, true] {
            for change in ["terminal", "key", "disconnect"] {
                let dir = assert_fs::TempDir::new().unwrap();
                let auth = isolated_auth(&dir);
                let body = r#"{"token":"synthetic-late-key","expires_at":9223372036854775807}"#;
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let (url, seen, release) = refresh_reply(reply.into_bytes()).await;
                let task = tokio::spawn({
                    let auth = auth.clone();
                    async move {
                        if explicit {
                            auth.auth_context_with_github_access_token_at(
                                "synthetic-bootstrap-a",
                                &url,
                            )
                            .await
                        } else {
                            auth.auth_context_oauth_at(&url).await
                        }
                    }
                });
                seen.await.unwrap();
                match change {
                    "terminal" => auth
                        .retain_terminal_rejection(
                            "synthetic-bootstrap-a",
                            &ApiKeyRecord::default(),
                        )
                        .unwrap(),
                    "key" => {
                        let fresh = ApiKeyRecord {
                            token: Some("synthetic-new-key".into()),
                            expires_at: Some(i64::MAX),
                            ..ApiKeyRecord::default()
                        }
                        .bind_to_bootstrap_token("synthetic-bootstrap-a");
                        auth.write_record_if_current("synthetic-bootstrap-a", &fresh, None)
                            .unwrap();
                    }
                    _ => {
                        super::clear_oauth_cache(
                            auth.access_token_file.as_ref().unwrap(),
                            auth.api_key_file.as_ref().unwrap(),
                        )
                        .unwrap();
                    }
                }
                release.send(()).unwrap();
                assert!(
                    task.await
                        .unwrap()
                        .unwrap_err()
                        .to_string()
                        .contains("credentials changed")
                );
                let record: ApiKeyRecord =
                    super::read_json_record(auth.api_key_file.as_deref()).unwrap();
                match change {
                    "terminal" => assert!(record.reauth_required),
                    "key" => assert_eq!(record.token.as_deref(), Some("synthetic-new-key")),
                    _ => {
                        assert!(auth.read_access_token().unwrap().is_none());
                        assert!(!auth.api_key_file.as_ref().unwrap().exists());
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn noninteractive_oauth_requires_sign_in_instead_of_device_flow() {
        let auth = PlatformAuthenticator::new(None, None, DeviceCodeHandler::default(), false);
        let err = auth
            .auth_context_oauth()
            .await
            .expect_err("missing cached auth should not start device flow")
            .to_string();

        assert!(err.contains("GitHub Copilot sign-in required"), "{err}");
    }

    #[test]
    fn api_key_record_reuse_requires_matching_bootstrap_token_for_explicit_auth() {
        let record = ApiKeyRecord {
            token: Some("copilot-token".into()),
            expires_at: Some(i64::MAX),
            endpoints: None,
            bootstrap_token_fingerprint: Some(bootstrap_token_fingerprint("github-token-a")),
            reauth_required: false,
        };

        assert!(record.can_reuse_for_bootstrap_token("github-token-a"));
        assert!(!record.can_reuse_for_bootstrap_token("github-token-b"));
    }

    #[test]
    fn api_key_record_oauth_reuse_requires_match_when_bootstrap_token_is_available() {
        let record = ApiKeyRecord {
            token: Some("copilot-token".into()),
            expires_at: Some(i64::MAX),
            endpoints: None,
            bootstrap_token_fingerprint: Some(bootstrap_token_fingerprint("github-token-a")),
            reauth_required: false,
        };

        assert!(record.can_reuse_for_oauth(Some("github-token-a")));
        assert!(!record.can_reuse_for_oauth(Some("github-token-b")));
        assert!(!record.can_reuse_for_oauth(None));
    }

    #[test]
    fn legacy_api_key_record_without_fingerprint_forces_refresh_when_bootstrap_token_is_known() {
        let record = ApiKeyRecord {
            token: Some("copilot-token".into()),
            expires_at: Some(i64::MAX),
            endpoints: None,
            bootstrap_token_fingerprint: None,
            reauth_required: false,
        };

        assert!(!record.can_reuse_for_bootstrap_token("github-token-a"));
        assert!(!record.can_reuse_for_oauth(Some("github-token-a")));
        assert!(record.can_reuse_for_oauth(None));
    }

    #[test]
    fn poll_interval_defaults_and_clamps() {
        assert_eq!(normalize_poll_interval_seconds(None), 5);
        assert_eq!(normalize_poll_interval_seconds(Some(0)), 1);
        assert_eq!(normalize_poll_interval_seconds(Some(9)), 9);
    }

    #[test]
    fn poll_interval_handles_pending_and_slow_down() {
        assert_eq!(
            next_poll_interval_seconds(5, Some("authorization_pending"), None)
                .expect("authorization pending interval"),
            5
        );
        assert_eq!(
            next_poll_interval_seconds(5, Some("slow_down"), None).expect("slow_down interval"),
            10
        );
    }

    #[test]
    fn poll_interval_rejects_terminal_errors() {
        let denied = next_poll_interval_seconds(5, Some("access_denied"), None)
            .expect_err("access denied should fail");
        assert_eq!(denied.to_string(), "GitHub device authorization was denied");

        let unknown = next_poll_interval_seconds(
            5,
            Some("device_flow_disabled"),
            Some("OAuth app device flow is disabled"),
        )
        .expect_err("device flow disabled should fail");
        assert_eq!(
            unknown.to_string(),
            "GitHub device authorization failed: device_flow_disabled (OAuth app device flow is disabled)"
        );
    }

    #[test]
    fn stale_access_token_retries_only_on_auth_failures() {
        assert!(should_retry_with_fresh_access_token_status(Some(
            StatusCode::UNAUTHORIZED
        )));
        assert!(should_retry_with_fresh_access_token_status(Some(
            StatusCode::FORBIDDEN
        )));
        assert!(!should_retry_with_fresh_access_token_status(Some(
            StatusCode::BAD_GATEWAY
        )));
        assert!(!should_retry_with_fresh_access_token_status(None));
    }
}
