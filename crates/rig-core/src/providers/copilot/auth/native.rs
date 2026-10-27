use super::{AuthContext, AuthError, DeviceCodeHandler, DeviceCodePrompt};
use crate::providers::internal::device_auth::{
    emit_device_code_prompt, ensure_parent_dir, read_json_record, token_expired, write_json_record,
};
use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;

const GITHUB_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";
const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_API_KEY_URL: &str = "https://api.github.com/copilot_internal/v2/token";
const DEVICE_CODE_POLL_SLEEP_SECONDS: u64 = 5;
const DEVICE_CODE_TIMEOUT_SECONDS: u64 = 15 * 60;
const DEVICE_CODE_SLOW_DOWN_SECONDS: u64 = 5;

#[derive(Debug, Clone)]
pub(super) struct PlatformAuthenticator {
    access_token_file: Option<PathBuf>,
    api_key_file: Option<PathBuf>,
    device_code_handler: DeviceCodeHandler,
    allow_device_flow: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct ApiKeyRecord {
    token: Option<String>,
    expires_at: Option<i64>,
    endpoints: Option<ApiKeyEndpoints>,
    bootstrap_token_fingerprint: Option<String>,
    #[serde(default)]
    reauthentication_required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
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

fn cache_revisions() -> &'static std::sync::Mutex<std::collections::HashMap<PathBuf, u64>> {
    static REVISIONS: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<PathBuf, u64>>,
    > = std::sync::OnceLock::new();
    REVISIONS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn cache_lock()
-> Result<std::sync::MutexGuard<'static, std::collections::HashMap<PathBuf, u64>>, AuthError> {
    cache_revisions()
        .lock()
        .map_err(|_| AuthError::Message("Copilot credential writer lock is poisoned".into()))
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
        let mut revision = self.cache_revision()?;
        let record: ApiKeyRecord = read_json_record(self.api_key_file.as_deref())?;
        let cached_access_token = self.read_access_token().ok().flatten();
        if record.reauthentication_required
            && !self.allow_device_flow
            && cached_access_token
                .as_deref()
                .is_some_and(|token| record.matches_bootstrap_token(token))
        {
            return Err(AuthError::Message(
                "GitHub Copilot sign-in required. Reconnect Copilot in Settings.".into(),
            ));
        }
        let api_base = record.api_base();
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
            self.access_token(revision).await?
        };
        if !self.allow_device_flow {
            return self.complete_cached_exchange(
                self.refresh_api_key(&access_token.token).await,
                record,
                &access_token.token,
                revision,
            );
        }
        let record = match self.refresh_api_key(&access_token.token).await {
            Ok(record) => record.bind_to_bootstrap_token(&access_token.token),
            Err(err) if access_token.from_cache && should_retry_with_fresh_access_token(&err) => {
                revision = self.clear_access_token()?;
                let fresh_access_token = self.reauthenticate_access_token(revision).await?;
                self.refresh_api_key(&fresh_access_token)
                    .await?
                    .bind_to_bootstrap_token(&fresh_access_token)
            }
            Err(err) => return Err(err),
        };
        let api_base = record.api_base();
        self.publish_api_key(&record, revision, true)?;
        Ok(AuthContext {
            api_key: record.token.unwrap_or_default(),
            api_base,
        })
    }

    pub(super) async fn auth_context_with_github_access_token(
        &self,
        access_token: &str,
    ) -> Result<AuthContext, AuthError> {
        let revision = self.cache_revision()?;
        let record: ApiKeyRecord = read_json_record(self.api_key_file.as_deref())?;
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
            .refresh_api_key(access_token)
            .await?
            .bind_to_bootstrap_token(access_token);
        let api_base = record.api_base();
        self.publish_api_key(&record, revision, false)?;
        Ok(AuthContext {
            api_key: record.token.unwrap_or_default(),
            api_base,
        })
    }

    async fn access_token(&self, revision: u64) -> Result<AccessTokenState, AuthError> {
        if let Some(token) = self.read_access_token()? {
            return Ok(AccessTokenState {
                token,
                from_cache: true,
            });
        }

        self.reauthenticate_access_token(revision)
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

    async fn refresh_api_key(&self, access_token: &str) -> Result<ApiKeyRecord, AuthError> {
        let client = reqwest::Client::new();
        let response = client
            .get(GITHUB_API_KEY_URL)
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

    fn complete_cached_exchange(
        &self,
        result: Result<ApiKeyRecord, AuthError>,
        previous: ApiKeyRecord,
        bootstrap: &str,
        revision: u64,
    ) -> Result<AuthContext, AuthError> {
        let record = match result {
            Ok(record) => record.bind_to_bootstrap_token(bootstrap),
            Err(error) => {
                if should_retry_with_fresh_access_token(&error) {
                    let mut rejected = previous.bind_to_bootstrap_token(bootstrap);
                    rejected.reauthentication_required = true;
                    self.publish_api_key(&rejected, revision, true)?;
                }
                return Err(error);
            }
        };
        let api_base = record.api_base();
        self.publish_api_key(&record, revision, true)?;
        Ok(AuthContext {
            api_key: record.token.unwrap_or_default(),
            api_base,
        })
    }

    fn cache_key(&self) -> PathBuf {
        self.access_token_file
            .clone()
            .or_else(|| self.api_key_file.clone())
            .unwrap_or_default()
    }

    fn cache_revision(&self) -> Result<u64, AuthError> {
        Ok(*cache_lock()?.entry(self.cache_key()).or_default())
    }

    fn publish_api_key(
        &self,
        record: &ApiKeyRecord,
        revision: u64,
        oauth: bool,
    ) -> Result<(), AuthError> {
        let mut revisions = cache_lock()?;
        if *revisions.entry(self.cache_key()).or_default() != revision
            || (oauth
                && !self
                    .read_access_token()?
                    .as_deref()
                    .is_some_and(|token| record.matches_bootstrap_token(token)))
        {
            return Err(AuthError::Message(
                "Copilot sign-in changed while authentication was in flight".into(),
            ));
        }
        write_json_record(self.api_key_file.as_deref(), record)
    }

    fn write_access_token(&self, token: &str, revision: u64) -> Result<(), AuthError> {
        let mut revisions = cache_lock()?;
        if *revisions.entry(self.cache_key()).or_default() != revision {
            return Err(AuthError::Message(
                "Copilot sign-in changed while authentication was in flight".into(),
            ));
        }

        let Some(path) = &self.access_token_file else {
            return Ok(());
        };

        ensure_parent_dir(path)?;
        std::fs::write(path, token.as_bytes())?;
        Ok(())
    }

    fn clear_access_token(&self) -> Result<u64, AuthError> {
        let mut revisions = cache_lock()?;
        let revision = revisions.entry(self.cache_key()).or_default();
        *revision += 1;
        if let Some(path) = &self.access_token_file {
            match std::fs::remove_file(path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(*revision)
    }

    async fn reauthenticate_access_token(&self, revision: u64) -> Result<String, AuthError> {
        if !self.allow_device_flow {
            return Err(AuthError::Message(
                "GitHub Copilot sign-in required. Reconnect Copilot in Settings before using this provider."
                    .into(),
            ));
        }
        let token = self.login_device_flow().await?;
        self.write_access_token(&token, revision)?;
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
            None => true,
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
        !self.reauthentication_required
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
    let mut hasher = DefaultHasher::new();
    bootstrap_token.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
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
    matches!(status, Some(reqwest::StatusCode::UNAUTHORIZED))
}

/// Read the terminal rejection of the current bootstrap credential without
/// refreshing tokens. A replacement credential retires the older verdict.
pub fn reauthentication_required(
    api_key_file: &std::path::Path,
    access_token_file: &std::path::Path,
) -> Result<bool, AuthError> {
    let _guard = cache_lock()?;
    let record: ApiKeyRecord = read_json_record(Some(api_key_file))?;
    if !record.reauthentication_required {
        return Ok(false);
    }
    let token = match std::fs::read_to_string(access_token_file) {
        Ok(token) => token,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    Ok(record.matches_bootstrap_token(token.trim()))
}

/// Retire in-flight credential writers before removing the shared OAuth cache.
pub fn clear_auth_cache(
    access_token_file: &std::path::Path,
    api_key_file: &std::path::Path,
) -> std::io::Result<usize> {
    let mut revisions = cache_revisions()
        .lock()
        .map_err(|_| std::io::Error::other("Copilot credential writer lock is poisoned"))?;
    *revisions
        .entry(access_token_file.to_path_buf())
        .or_default() += 1;
    let mut removed = 0;
    for path in [access_token_file, api_key_file] {
        match std::fs::remove_file(path) {
            Ok(()) => removed += 1,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(removed)
}

/// Read a live API credential only when its bootstrap binding is current.
/// This does not refresh, start device flow, or mutate either cache file.
pub fn cached_auth_context(
    api_key_file: &std::path::Path,
    access_token_file: &std::path::Path,
) -> Result<Option<AuthContext>, AuthError> {
    let _guard = cache_lock()?;
    let record: ApiKeyRecord = read_json_record(Some(api_key_file))?;
    let bootstrap = match std::fs::read_to_string(access_token_file) {
        Ok(value) => Some(value.trim().to_owned()).filter(|value| !value.is_empty()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    if !record.can_reuse_for_oauth(bootstrap.as_deref()) || token_expired(record.expires_at, 60) {
        return Ok(None);
    }
    let api_base = record.api_base();
    Ok(record
        .token
        .map(|api_key| AuthContext { api_key, api_base }))
}

#[cfg(test)]
mod tests {
    use super::{
        ApiKeyRecord, DeviceCodeHandler, PlatformAuthenticator, bootstrap_token_fingerprint,
        next_poll_interval_seconds, normalize_poll_interval_seconds,
        should_retry_with_fresh_access_token_status,
    };
    use reqwest::StatusCode;

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
            reauthentication_required: false,
            bootstrap_token_fingerprint: Some(bootstrap_token_fingerprint("github-token-a")),
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
            reauthentication_required: false,
            bootstrap_token_fingerprint: Some(bootstrap_token_fingerprint("github-token-a")),
        };

        assert!(record.can_reuse_for_oauth(Some("github-token-a")));
        assert!(!record.can_reuse_for_oauth(Some("github-token-b")));
        assert!(record.can_reuse_for_oauth(None));
    }

    #[test]
    fn legacy_api_key_record_without_fingerprint_forces_refresh_when_bootstrap_token_is_known() {
        let record = ApiKeyRecord {
            token: Some("copilot-token".into()),
            expires_at: Some(i64::MAX),
            endpoints: None,
            reauthentication_required: false,
            bootstrap_token_fingerprint: None,
        };

        assert!(!record.can_reuse_for_bootstrap_token("github-token-a"));
        assert!(!record.can_reuse_for_oauth(Some("github-token-a")));
        assert!(record.can_reuse_for_oauth(None));
    }

    #[test]
    fn terminal_marker_disables_api_key_reuse_and_preserves_credential_binding() {
        let mut record = ApiKeyRecord::default().bind_to_bootstrap_token("fixture-old");
        record.token = Some("fixture-live-key".into());
        record.expires_at = Some(i64::MAX);
        assert!(record.has_live_api_key());
        record.reauthentication_required = true;
        let encoded = serde_json::to_vec(&record).unwrap();
        let rejected: ApiKeyRecord = serde_json::from_slice(&encoded).unwrap();
        assert!(!rejected.has_live_api_key());
        assert!(rejected.matches_bootstrap_token("fixture-old"));
        assert!(!rejected.matches_bootstrap_token("fixture-new"));
        assert_eq!(rejected.token.as_deref(), Some("fixture-live-key"));
    }

    #[test]
    fn late_rejection_cannot_replace_new_login_or_resurrect_after_disconnect() {
        let root = assert_fs::TempDir::new().unwrap();
        let access = root.path().join("access-token");
        let key = root.path().join("api-key.json");
        let auth = PlatformAuthenticator::new(
            Some(access.clone()),
            Some(key.clone()),
            DeviceCodeHandler::default(),
            false,
        );
        let revision_a = auth.cache_revision().unwrap();
        auth.write_access_token("fixture-a", revision_a).unwrap();
        let mut rejected_a = ApiKeyRecord::default().bind_to_bootstrap_token("fixture-a");
        rejected_a.reauthentication_required = true;
        super::clear_auth_cache(&access, &key).unwrap();
        let revision_b = auth.cache_revision().unwrap();
        auth.write_access_token("fixture-b", revision_b).unwrap();
        let mut healthy_b = ApiKeyRecord::default().bind_to_bootstrap_token("fixture-b");
        healthy_b.token = Some("fixture-live-b".into());
        healthy_b.expires_at = Some(i64::MAX);
        auth.publish_api_key(&healthy_b, revision_b, true).unwrap();
        let before = std::fs::read(&key).unwrap();
        assert!(auth.publish_api_key(&rejected_a, revision_a, true).is_err());
        assert_eq!(std::fs::read(&key).unwrap(), before);
        assert!(!super::reauthentication_required(&key, &access).unwrap());
        assert_eq!(super::clear_auth_cache(&access, &key).unwrap(), 2);
        let revision_c = auth.cache_revision().unwrap();
        auth.write_access_token("fixture-a", revision_c).unwrap();
        let mut healthy_a = ApiKeyRecord::default().bind_to_bootstrap_token("fixture-a");
        healthy_a.token = Some("fixture-live-a".into());
        healthy_a.expires_at = Some(i64::MAX);
        auth.publish_api_key(&healthy_a, revision_c, true).unwrap();
        assert!(
            auth.publish_api_key(&rejected_a, revision_a, true).is_err(),
            "same-credential ABA must retain its revision fence"
        );
        assert!(!super::reauthentication_required(&key, &access).unwrap());
        assert_eq!(super::clear_auth_cache(&access, &key).unwrap(), 2);
        assert!(auth.publish_api_key(&healthy_b, revision_b, true).is_err());
        assert!(auth.write_access_token("fixture-b", revision_b).is_err());
        assert!(!access.exists());
        assert!(!key.exists());
    }

    #[tokio::test]
    async fn rejected_exchange_publishes_only_definitive_sign_in_failure() {
        use std::io::{Read, Write};
        let root = assert_fs::TempDir::new().unwrap();
        let access = root.path().join("access-token");
        let key = root.path().join("api-key.json");
        let auth = PlatformAuthenticator::new(
            Some(access.clone()),
            Some(key.clone()),
            DeviceCodeHandler::default(),
            false,
        );
        let revision = auth.cache_revision().unwrap();
        auth.write_access_token("fixture-current", revision)
            .unwrap();
        for status in [403, 502, 401] {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let responder = std::thread::spawn(move || {
                let (mut connection, _) = listener.accept().unwrap();
                let mut request = [0; 4096];
                let _ = connection.read(&mut request).unwrap();
                write!(
                    connection,
                    "HTTP/1.1 {status} Synthetic\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                )
                .unwrap();
            });
            let error = reqwest::Client::new()
                .get(format!("http://{address}/copilot_internal/v2/token"))
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap_err();
            responder.join().unwrap();
            assert!(
                auth.complete_cached_exchange(
                    Err(error.into()),
                    ApiKeyRecord::default(),
                    "fixture-current",
                    revision
                )
                .is_err()
            );
            assert_eq!(
                super::reauthentication_required(&key, &access).unwrap(),
                status == 401
            );
            assert_eq!(std::fs::read_to_string(&access).unwrap(), "fixture-current");
        }
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
        assert!(!should_retry_with_fresh_access_token_status(Some(
            StatusCode::FORBIDDEN
        )));
        assert!(!should_retry_with_fresh_access_token_status(Some(
            StatusCode::BAD_GATEWAY
        )));
        assert!(!should_retry_with_fresh_access_token_status(None));
    }
}
