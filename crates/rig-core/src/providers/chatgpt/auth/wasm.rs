//! WASM ChatGPT auth implementation.

use super::{AuthContext, AuthError, DeviceCodeHandler};
use std::path::PathBuf;

pub(super) struct PendingDeviceFlow;

#[derive(Debug, Clone, Default)]
pub(super) struct PlatformAuthenticator;

impl PlatformAuthenticator {
    pub(super) fn new(
        _auth_file: Option<PathBuf>,
        _oauth_http_client: Option<reqwest::Client>,
        _device_code_handler: DeviceCodeHandler,
        _allow_device_flow: bool,
    ) -> Self {
        Self
    }

    pub(super) fn device_flow_allowed(&self) -> bool {
        false
    }

    pub(super) async fn cached_or_refreshed_context(
        &self,
    ) -> Result<Option<AuthContext>, AuthError> {
        Err(AuthError::Message(
            "ChatGPT OAuth is not supported on wasm targets".into(),
        ))
    }

    pub(super) async fn login_device_flow(&self) -> Result<PendingDeviceFlow, AuthError> {
        Err(AuthError::Message(
            "ChatGPT OAuth is not supported on wasm targets".into(),
        ))
    }

    pub(super) fn persist_device_flow(
        &self,
        _fresh: PendingDeviceFlow,
    ) -> Result<AuthContext, AuthError> {
        Err(AuthError::Message(
            "ChatGPT OAuth is not supported on wasm targets".into(),
        ))
    }

    pub(super) async fn refresh_after_rejection(
        &self,
        _rejected_access_token: &str,
    ) -> Result<AuthContext, AuthError> {
        Err(AuthError::Message(
            "ChatGPT OAuth is not supported on wasm targets".into(),
        ))
    }
}
