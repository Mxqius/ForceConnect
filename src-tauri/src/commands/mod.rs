//! The `#[tauri::command]` surface.
//!
//! Commands stay thin: validate, delegate, map errors. Logic lives in the
//! crates under `crates/`, which are testable without a running app.
//!
//! Types crossing into TypeScript are defined here rather than reusing the
//! internal ones. That is deliberate — it lets the frontend contract be shaped
//! for the UI (and keeps `u64` values, which JavaScript cannot represent
//! exactly, from leaking across as numbers).

pub mod adapters;
pub mod app;
pub mod dns;
pub mod probe;
pub mod servers;

use serde::{Deserialize, Serialize};

/// Anything a command can fail with, in a shape the UI can branch on.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, thiserror::Error)]
#[serde(tag = "kind", content = "message", rename_all = "camelCase")]
pub enum CommandError {
    #[error("{0}")]
    Network(String),

    #[error("{0}")]
    Elevation(String),

    #[error("{0}")]
    Catalog(String),

    #[error("{0}")]
    Internal(String),
}

pub type CommandResult<T> = Result<T, CommandError>;

impl From<forceconnect_net::NetError> for CommandError {
    fn from(value: forceconnect_net::NetError) -> Self {
        Self::Network(value.to_string())
    }
}

impl From<forceconnect_dns::CatalogError> for CommandError {
    fn from(value: forceconnect_dns::CatalogError) -> Self {
        Self::Catalog(value.to_string())
    }
}
