//! DNS domain model for ForceConnect.
//!
//! Deliberately free of any Tauri dependency: the catalog, the prober, and the
//! local resolver are all plain Rust, so they can be exercised with
//! `cargo test -p forceconnect-dns` and reasoned about without a running app.

pub mod cache;
pub mod catalog;
pub mod forwarder;
pub mod leak;
pub mod probe;
pub mod querylog;
pub mod server;
pub mod upstream;

pub use catalog::{Catalog, CatalogError};
pub use forwarder::{BoundSockets, ForwarderError, LocalResolver, StatsSnapshot};
pub use server::{DnsServer, Endpoint, Protocol, Source, ValidationError};
