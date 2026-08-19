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

/// Visible to `commands::probe`, which resolves a server id that may name one
/// of the user's own entries rather than a catalog one.
pub(crate) fn read_all(app: &AppHandle) -> Result<Vec<DnsServer>, CommandError> {
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

/// Fill in the parts of an address the user should not have to type.
///
/// The form used to demand a complete `https://host/path` for DoH and DoH3,
/// which is more than most people have: a provider hands out a hostname — a DNS
/// record — and the `/dns-query` path is the RFC 8484 well-known default that
/// nearly every resolver actually serves. Requiring the ceremony made a valid
/// server look invalid.
///
/// So the URL is completed rather than required. Anything already complete is
/// passed through untouched, and anything that is still not a valid address
/// after this falls through to `DnsServer::validate`, which rejects it with the
/// same message a bad catalog entry would get. Only the two URL protocols are
/// touched; DoT and DoQ already take a bare hostname, and plain DNS takes an IP
/// and nothing else.
fn normalize_addr(protocol: Protocol, addr: &str) -> String {
    if !matches!(protocol, Protocol::Doh | Protocol::Doh3) {
        return addr.to_owned();
    }

    // A scheme we do not speak is left alone so validation can reject it by
    // name, rather than being silently rewritten into something that works.
    if addr.contains("://") && !addr.starts_with("https://") {
        return addr.to_owned();
    }

    let rest = addr.strip_prefix("https://").unwrap_or(addr);
    let rest = rest.trim_end_matches('/');
    if rest.is_empty() {
        return addr.to_owned();
    }

    if rest.contains('/') {
        format!("https://{rest}")
    } else {
        format!("https://{rest}/dns-query")
    }
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
        .map(|a| a.trim())
        .filter(|a| !a.is_empty())
        .map(|a| normalize_addr(server.protocol, a))
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
    fn bare_hostnames_become_doh_urls() {
        // What a provider actually gives you is a name, not a URL.
        assert_eq!(
            normalize_addr(Protocol::Doh, "dns.example.com"),
            "https://dns.example.com/dns-query"
        );
        assert_eq!(
            normalize_addr(Protocol::Doh3, "dns.example.com"),
            "https://dns.example.com/dns-query"
        );
        // A trailing slash is not a path.
        assert_eq!(
            normalize_addr(Protocol::Doh, "https://dns.example.com/"),
            "https://dns.example.com/dns-query"
        );
    }

    #[test]
    fn complete_addresses_are_left_alone() {
        assert_eq!(
            normalize_addr(Protocol::Doh, "https://dns.example.com/query"),
            "https://dns.example.com/query"
        );
        assert_eq!(
            normalize_addr(Protocol::Doh, "dns.example.com/query"),
            "https://dns.example.com/query"
        );
        // Other protocols take what they take; a hostname here is not a URL.
        assert_eq!(
            normalize_addr(Protocol::Dot, "dns.example.com"),
            "dns.example.com"
        );
        assert_eq!(normalize_addr(Protocol::Udp, "1.1.1.1"), "1.1.1.1");
    }

    #[test]
    fn a_wrong_scheme_is_not_rewritten() {
        // Better to fail validation by name than to be quietly upgraded into
        // something the user did not ask for.
        assert_eq!(
            normalize_addr(Protocol::Doh, "http://dns.example.com/dns-query"),
            "http://dns.example.com/dns-query"
        );
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
