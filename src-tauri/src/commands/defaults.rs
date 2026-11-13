//! The default servers, as the user has shaped them.
//!
//! The catalog in `servers.json` is embedded in the binary and cannot change.
//! Two small files in the config directory sit on top of it:
//!
//! - `removed-servers.json` — ids of defaults the user took off their list.
//!   Removal hides rather than deletes, so "restore" is the same file emptied.
//! - `default-servers.json` — the last catalog downloaded from GitHub, laid
//!   over the bundled one (see [`Catalog::effective`]).
//!
//! The user's own servers live in `custom-servers.json` and are not touched by
//! anything here.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Once;
use std::time::Duration;

use forceconnect_dns::{Catalog, DnsServer};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::commands::servers::catalog;
use crate::commands::{CommandError, CommandResult};

/// The catalog on the project's main branch — the same file the next release
/// will embed, so "restore" brings back what a fresh install would have.
const REMOTE_URL: &str =
    "https://raw.githubusercontent.com/Mxqius/ForceConnect/main/src-tauri/crates/dns/servers.json";

/// The real file is ~12 KB. This only exists so a wrong URL, or a hijacked
/// one, cannot make the app buffer something enormous.
const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;

fn config_file(app: &AppHandle, name: &str) -> Result<PathBuf, CommandError> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|err| CommandError::Internal(format!("no config directory: {err}")))?;
    Ok(dir.join(name))
}

/// Write beside the target and rename over it, so a crash mid-write leaves the
/// previous file rather than half of a new one.
fn write_atomic(path: &PathBuf, body: &[u8]) -> Result<(), CommandError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| CommandError::Internal(format!("creating config dir: {err}")))?;
    }

    let staging = path.with_extension("json.tmp");
    std::fs::write(&staging, body)
        .and_then(|()| std::fs::rename(&staging, path))
        .map_err(|err| CommandError::Internal(format!("saving {}: {err}", path.display())))
}

/// Unreadable is treated as empty, as with custom servers: a damaged file
/// should cost the user a setting, not the whole server list.
fn read_removed(app: &AppHandle) -> Result<Vec<String>, CommandError> {
    let path = config_file(app, "removed-servers.json")?;
    let Ok(body) = std::fs::read(&path) else {
        return Ok(Vec::new());
    };

    Ok(serde_json::from_slice(&body).unwrap_or_else(|err| {
        tracing::warn!("ignoring unreadable removed-servers file: {err}");
        Vec::new()
    }))
}

fn write_removed(app: &AppHandle, ids: &[String]) -> Result<(), CommandError> {
    let path = config_file(app, "removed-servers.json")?;
    let body =
        serde_json::to_vec_pretty(ids).map_err(|err| CommandError::Internal(err.to_string()))?;
    write_atomic(&path, &body)
}

/// Revalidated on every read: the file is on disk where anything could have
/// edited it, and a bad one is dropped rather than trusted.
fn read_downloaded(app: &AppHandle) -> Result<Option<Catalog>, CommandError> {
    let path = config_file(app, "default-servers.json")?;
    let Ok(body) = std::fs::read_to_string(&path) else {
        return Ok(None);
    };

    match Catalog::from_remote_json(&body) {
        Ok(catalog) => Ok(Some(catalog)),
        Err(err) => {
            tracing::warn!("ignoring invalid downloaded server list: {err}");
            Ok(None)
        }
    }
}

/// Every default server the user has not removed.
pub(crate) fn builtin_servers(app: &AppHandle) -> Result<Vec<DnsServer>, CommandError> {
    let downloaded = read_downloaded(app)?;
    let removed = read_removed(app)?;
    Ok(catalog().effective(downloaded.as_ref(), &removed))
}

pub(crate) fn find_builtin(app: &AppHandle, id: &str) -> Result<Option<DnsServer>, CommandError> {
    Ok(builtin_servers(app)?.into_iter().find(|s| s.id == id))
}

