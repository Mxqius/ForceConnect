use std::sync::OnceLock;

use forceconnect_dns::{Catalog, Protocol as CatalogProtocol, Source as CatalogSource};
use serde::{Deserialize, Serialize};

use crate::commands::CommandResult;

/// The built-in catalog is immutable and parsed once.
///
/// A parse failure here is an authoring mistake in `servers.json`, and the
/// catalog test in `forceconnect-dns` turns it into a failing build rather than
/// a panic on a user's machine — so this cannot realistically fail at runtime.
fn catalog() -> &'static Catalog {
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

/// Every built-in server.
///
/// Custom servers are merged in once persistence lands; the shape of the
/// response does not change when they do.
#[tauri::command]
#[specta::specta]
pub fn list_servers() -> CommandResult<Vec<Server>> {
    Ok(catalog().servers().iter().map(to_dto).collect())
}

/// Protocols represented anywhere in the catalog, in display order.
///
/// Drives the server browser's tabs, so a protocol nobody offers never gets a
/// tab that would always be empty.
#[tauri::command]
#[specta::specta]
pub fn list_protocols() -> CommandResult<Vec<Protocol>> {
    Ok(catalog().protocols().into_iter().map(Into::into).collect())
}
