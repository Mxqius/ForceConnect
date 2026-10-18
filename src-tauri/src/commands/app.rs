use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::{CommandError, CommandResult};
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub os: String,
    /// Whether the app holds administrator rights.
    ///
    /// On Windows, whether this process is elevated: expected to be `true`,
    /// since the executable embeds a `requireAdministrator` manifest, and
    /// `false` only for a build with the manifest stripped. On Linux and macOS,
    /// whether the privileged agent is running — `false` when the user declined
    /// the prompt at launch. Either way a `false` means every DNS action will
    /// refuse rather than fail halfway.
    pub elevated: bool,
    /// This process's own id, so a user can find it in Task Manager or
    /// Activity Monitor.
    pub pid: u32,
}

// Async, and on a blocking thread, because on Linux and macOS the answer is not
// known until the user has answered the administrator prompt — and a
// synchronous command would hold the main thread while they type.
#[tauri::command]
#[specta::specta]
pub async fn app_info(state: State<'_, AppState>) -> CommandResult<AppInfo> {
    let privileged = state.dns();
    let elevated = tauri::async_runtime::spawn_blocking(move || privileged.is_elevated())
        .await
        .map_err(|err| CommandError::Internal(err.to_string()))?;

    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_owned(),
        os: os_description(),
        elevated,
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
