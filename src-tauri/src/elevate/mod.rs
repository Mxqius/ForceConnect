//! Talking to the elevated helper.
//!
//! The UI never performs a privileged operation itself. It launches the helper
//! on demand — one UAC prompt — and asks it to do the small set of things that
//! require administrator rights.
//!
//! Note that this is one prompt per *app run*, not per Windows session: closing
//! and reopening ForceConnect asks again. Eliminating that entirely would mean
//! a service installed at setup time, which is a larger security surface and a
//! separate decision.

pub mod client;
pub mod launcher;

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ElevationError {
    /// The user dismissed the UAC prompt. Not an error condition — a decision.
    #[error("elevation was declined")]
    Declined,

    #[error("the helper executable is missing from {0}")]
    HelperMissing(PathBuf),

    #[error("could not start the helper: {0}")]
    Launch(String),

    #[error("could not reach the helper: {0}")]
    Connect(String),

    #[error("the helper rejected the request: {0}")]
    Rejected(String),

    #[error("the helper reported: {0}")]
    Remote(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Locate `forceconnect-helper.exe`.
///
/// Checked next to the running binary first, which covers both `cargo`/`tauri
/// dev` output and an installed layout where Tauri places `externalBin`
/// alongside the main executable. The resource directory is the fallback.
pub fn helper_path(resource_dir: Option<PathBuf>) -> Result<PathBuf, ElevationError> {
    const NAME: &str = "forceconnect-helper.exe";

    let mut searched = Vec::new();

    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let candidate = dir.join(NAME);
        if candidate.is_file() {
            return Ok(candidate);
        }
        searched.push(candidate);
    }

    if let Some(dir) = resource_dir {
        let candidate = dir.join(NAME);
        if candidate.is_file() {
            return Ok(candidate);
        }
        searched.push(candidate);
    }

    Err(ElevationError::HelperMissing(
        searched.into_iter().next().unwrap_or_else(|| NAME.into()),
    ))
}

/// A 256-bit hex nonce from the OS CSPRNG.
///
/// Worth restating: this is a squatting guard, not authentication. It travels
/// on the helper's command line, which any process running as the same user can
/// read. The real assurance is that the helper pins its client to the exact
/// process that triggered the UAC prompt, and that the protocol exposes nothing
/// the user could not already do through Windows Settings.
#[cfg(windows)]
pub fn generate_nonce() -> String {
    use windows::Win32::Security::Cryptography::{
        BCRYPT_USE_SYSTEM_PREFERRED_RNG, BCryptGenRandom,
    };

    let mut bytes = [0u8; 32];
    // SAFETY: writing exactly `bytes.len()` bytes into a stack buffer.
    unsafe {
        let _ = BCryptGenRandom(None, &mut bytes, BCRYPT_USE_SYSTEM_PREFERRED_RNG);
    }

    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(not(windows))]
pub fn generate_nonce() -> String {
    "unsupported".into()
}

/// Pipe name for this session, suffixed so two instances cannot collide and a
/// stale helper cannot be mistaken for ours.
pub fn pipe_name(nonce: &str) -> String {
    format!("{}-{}", forceconnect_ipc::PIPE_PREFIX, &nonce[..16])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_names_are_scoped_to_the_session() {
        let a = pipe_name(&"a".repeat(64));
        let b = pipe_name(&"b".repeat(64));

        assert!(a.starts_with(r"\\.\pipe\forceconnect"));
        assert_ne!(a, b, "two sessions must not share a pipe name");
    }

    #[cfg(windows)]
    #[test]
    fn nonces_are_long_and_distinct() {
        let a = generate_nonce();
        let b = generate_nonce();

        assert_eq!(a.len(), 64, "256 bits as hex");
        assert_ne!(a, b);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        // An all-zero nonce would mean the RNG call silently failed.
        assert_ne!(a, "0".repeat(64));
    }
}
