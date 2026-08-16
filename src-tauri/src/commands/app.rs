use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::CommandResult;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub os: String,
    /// Whether this process holds administrator rights.
    ///
    /// Expected to be `true`: the executable embeds a `requireAdministrator`
    /// manifest, so Windows asks once at launch and the app has what it needs
    /// for the rest of the session. A `false` here means a build with the
    /// manifest stripped, and every DNS action will refuse rather than fail
    /// halfway.
    pub elevated: bool,
    /// This process's own id, so a user can confirm in Task Manager exactly
    /// which process is the one running elevated.
    pub pid: u32,
}

#[tauri::command]
#[specta::specta]
pub fn app_info() -> CommandResult<AppInfo> {
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_owned(),
        os: os_description(),
        elevated: forceconnect_privileged::is_elevated(),
        pid: std::process::id(),
    })
}

/// Sync the window-close behaviour with the persisted preference.
///
/// The preference itself lives in the frontend's store; this just tells the
/// close handler in `lib.rs` whether it should hide the window instead of
/// letting it exit.
#[tauri::command]
#[specta::specta]
pub fn set_minimize_to_tray(state: State<'_, AppState>, enabled: bool) -> CommandResult<()> {
    state.set_minimize_to_tray(enabled);
    Ok(())
}

/// A snapshot of the connection, for the tray to show.
///
/// The tray has no way to know this on its own — "connected to X over Y" is
/// derived from the catalog entry the user picked, not from anything the
/// adapter's DNS settings alone reveal — so the frontend pushes it across
/// whenever `useConnection`'s store changes.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrayStatus {
    pub connected: bool,
    pub server_name: Option<String>,
    pub protocol_label: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub fn set_tray_status(app: tauri::AppHandle, status: TrayStatus) -> CommandResult<()> {
    crate::tray::update_status(&app, &status);
    Ok(())
}

fn os_description() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}
