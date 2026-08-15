//! The built-in server catalog.
//!
//! The catalog is data, not code: it lives in `servers.json` and is embedded at
//! build time. Adding a provider is a data-only change, which is the whole
//! point — it keeps the most likely outside contribution to this project from
//! requiring any Rust.

use std::collections::HashSet;

use crate::server::{DnsServer, Protocol, ValidationError};

const BUILTIN_JSON: &str = include_str!("../servers.json");

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("catalog is not valid JSON: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("catalog contains an invalid server: {0}")]
    Invalid(#[from] ValidationError),

    #[error("duplicate server id '{0}'")]
    DuplicateId(String),
}

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    servers: Vec<DnsServer>,
}

impl Catalog {
    /// Load and validate the embedded catalog.
    ///
    /// A malformed catalog is a build-time authoring mistake, and the test at
    /// the bottom of this file turns it into a failing `cargo test` rather than
    /// a panic on a user's machine.
    pub fn builtin() -> Result<Self, CatalogError> {
        Self::from_json(BUILTIN_JSON)
    }

    pub fn from_json(json: &str) -> Result<Self, CatalogError> {
        let servers: Vec<DnsServer> = serde_json::from_str(json)?;

        let mut seen = HashSet::with_capacity(servers.len());
        for server in &servers {
            server.validate()?;
            if !seen.insert(server.id.as_str()) {
                return Err(CatalogError::DuplicateId(server.id.clone()));
            }
        }

        Ok(Self { servers })
    }

    pub fn servers(&self) -> &[DnsServer] {
        &self.servers
    }

    pub fn len(&self) -> usize {
        self.servers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.servers.is_empty()
    }

    pub fn get(&self, id: &str) -> Option<&DnsServer> {
        self.servers.iter().find(|s| s.id == id)
    }

    /// What the server browser renders: optionally narrowed to one protocol,
    /// then to a search string.
    pub fn filter(&self, protocol: Option<Protocol>, query: &str) -> Vec<&DnsServer> {
        self.servers
            .iter()
            .filter(|s| protocol.is_none_or(|p| s.supports(p)))
            .filter(|s| s.matches(query))
            .collect()
    }

    /// Distinct protocols offered across the whole catalog, in display order.
    pub fn protocols(&self) -> Vec<Protocol> {
        let mut protocols: Vec<Protocol> = self
            .servers
            .iter()
            .flat_map(|s| s.endpoints.iter().map(|e| e.protocol))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        protocols.sort_unstable();
        protocols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_catalog_is_valid() {
        // Guards every future edit to servers.json: bad addresses, missing
        // bootstrap IPs on encrypted endpoints, and duplicate ids all fail here.
        let catalog = Catalog::builtin().expect("built-in catalog should be valid");
        assert!(
            catalog.len() >= 20,
            "expected a substantial catalog, got {}",
            catalog.len()
        );
    }

    #[test]
    fn builtin_catalog_covers_every_protocol() {
        let catalog = Catalog::builtin().unwrap();
        for protocol in [
            Protocol::Udp,
            Protocol::Dot,
            Protocol::Doh,
            Protocol::Doq,
            Protocol::Doh3,
        ] {
            assert!(
                catalog.servers().iter().any(|s| s.supports(protocol)),
                "no built-in server offers {protocol:?}"
            );
        }
    }

    #[test]
    fn every_encrypted_endpoint_can_bootstrap() {
        // Without this, the first encrypted lookup after we become the system
        // resolver would try to resolve its own upstream through itself.
        let catalog = Catalog::builtin().unwrap();
        for server in catalog.servers() {
            for endpoint in &server.endpoints {
                if endpoint.protocol.is_encrypted() {
                    assert!(
                        !endpoint.bootstrap.is_empty(),
                        "{} has no bootstrap for {:?}",
                        server.id,
                        endpoint.protocol
                    );
                }
            }
        }
    }

    #[test]
    fn filtering_narrows_by_protocol_and_query() {
        let catalog = Catalog::builtin().unwrap();

        let all = catalog.filter(None, "");
        let doh = catalog.filter(Some(Protocol::Doh), "");
        assert!(doh.len() < all.len(), "DoH subset should be smaller");
        assert!(doh.iter().all(|s| s.supports(Protocol::Doh)));

        let cloudflare = catalog.filter(None, "cloudflare");
        assert!(!cloudflare.is_empty());
        assert!(cloudflare.iter().all(|s| s.matches("cloudflare")));
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let json = r#"[
            {"id":"a","name":"A","provider":"a","endpoints":[{"protocol":"udp","addrs":["1.1.1.1"]}]},
            {"id":"a","name":"B","provider":"b","endpoints":[{"protocol":"udp","addrs":["8.8.8.8"]}]}
        ]"#;

        assert!(matches!(
            Catalog::from_json(json),
            Err(CatalogError::DuplicateId(_))
        ));
    }
}
