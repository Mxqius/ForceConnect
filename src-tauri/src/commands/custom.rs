//! Custom servers, persisted alongside the built-in catalog.

use std::path::PathBuf;

use forceconnect_dns::server::{DnsServer, Endpoint as CatalogEndpoint, Source};
use forceconnect_dns::{Catalog, Protocol as CatalogProtocol};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::commands::servers::{Protocol, Server, to_dto};
use crate::commands::{CommandError, CommandResult};

/// What the add-server form sends.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NewServer {
    pub name: String,
    pub protocol: Protocol,
    /// One or more addresses: IPs for plain DNS, a URL for DoH/DoH3, a
    /// hostname for DoT/DoQ.
    pub addrs: Vec<String>,
    /// Required for encrypted endpoints named by hostname.
    pub bootstrap: Vec<String>,
    pub tags: Vec<String>,
}

fn store_path(app: &AppHandle) -> Result<PathBuf, CommandError> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|err| CommandError::Internal(format!("no config directory: {err}")))?;

    Ok(dir.join("custom-servers.json"))
}

fn read_all(app: &AppHandle) -> Result<Vec<DnsServer>, CommandError> {
    let path = store_path(app)?;

    let Ok(body) = std::fs::read(&path) else {
        return Ok(Vec::new());
    };

    // A corrupt file should not make the app unusable. Losing custom servers
    // is bad, but refusing to start is worse, and the file is left in place
    // so it can be recovered by hand.
    match serde_json::from_slice(&body) {
        Ok(servers) => Ok(servers),
        Err(err) => {
            tracing::warn!("ignoring unreadable custom server file: {err}");
            Ok(Vec::new())
        }
    }
}

fn write_all(app: &AppHandle, servers: &[DnsServer]) -> Result<(), CommandError> {
    let path = store_path(app)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| CommandError::Internal(format!("creating config dir: {err}")))?;
    }

    let body = serde_json::to_vec_pretty(servers)
        .map_err(|err| CommandError::Internal(err.to_string()))?;

    std::fs::write(&path, body)
        .map_err(|err| CommandError::Internal(format!("writing custom servers: {err}")))
}

/// Turn a free-form name into a stable, unique id.
///
/// The id is persisted and referenced by favourites and the active selection,
/// so it must not change when the user edits unrelated fields.
fn make_id(name: &str, existing: &[DnsServer]) -> String {
    let base: String = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();

    let base = base.trim_matches('-').to_owned();
    let base = if base.is_empty() {
        "custom".to_owned()
    } else {
        base
    };

    let mut candidate = format!("custom-{base}");
    let mut suffix = 2;
    while existing.iter().any(|s| s.id == candidate) {
        candidate = format!("custom-{base}-{suffix}");
        suffix += 1;
    }

    candidate
}

fn to_catalog_protocol(protocol: Protocol) -> CatalogProtocol {
    match protocol {
        Protocol::Udp => CatalogProtocol::Udp,
        Protocol::Tcp => CatalogProtocol::Tcp,
        Protocol::Dot => CatalogProtocol::Dot,
        Protocol::Doh => CatalogProtocol::Doh,
        Protocol::Doq => CatalogProtocol::Doq,
        Protocol::Doh3 => CatalogProtocol::Doh3,
    }
}

/// Every server: built-ins first, then the user's own.
#[tauri::command]
#[specta::specta]
pub fn list_custom_servers(app: AppHandle) -> CommandResult<Vec<Server>> {
    Ok(read_all(&app)?.iter().map(to_dto).collect())
}

/// Validate and save a custom server.
///
/// Validation runs through the same rules as the built-in catalog, so a custom
/// entry cannot be saved in a state that would fail at connect time — including
/// the bootstrap requirement that keeps an encrypted upstream from trying to
/// resolve itself through us.
#[tauri::command]
#[specta::specta]
pub fn add_custom_server(app: AppHandle, server: NewServer) -> CommandResult<Server> {
    let name = server.name.trim().to_owned();
    if name.is_empty() {
        return Err(CommandError::Internal("a name is required".into()));
    }

    let addrs: Vec<String> = server
        .addrs
        .iter()
        .map(|a| a.trim().to_owned())
        .filter(|a| !a.is_empty())
        .collect();

    if addrs.is_empty() {
        return Err(CommandError::Internal(
            "at least one address is required".into(),
        ));
    }

    let bootstrap = server
        .bootstrap
        .iter()
        .map(|b| b.trim())
        .filter(|b| !b.is_empty())
        .map(|b| {
            b.parse().map_err(|_| {
                CommandError::Internal(format!("'{b}' is not a valid bootstrap IP address"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut existing = read_all(&app)?;

    let candidate = DnsServer {
        id: make_id(&name, &existing),
        name,
        variant: None,
        provider: "custom".into(),
        tags: server
            .tags
            .iter()
            .map(|t| t.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect(),
        homepage: None,
        endpoints: vec![CatalogEndpoint {
            protocol: to_catalog_protocol(server.protocol),
            addrs,
            bootstrap,
        }],
        source: Source::Custom,
    };

    candidate
        .validate()
        .map_err(|err| CommandError::Internal(err.to_string()))?;

    // Reject an id that already exists in the built-in catalog, which would
    // make the two indistinguishable everywhere the id is used as a key.
    if Catalog::builtin()
        .map_err(CommandError::from)?
        .get(&candidate.id)
        .is_some()
    {
        return Err(CommandError::Internal(
            "that name collides with a built-in server".into(),
        ));
    }

    let dto = to_dto(&candidate);
    existing.push(candidate);
    write_all(&app, &existing)?;

    Ok(dto)
}

#[tauri::command]
#[specta::specta]
pub fn remove_custom_server(app: AppHandle, id: String) -> CommandResult<()> {
    let mut servers = read_all(&app)?;
    let before = servers.len();
    servers.retain(|s| s.id != id);

    if servers.len() == before {
        return Err(CommandError::Internal(format!("no custom server '{id}'")));
    }

    write_all(&app, &servers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn server(id: &str) -> DnsServer {
        DnsServer {
            id: id.into(),
            name: id.into(),
            variant: None,
            provider: "custom".into(),
            tags: vec![],
            homepage: None,
            endpoints: vec![],
            source: Source::Custom,
        }
    }

    #[test]
    fn ids_are_slugified_and_prefixed() {
        assert_eq!(make_id("My Resolver", &[]), "custom-my-resolver");
        assert_eq!(make_id("  spaced  ", &[]), "custom-spaced");
        assert_eq!(make_id("!!!", &[]), "custom-custom");
    }

    #[test]
    fn ids_do_not_collide() {
        // Two servers named the same must stay separately addressable, since
        // favourites and the active selection are keyed by id.
        let existing = vec![server("custom-home"), server("custom-home-2")];
        assert_eq!(make_id("Home", &existing), "custom-home-3");
    }
}
