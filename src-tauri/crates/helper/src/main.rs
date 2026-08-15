//! ForceConnect's elevated helper.
//!
//! This process exists so the user interface never has to run as administrator.
//! It is launched on demand by the main app via `ShellExecuteExW` with the
//! `runas` verb, serves a single client over a named pipe, and exits when that
//! client goes away — reverting anything it changed on the way out.
//!
//! It is not meant to be run directly, and refuses to do anything if it is.
//!
//! Dependencies here are kept deliberately few. Every crate linked into a
//! binary that runs with administrator rights is code a reviewer has to trust.
//!
//! # Why there are no tests in this file
//!
//! The embedded manifest requests `requireAdministrator`, so Windows refuses to
//! launch *any* binary built from this target without elevation — including the
//! test harness, which fails with `ERROR_ELEVATION_REQUIRED` (740). Logic worth
//! testing therefore lives in `forceconnect-ipc` and `forceconnect-net`, and
//! this file stays a shell around it.

use std::process::ExitCode;

use forceconnect_ipc::Launch;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("FORCECONNECT_LOG")
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

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

    // The pipe server, parent watchdog, and revert journal land with the
    // privileged operations themselves. Until then this binary exists so the
    // manifest, bundling, and signing path can be exercised end to end.
    tracing::warn!("privileged operations are not implemented yet; exiting");
    ExitCode::SUCCESS
}
