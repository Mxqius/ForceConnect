//! The `#[tauri::command]` surface.
//!
//! Commands stay thin: they validate, delegate, and map errors. Logic belongs
//! in the crates under `crates/`, which are testable without a running app.

use serde::{Deserialize, Serialize};

/// Anything a command can fail with, in a shape the UI can branch on.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, thiserror::Error)]
#[serde(tag = "kind", content = "message", rename_all = "camelCase")]
pub enum CommandError {
    #[error("{0}")]
    Network(String),

    #[error("{0}")]
    Elevation(String),

    #[error("{0}")]
    Internal(String),
}

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub os: String,
    /// Whether *this* process is elevated. Expected to be `false` in normal
    /// operation — the UI runs unelevated by design and delegates privileged
    /// work to the helper. A `true` here means someone ran the app as
    /// administrator, which is supported but worth surfacing.
    pub elevated: bool,
}

#[tauri::command]
#[specta::specta]
pub fn app_info() -> CommandResult<AppInfo> {
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_owned(),
        os: os_description(),
        elevated: is_elevated(),
    })
}

fn os_description() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

/// Report whether the current process token is elevated.
#[cfg(windows)]
fn is_elevated() -> bool {
    use std::mem::size_of;

    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Security::{
        GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    // SAFETY: the handle is closed on every path, and `GetTokenInformation` is
    // given a correctly sized buffer for the class being requested.
    unsafe {
        let mut token = Default::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut returned = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&raw mut elevation as *mut _),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned,
        )
        .is_ok();

        let _ = CloseHandle(token);
        ok && elevation.TokenIsElevated != 0
    }
}

#[cfg(not(windows))]
fn is_elevated() -> bool {
    false
}
