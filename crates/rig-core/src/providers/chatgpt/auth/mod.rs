//! Shared ChatGPT authentication types and target-specific dispatch.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, Weak};
use tokio::sync::Mutex;

pub use crate::providers::internal::auth::{DeviceCodeHandler, DeviceCodePrompt};

#[cfg(not(target_family = "wasm"))]
mod native;
#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(not(target_family = "wasm"))]
use native as platform;
#[cfg(target_family = "wasm")]
use wasm as platform;

#[derive(Clone)]
pub enum AuthSource {
    AccessToken {
        access_token: String,
        account_id: Option<String>,
    },
    OAuth,
}

impl fmt::Debug for AuthSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccessToken { .. } => f.write_str("AccessToken(<redacted>)"),
            Self::OAuth => f.write_str("OAuth"),
        }
    }
}

#[derive(Clone)]
pub struct Authenticator {
    source: AuthSource,
    platform: platform::PlatformAuthenticator,
    state_lock: Arc<Mutex<()>>,
}

impl fmt::Debug for Authenticator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Authenticator")
            .field("source", &self.source)
            .field("platform", &self.platform)
            .finish()
    }
}

pub use crate::providers::internal::auth::AuthError;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub access_token: String,
    pub account_id: Option<String>,
}

impl Authenticator {
    pub fn new(
        source: AuthSource,
        auth_file: Option<PathBuf>,
        oauth_http_client: Option<reqwest::Client>,
        device_code_handler: DeviceCodeHandler,
        allow_device_flow: bool,
    ) -> Self {
        let state_lock = shared_state_lock(auth_file.as_deref());
        Self {
            source,
            platform: platform::PlatformAuthenticator::new(
                auth_file,
                oauth_http_client,
                device_code_handler,
                allow_device_flow,
            ),
            state_lock,
        }
    }

    pub async fn auth_context(&self) -> Result<AuthContext, AuthError> {
        match &self.source {
            AuthSource::AccessToken {
                access_token,
                account_id,
            } => Ok(AuthContext {
                access_token: access_token.clone(),
                account_id: account_id.clone(),
            }),
            AuthSource::OAuth => {
                let _guard = self.state_lock.lock().await;
                self.platform.auth_context_oauth().await
            }
        }
    }

    pub async fn refresh_after_rejection(
        &self,
        rejected: &AuthContext,
    ) -> Result<AuthContext, AuthError> {
        match &self.source {
            AuthSource::AccessToken { .. } => Err(AuthError::Message(
                "ChatGPT access token was rejected and cannot be refreshed".into(),
            )),
            AuthSource::OAuth => {
                let _guard = self.state_lock.lock().await;
                self.platform
                    .refresh_after_rejection(&rejected.access_token)
                    .await
            }
        }
    }
}

fn shared_state_lock(auth_file: Option<&Path>) -> Arc<Mutex<()>> {
    static LOCKS: OnceLock<StdMutex<HashMap<PathBuf, Weak<Mutex<()>>>>> = OnceLock::new();

    let Some(path) = auth_file else {
        return Arc::new(Mutex::new(()));
    };
    let locks = LOCKS.get_or_init(|| StdMutex::new(HashMap::new()));
    let mut locks = locks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(lock) = locks.get(path).and_then(Weak::upgrade) {
        return lock;
    }
    let lock = Arc::new(Mutex::new(()));
    locks.insert(path.to_path_buf(), Arc::downgrade(&lock));
    lock
}

#[cfg(test)]
mod tests {
    use super::shared_state_lock;
    use std::sync::Arc;

    #[test]
    fn clients_for_one_auth_file_share_the_refresh_lock() {
        let path = std::path::Path::new("/tmp/rig-chatgpt-shared-auth.json");
        let first = shared_state_lock(Some(path));
        let second = shared_state_lock(Some(path));
        assert!(Arc::ptr_eq(&first, &second));

        let other = shared_state_lock(Some(std::path::Path::new(
            "/tmp/rig-chatgpt-other-auth.json",
        )));
        assert!(!Arc::ptr_eq(&first, &other));
    }
}
