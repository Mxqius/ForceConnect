//! DNS domain model for ForceConnect.
//!
//! Deliberately free of any Tauri dependency: the catalog, the rules engine,
//! and (later) the forwarder are all plain Rust, so they can be exercised with
//! `cargo test -p forceconnect-dns` and reasoned about without a running app.

pub mod catalog;
pub mod probe;
pub mod server;
pub mod upstream;

pub use catalog::{Catalog, CatalogError};
pub use server::{DnsServer, Endpoint, Protocol, Source, ValidationError};
