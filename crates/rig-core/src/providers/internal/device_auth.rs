//! Device-flow OAuth helpers shared by the native (non-wasm) ChatGPT and
//! Copilot authenticators: on-disk JSON record caching, token expiry checks,
//! and the device-code prompt fallback.

use super::auth::AuthError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;
use std::sync::Arc;

/// Invokes the provider's device-code callback when one is registered,
/// otherwise prints `fallback_message` to stdout.
pub(crate) fn emit_device_code_prompt<P>(
    callback: Option<&Arc<dyn Fn(P) + Send + Sync>>,
    prompt: P,
    fallback_message: &str,
) {
    if let Some(callback) = callback {
        callback(prompt);
    } else {
        println!("{fallback_message}");
    }
}

pub(crate) fn ensure_parent_dir(path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Returns true when the token is expired (or has no expiry), treating the
/// token as expired `skew_seconds` before its actual `expires_at`.
pub(crate) fn token_expired(expires_at: Option<i64>, skew_seconds: i64) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default();

    match expires_at {
        Some(exp) => now >= exp - skew_seconds,
        None => true,
    }
}

/// Reads a JSON record from `path`, returning `T::default()` when no path is
/// configured or the file does not exist.
pub(crate) fn read_json_record<T: Default + DeserializeOwned>(
    path: Option<&Path>,
) -> Result<T, AuthError> {
    let Some(path) = path else {
        return Ok(T::default());
    };

    match std::fs::read(path) {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(T::default()),
        Err(err) => Err(err.into()),
    }
}

/// Writes a JSON record to `path` (creating parent directories), a no-op when
/// no path is configured.
pub(crate) fn write_json_record<T: Serialize>(
    path: Option<&Path>,
    record: &T,
) -> Result<(), AuthError> {
    let Some(path) = path else {
        return Ok(());
    };

    ensure_parent_dir(path)?;
    let data = serde_json::to_vec_pretty(record)?;
    write_private_record(path, &data)?;
    Ok(())
}

#[cfg(unix)]
fn write_private_record(path: &Path, data: &[u8]) -> Result<(), std::io::Error> {
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let mut random = [0_u8; 12];
    getrandom::fill(&mut random).map_err(std::io::Error::other)?;
    let suffix = random
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("auth.json");
    let temporary = path.with_file_name(format!(".{file_name}.{suffix}.tmp"));
    let result = (|| {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)?;
        file.write_all(data)?;
        file.sync_all()?;
        std::fs::rename(&temporary, path)?;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result
}

#[cfg(not(unix))]
fn write_private_record(path: &Path, data: &[u8]) -> Result<(), std::io::Error> {
    // Windows applies ACLs inherited from the per-user config directory. Its
    // rename primitive does not atomically replace an existing file, so retain
    // the previous overwrite behavior instead of adding a delete gap.
    std::fs::write(path, data)
}

#[cfg(all(test, unix))]
mod tests {
    use super::write_json_record;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn auth_records_are_replaced_atomically_with_private_permissions() {
        let temp = assert_fs::TempDir::new().expect("temp auth directory");
        let path = temp.path().join("nested/auth.json");

        write_json_record(Some(&path), &serde_json::json!({"token": "first"}))
            .expect("first auth write");
        write_json_record(Some(&path), &serde_json::json!({"token": "second"}))
            .expect("replacement auth write");

        let stored: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).expect("read auth record"))
                .expect("parse auth record");
        assert_eq!(stored["token"], "second");
        assert_eq!(
            std::fs::metadata(&path)
                .expect("auth metadata")
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
