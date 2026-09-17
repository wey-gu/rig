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
    refresh_lock: Arc<Mutex<()>>,
    device_flow_lock: Arc<Mutex<()>>,
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
        let refresh_lock = shared_state_lock(auth_file.as_deref(), SharedLockKind::Refresh);
        let device_flow_lock = shared_state_lock(auth_file.as_deref(), SharedLockKind::DeviceFlow);
        Self {
            source,
            platform: platform::PlatformAuthenticator::new(
                auth_file,
                oauth_http_client,
                device_code_handler,
                allow_device_flow,
            ),
            refresh_lock,
            device_flow_lock,
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
                {
                    let _guard = self.refresh_lock.lock().await;
                    if let Some(context) = self.platform.cached_or_refreshed_context().await? {
                        return Ok(context);
                    }
                }

                if !self.platform.device_flow_allowed() {
                    return Err(sign_in_required());
                }

                // Only interactive clients wait for another device flow. A
                // long-running/background client uses `allow_device_flow(false)`
                // and therefore fails fast above instead of waiting up to the
                // 15-minute user authorization deadline.
                let _device_guard = self.device_flow_lock.lock().await;

                // An earlier interactive client may have completed while this
                // one waited. Recheck under the short refresh lock before
                // starting another device flow, but never hold that lock while
                // waiting for the user.
                {
                    let _guard = self.refresh_lock.lock().await;
                    if let Some(context) = self.platform.cached_or_refreshed_context().await? {
                        return Ok(context);
                    }
                }

                let fresh = self.platform.login_device_flow().await?;
                let _guard = self.refresh_lock.lock().await;
                self.platform.persist_device_flow(fresh)
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
                let _guard = self.refresh_lock.lock().await;
                self.platform
                    .refresh_after_rejection(&rejected.access_token)
                    .await
            }
        }
    }
}

fn sign_in_required() -> AuthError {
    AuthError::Message(
        "ChatGPT sign-in required. Reconnect ChatGPT in Settings before using this provider."
            .into(),
    )
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum SharedLockKind {
    Refresh,
    DeviceFlow,
}

type SharedLockRegistry = HashMap<(PathBuf, SharedLockKind), Weak<Mutex<()>>>;

fn shared_state_lock(auth_file: Option<&Path>, kind: SharedLockKind) -> Arc<Mutex<()>> {
    static LOCKS: OnceLock<StdMutex<SharedLockRegistry>> = OnceLock::new();

    let Some(path) = auth_file else {
        return Arc::new(Mutex::new(()));
    };
    let locks = LOCKS.get_or_init(|| StdMutex::new(HashMap::new()));
    let mut locks = locks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let key = (path.to_path_buf(), kind);
    if let Some(lock) = locks.get(&key).and_then(Weak::upgrade) {
        return lock;
    }
    let lock = Arc::new(Mutex::new(()));
    locks.insert(key, Arc::downgrade(&lock));
    lock
}

#[cfg(test)]
mod tests {
    use super::{AuthSource, Authenticator, DeviceCodeHandler, SharedLockKind, shared_state_lock};
    use std::sync::Arc;

    #[test]
    fn clients_for_one_auth_file_share_the_refresh_lock() {
        let path = std::path::Path::new("/tmp/rig-chatgpt-shared-auth.json");
        let first = shared_state_lock(Some(path), SharedLockKind::Refresh);
        let second = shared_state_lock(Some(path), SharedLockKind::Refresh);
        assert!(Arc::ptr_eq(&first, &second));

        let device_flow = shared_state_lock(Some(path), SharedLockKind::DeviceFlow);
        assert!(!Arc::ptr_eq(&first, &device_flow));

        let other = shared_state_lock(
            Some(std::path::Path::new("/tmp/rig-chatgpt-other-auth.json")),
            SharedLockKind::Refresh,
        );
        assert!(!Arc::ptr_eq(&first, &other));
    }

    #[tokio::test]
    async fn pending_interactive_device_flow_does_not_block_noninteractive_client() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let auth_file = temp.path().join("auth.json");
        std::fs::write(&auth_file, r#"{"reauth_required":true}"#)
            .expect("write terminal auth state");

        let interactive = Authenticator::new(
            AuthSource::OAuth,
            Some(auth_file.clone()),
            None,
            DeviceCodeHandler::default(),
            true,
        );
        let background = Authenticator::new(
            AuthSource::OAuth,
            Some(auth_file),
            None,
            DeviceCodeHandler::default(),
            false,
        );

        let _pending_device_flow = interactive.device_flow_lock.lock().await;
        let result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            background.auth_context(),
        )
        .await
        .expect("background auth must not wait for the interactive device flow")
        .expect_err("terminal auth state requires sign-in")
        .to_string();

        assert!(result.contains("ChatGPT sign-in required"), "{result}");
    }
}
