//! ForceConnect's elevated helper.
//!
//! The logic lives here rather than in `main.rs` so it can be tested: the
//! binary target embeds a `requireAdministrator` manifest, and Windows refuses
//! to launch anything built from it without elevation — including a test
//! harness, which fails with `ERROR_ELEVATION_REQUIRED`. `embed-resource`
//! applies the manifest with `rustc-link-arg-bins`, so this library and its
//! tests are unaffected.
//!
//! Dependencies are kept deliberately few. Every crate linked into a binary
//! that runs with administrator rights is code a reviewer has to trust.

pub mod dns_settings;
pub mod journal;
pub mod security;
pub mod server;
pub mod watchdog;

use std::process::ExitCode;

use forceconnect_ipc::Launch;

#[derive(Debug, thiserror::Error)]
pub enum HelperError {
    #[error("bad arguments: {0}")]
    Arguments(#[from] forceconnect_ipc::LaunchArgsError),

    #[error("security: {0}")]
    Security(String),

    #[error("pipe: {0}")]
    Pipe(String),

    #[error("the client failed authentication: {0}")]
    Rejected(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Entry point, called by the thin binary.
pub fn run() -> ExitCode {
    init_tracing();

    let launch = match Launch::parse(std::env::args().skip(1)) {
        Ok(launch) => launch,
        Err(err) => {
            tracing::error!("{err}");
            return ExitCode::from(2);
        }
    };

    tracing::info!(
        pipe = %launch.pipe,
        parent_pid = launch.parent_pid,
        "helper starting"
    );

    match server::serve(&launch) {
        Ok(()) => {
            tracing::info!("helper exiting cleanly");
            ExitCode::SUCCESS
        }
        Err(err) => {
            tracing::error!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_env("FORCECONNECT_LOG")
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}
