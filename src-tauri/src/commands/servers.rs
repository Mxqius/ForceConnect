use std::sync::OnceLock;

use std::collections::HashSet;

use forceconnect_dns::{Catalog, Protocol as CatalogProtocol, Source as CatalogSource};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::commands::{CommandResult, custom, defaults};

/// The catalog bundled in the binary is immutable and parsed once. What the
/// user sees is this with their removals and any GitHub download applied — see
/// `commands::defaults`.
///
/// A parse failure here is an authoring mistake in `servers.json`, and the
/// catalog test in `forceconnect-dns` turns it into a failing build rather than
/// a panic on a user's machine — so this cannot realistically fail at runtime.
pub(crate) fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| Catalog::builtin().expect("built-in catalog is validated by tests"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Udp,
    Tcp,
    Dot,
    Doh,
    Doq,
    Doh3,
}

impl From<CatalogProtocol> for Protocol {
    fn from(value: CatalogProtocol) -> Self {
        match value {
            CatalogProtocol::Udp => Self::Udp,
            CatalogProtocol::Tcp => Self::Tcp,
            CatalogProtocol::Dot => Self::Dot,
            CatalogProtocol::Doh => Self::Doh,
            CatalogProtocol::Doq => Self::Doq,
            CatalogProtocol::Doh3 => Self::Doh3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ServerSource {
    Builtin,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub protocol: Protocol,
    /// Plain IPs for `udp`/`tcp`, a URL for `doh`/`doh3`, a hostname otherwise.
    pub addrs: Vec<String>,
    /// IPs used to resolve the endpoint's hostname, queried directly rather
    /// than through our own resolver.
    pub bootstrap: Vec<String>,
    /// Short label for the protocol badge, e.g. `DoH3`.
    pub label: String,
    pub encrypted: bool,
    /// Whether selecting this endpoint requires the local forwarder.
    pub requires_forwarder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub id: String,
    pub name: String,
    pub variant: Option<String>,
    /// Name and variant joined, so the UI does not reimplement the rule.
    pub display_name: String,
    /// Icon key, resolved to a bundled asset by the UI.
    pub provider: String,
    pub tags: Vec<String>,
    pub homepage: Option<String>,
    pub endpoints: Vec<Endpoint>,
    /// Protocols this server offers, for filtering without walking endpoints.
    pub protocols: Vec<Protocol>,
    pub source: ServerSource,
}

pub(crate) fn to_dto(server: &forceconnect_dns::DnsServer) -> Server {
    Server {
        id: server.id.clone(),
        name: server.name.clone(),
        variant: server.variant.clone(),
        display_name: server.display_name(),
        provider: server.provider.clone(),
        tags: server.tags.clone(),
        homepage: server.homepage.clone(),
        protocols: server.endpoints.iter().map(|e| e.protocol.into()).collect(),
        endpoints: server
            .endpoints
            .iter()
            .map(|e| Endpoint {
                protocol: e.protocol.into(),
                addrs: e.addrs.clone(),
                bootstrap: e.bootstrap.iter().map(ToString::to_string).collect(),
                label: e.protocol.label().to_owned(),
                encrypted: e.protocol.is_encrypted(),
                requires_forwarder: e.protocol.requires_forwarder(),
            })
            .collect(),
        source: match server.source {
            CatalogSource::Builtin => ServerSource::Builtin,
            CatalogSource::Custom => ServerSource::Custom,
        },
    }
}

/// Every default server the user has not removed. The user's own come from
/// `list_custom_servers`, and the frontend merges the two.
#[tauri::command]
#[specta::specta]
pub fn list_servers(app: AppHandle) -> CommandResult<Vec<Server>> {
    Ok(defaults::builtin_servers(&app)?
        .iter()
        .map(to_dto)
        .collect())
}

/// Protocols represented anywhere in the list, in display order.
///
/// Drives the server browser's filters, so a protocol nobody offers — which is
/// what removing the last server of one leaves behind — never gets a filter
/// that would always be empty.
#[tauri::command]
#[specta::specta]
pub fn list_protocols(app: AppHandle) -> CommandResult<Vec<Protocol>> {
    let mut protocols: Vec<CatalogProtocol> = defaults::builtin_servers(&app)?
        .iter()
        .chain(&custom::read_all(&app)?)
        .flat_map(|s| s.endpoints.iter().map(|e| e.protocol))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    protocols.sort_unstable();
    Ok(protocols.into_iter().map(Into::into).collect())
}
