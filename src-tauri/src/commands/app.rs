use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::{CommandError, CommandResult};
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

/// Switch the main window between its full and compact sizes.
///
/// Persisted by the frontend; at launch `window::restore_saved_mode` applies it
/// before the first paint, and this handles every later toggle.
#[tauri::command]
#[specta::specta]
pub fn set_compact_mode(
    window: tauri::WebviewWindow,
    compact: bool,
    always_on_top: bool,
) -> CommandResult<()> {
    crate::window::apply_mode(&window, compact, always_on_top)
        .map_err(|err| CommandError::Internal(err.to_string()))
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

/// Whether this build can actually install an update.
///
/// The updater plugin is always registered, but it can only *verify* a download
/// if `plugins.updater.pubkey` was filled in at build time — and an update that
/// cannot be verified is one an attacker can write, so the plugin rightly
/// refuses it. Without this, the only way for a user to discover an unsigned
/// build is to download an update and watch it fail at the last step.
///
/// Reported up front instead, so the Settings screen can say "this build cannot
/// install updates" rather than offering a button that is guaranteed to fail.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdaterStatus {
    /// A signing key and at least one endpoint are both present.
    pub configured: bool,
    pub endpoints: Vec<String>,
}

#[tauri::command]
#[specta::specta]
pub fn updater_status(app: tauri::AppHandle) -> CommandResult<UpdaterStatus> {
    let config = app.config();

    let updater = config
        .plugins
        .0
        .get("updater")
        .and_then(|value| value.as_object());

    let pubkey = updater
        .and_then(|u| u.get("pubkey"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let endpoints: Vec<String> = updater
        .and_then(|u| u.get("endpoints"))
        .and_then(|v| v.as_array())
        .map(|list| {
            list.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    Ok(UpdaterStatus {
        configured: !pubkey.trim().is_empty() && !endpoints.is_empty(),
        endpoints,
    })
}

fn os_description() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}
