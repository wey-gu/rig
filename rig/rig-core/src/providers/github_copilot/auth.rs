use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

const GITHUB_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";
const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_API_KEY_URL: &str = "https://api.github.com/copilot_internal/v2/token";
const DEVICE_CODE_POLL_SLEEP_SECONDS: u64 = 5;
const DEVICE_CODE_MAX_ATTEMPTS: usize = 180;

#[derive(Debug, Clone)]
pub struct DeviceCodePrompt {
    pub verification_uri: String,
    pub user_code: String,
}

#[derive(Clone, Default)]
pub struct DeviceCodeHandler(Option<Arc<dyn Fn(DeviceCodePrompt) + Send + Sync>>);

impl DeviceCodeHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(DeviceCodePrompt) + Send + Sync + 'static,
    {
        Self(Some(Arc::new(handler)))
    }

    fn emit(&self, prompt: DeviceCodePrompt) {
        if let Some(handler) = &self.0 {
            handler(prompt);
        } else {
            println!(
                "Sign in with GitHub Copilot:\n1) Visit {}\n2) Enter code: {}",
                prompt.verification_uri, prompt.user_code
            );
        }
    }
}

impl fmt::Debug for DeviceCodeHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_some() {
            f.write_str("DeviceCodeHandler(<callback>)")
        } else {
            f.write_str("DeviceCodeHandler(None)")
        }
    }
}

#[derive(Clone)]
pub enum AuthSource {
    ApiKey(String),
    OAuth,
}

impl fmt::Debug for AuthSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiKey(_) => f.write_str("ApiKey(<redacted>)"),
            Self::OAuth => f.write_str("OAuth"),
        }
    }
}

#[derive(Clone)]
pub struct Authenticator {
    source: AuthSource,
    access_token_file: Option<PathBuf>,
    api_key_file: Option<PathBuf>,
    device_code_handler: DeviceCodeHandler,
    state_lock: Arc<Mutex<()>>,
}

impl fmt::Debug for Authenticator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Authenticator")
            .field("source", &self.source)
            .field("access_token_file", &self.access_token_file)
            .field("api_key_file", &self.api_key_file)
            .field("device_code_handler", &self.device_code_handler)
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub api_key: String,
    pub api_base: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct ApiKeyRecord {
    token: Option<String>,
    expires_at: Option<i64>,
    endpoints: Option<ApiKeyEndpoints>,
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
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

impl Authenticator {
    pub fn new(
        source: AuthSource,
        access_token_file: Option<PathBuf>,
        api_key_file: Option<PathBuf>,
        device_code_handler: DeviceCodeHandler,
    ) -> Self {
        Self {
            source,
            access_token_file,
            api_key_file,
            device_code_handler,
            state_lock: Arc::new(Mutex::new(())),
        }
    }

    pub async fn auth_context(&self) -> Result<AuthContext, AuthError> {
        match &self.source {
            AuthSource::ApiKey(api_key) => Ok(AuthContext {
                api_key: api_key.clone(),
                api_base: None,
            }),
            AuthSource::OAuth => {
                let _guard = self.state_lock.lock().await;
                self.auth_context_locked().await
            }
        }
    }

    async fn auth_context_locked(&self) -> Result<AuthContext, AuthError> {
        #[cfg(target_family = "wasm")]
        {
            Err(AuthError::Message(
                "GitHub Copilot OAuth is not supported on wasm targets".into(),
            ))
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let record = self.read_api_key_record()?;
            let api_base = record.api_base();
            if let Some(token) = record.token
                && !token_expired(record.expires_at)
            {
                return Ok(AuthContext {
                    api_key: token,
                    api_base,
                });
            }

            let access_token = self.access_token().await?;
            let record = self.refresh_api_key(&access_token).await?;
            let api_base = record.api_base();
            self.write_api_key_record(&record)?;
            Ok(AuthContext {
                api_key: record.token.unwrap_or_default(),
                api_base,
            })
        }
    }

    #[cfg(not(target_family = "wasm"))]
    async fn access_token(&self) -> Result<String, AuthError> {
        if let Some(path) = &self.access_token_file {
            match std::fs::read_to_string(path) {
                Ok(token) if !token.trim().is_empty() => return Ok(token.trim().to_owned()),
                Ok(_) => {}
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                Err(err) => return Err(err.into()),
            }
        }

        let token = self.login_device_flow().await?;
        if let Some(path) = &self.access_token_file {
            ensure_parent_dir(path)?;
            std::fs::write(path, token.as_bytes())?;
        }
        Ok(token)
    }

    #[cfg(not(target_family = "wasm"))]
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

        self.device_code_handler.emit(DeviceCodePrompt {
            verification_uri: device.verification_uri.clone(),
            user_code: device.user_code.clone(),
        });

        for _ in 0..DEVICE_CODE_MAX_ATTEMPTS {
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

            if response.error.as_deref() != Some("authorization_pending") {
                return Err(AuthError::Message(format!(
                    "GitHub device authorization failed: {}",
                    response.error.unwrap_or_else(|| "unknown error".into())
                )));
            }

            tokio::time::sleep(std::time::Duration::from_secs(
                DEVICE_CODE_POLL_SLEEP_SECONDS,
            ))
            .await;
        }

        Err(AuthError::Message(
            "Timed out waiting for GitHub Copilot device authorization".into(),
        ))
    }

    #[cfg(not(target_family = "wasm"))]
    async fn refresh_api_key(&self, access_token: &str) -> Result<ApiKeyRecord, AuthError> {
        let client = reqwest::Client::new();
        let response = client
            .get(GITHUB_API_KEY_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("editor-version", "vscode/1.95.0")
            .header("editor-plugin-version", "copilot-chat/0.26.7")
            .header("user-agent", "GitHubCopilotChat/0.26.7")
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

    #[cfg(not(target_family = "wasm"))]
    fn read_api_key_record(&self) -> Result<ApiKeyRecord, AuthError> {
        let Some(path) = &self.api_key_file else {
            return Ok(ApiKeyRecord::default());
        };

        match std::fs::read(path) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(ApiKeyRecord::default()),
            Err(err) => Err(err.into()),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn write_api_key_record(&self, record: &ApiKeyRecord) -> Result<(), AuthError> {
        let Some(path) = &self.api_key_file else {
            return Ok(());
        };

        ensure_parent_dir(path)?;
        std::fs::write(path, serde_json::to_vec_pretty(record)?)?;
        Ok(())
    }
}

impl ApiKeyRecord {
    fn api_base(&self) -> Option<String> {
        self.endpoints
            .as_ref()
            .and_then(|endpoints| endpoints.api.as_ref())
            .cloned()
    }
}

#[cfg(not(target_family = "wasm"))]
fn ensure_parent_dir(path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn token_expired(expires_at: Option<i64>) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default();

    match expires_at {
        Some(exp) => now >= exp,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::ApiKeyRecord;

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
}