/// Take default servers off the user's list.
///
/// Returns how many were actually removed; ids that are not a current default
/// (already removed, or a custom server) are ignored rather than failing the
/// batch, so a double-click or a stale selection does not turn into an error.
#[tauri::command]
#[specta::specta]
pub fn remove_builtin_servers(app: AppHandle, ids: Vec<String>) -> CommandResult<u32> {
    let current: HashSet<String> = builtin_servers(&app)?.into_iter().map(|s| s.id).collect();

    let mut removed = read_removed(&app)?;
    let mut count = 0;
    for id in ids {
        if current.contains(&id) && !removed.contains(&id) {
            removed.push(id);
            count += 1;
        }
    }

    if count > 0 {
        write_removed(&app, &removed)?;
    }
    Ok(count)
}

/// Put specific default servers back on the list — the undo for a removal.
///
/// Unlike [`restore_default_servers`] this stays offline and touches only the
/// ids it is given.
#[tauri::command]
#[specta::specta]
pub fn reinstate_builtin_servers(app: AppHandle, ids: Vec<String>) -> CommandResult<()> {
    let mut removed = read_removed(&app)?;
    let before = removed.len();
    removed.retain(|id| !ids.contains(id));

    if removed.len() != before {
        write_removed(&app, &removed)?;
    }
    Ok(())
}

/// Where a restore got its list from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum DefaultsOrigin {
    /// Fetched from GitHub just now.
    Github,
    /// GitHub could not be reached, so this is the copy already on the
    /// computer: the one bundled with the app, plus any earlier download.
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RestoreReport {
    pub origin: DefaultsOrigin,
    /// Servers that are on the list now and were not before.
    pub restored: u32,
    /// Default servers on the list afterwards.
    pub total: u32,
}

/// Bring every default server back, preferring the current list on GitHub.
///
/// Offline is not a dead end: the user asked for their servers back, and the
/// bundled copy can do that without the network. The report says which one they
/// got, so the UI does not claim GitHub answered when it did not.
#[tauri::command]
#[specta::specta]
pub async fn restore_default_servers(app: AppHandle) -> CommandResult<RestoreReport> {
    let before: HashSet<String> = builtin_servers(&app)?.into_iter().map(|s| s.id).collect();

    let origin = match fetch_remote().await {
        Ok(body) => {
            write_atomic(&config_file(&app, "default-servers.json")?, body.as_bytes())?;
            DefaultsOrigin::Github
        }
        Err(err) => {
            tracing::warn!("restoring defaults without GitHub: {err}");
            DefaultsOrigin::Local
        }
    };

    write_removed(&app, &[])?;

    let after = builtin_servers(&app)?;
    Ok(RestoreReport {
        origin,
        restored: after.iter().filter(|s| !before.contains(&s.id)).count() as u32,
        total: after.len() as u32,
    })
}

/// The validated catalog text, ready to save as it arrived.
async fn fetch_remote() -> Result<String, CommandError> {
    ensure_crypto_provider();

    let network = |what: &str, err: reqwest::Error| CommandError::Network(format!("{what}: {err}"));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .https_only(true)
        .user_agent(concat!("ForceConnect/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|err| network("preparing the request", err))?;

    let response = client
        .get(REMOTE_URL)
        .send()
        .await
        .map_err(|err| network("could not reach GitHub", err))?
        .error_for_status()
        .map_err(|err| network("GitHub refused the request", err))?;

    if response
        .content_length()
        .is_some_and(|len| len > MAX_BODY_BYTES as u64)
    {
        return Err(CommandError::Network("the server list is too large".into()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|err| network("the download was interrupted", err))?;
    if bytes.len() > MAX_BODY_BYTES {
        return Err(CommandError::Network("the server list is too large".into()));
    }

    let text = String::from_utf8(bytes.to_vec())
        .map_err(|_| CommandError::Catalog("the server list is not valid text".into()))?;

    // Nothing is saved until it has passed the same checks the bundled list
    // gets, so a half-published or malformed file cannot replace a good one.
    Catalog::from_remote_json(&text)?;
    Ok(text)
}

/// reqwest is built without a bundled TLS backend, because the updater already
/// brings `ring` and a second one is dead weight. That leaves picking the
/// process default to us, and the updater only does it lazily on its own first
/// check — so a restore before any update check would find none.
fn ensure_crypto_provider() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| {
        if rustls::crypto::CryptoProvider::get_default().is_none() {
            // An error means another thread installed one between the check and
            // here, which is the outcome we wanted anyway.
            let _ = rustls::crypto::ring::default_provider().install_default();
        }
    });
}
