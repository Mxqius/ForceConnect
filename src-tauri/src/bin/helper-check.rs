//! End-to-end check of the elevation path.
//!
//! The UAC prompt is modal, so this stage of the design cannot be verified by
//! anything that runs unattended. This binary makes it a single command with a
//! single click:
//!
//! ```text
//! cargo run --bin helper-check                 # connect only, changes nothing
//! cargo run --bin helper-check -- --apply      # also write and revert DNS
//! ```
//!
//! Without `--apply` it stops after the handshake and a ping: enough to prove
//! the launcher, the pipe DACL, the mandatory-integrity label, and the caller
//! pinning all work, without touching the machine's network configuration.

use std::process::ExitCode;

use forceconnect_ipc::{Family, InterfaceLuid, Request, Response};
use forceconnect_lib::elevate::{client::HelperSession, helper_path};
use forceconnect_net::{Family as NetFamily, adapters, route};

fn main() -> ExitCode {
    let apply = std::env::args().any(|a| a == "--apply");

    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(err) => {
            println!("[fail] could not start a runtime: {err}");
            return ExitCode::FAILURE;
        }
    };

    match runtime.block_on(check(apply)) {
        Ok(()) => {
            println!("\nall stages passed");
            ExitCode::SUCCESS
        }
        Err(err) => {
            println!("\n[fail] {err}");
            ExitCode::FAILURE
        }
    }
}

async fn check(apply: bool) -> Result<(), String> {
    let path = helper_path(None).map_err(|e| e.to_string())?;
    println!("[ok]   found helper at {}", path.display());

    println!("       requesting elevation — approve the prompt");
    let session = HelperSession::establish(path, 0)
        .await
        .map_err(|e| e.to_string())?;
    println!(
        "[ok]   connected: helper {} running as pid {}",
        session.helper_version, session.pid
    );
    println!("       (the pipe DACL and medium integrity label are proven by this)");

    match session
        .call(Request::Ping)
        .await
        .map_err(|e| e.to_string())?
    {
        Response::Pong => println!("[ok]   ping answered"),
        other => return Err(format!("expected Pong, got {other:?}")),
    }

    if !apply {
        println!("\n[skip] DNS write not attempted; pass --apply to test it");
        let _ = session.call(Request::Shutdown).await;
        return Ok(());
    }

    let all = adapters::list().map_err(|e| e.to_string())?;
    let target =
        route::best_adapter(&all, NetFamily::V4).ok_or("no active adapter to test against")?;
    println!(
        "[ok]   target adapter: {} (metric {})",
        target.friendly_name, target.metric_v4
    );

    let before = adapters::dns_config(&target.guid, NetFamily::V4).map_err(|e| e.to_string())?;
    println!("[ok]   current v4 config: {before:?}");

    let luid = InterfaceLuid(target.luid.0);
    let test_server = "9.9.9.9".parse().unwrap();

    match session
        .call(Request::SetDns {
            luid,
            family: Family::V4,
            servers: vec![test_server],
        })
        .await
        .map_err(|e| e.to_string())?
    {
        Response::Applied { previous } => {
            println!("[ok]   set DNS to 9.9.9.9; helper saved previous={previous:?}");
        }
        other => return Err(format!("expected Applied, got {other:?}")),
    }

    let during = adapters::dns_config(&target.guid, NetFamily::V4).map_err(|e| e.to_string())?;
    if during.servers() != [test_server] {
        return Err(format!("readback shows {during:?}, expected 9.9.9.9"));
    }
    println!("[ok]   readback confirms the change landed");

    session
        .call(Request::RestoreAll)
        .await
        .map_err(|e| e.to_string())?;

    let after = adapters::dns_config(&target.guid, NetFamily::V4).map_err(|e| e.to_string())?;
    if after != before {
        return Err(format!(
            "revert left {after:?} but the adapter started as {before:?}"
        ));
    }
    println!("[ok]   reverted exactly to the original configuration");

    // The DHCP case is the one that silently breaks: restoring must clear the
    // static list, not write the observed servers back as static.
    if before.is_dhcp() {
        println!("[ok]   DHCP was restored as DHCP, not pinned to observed servers");
    }

    let _ = session.call(Request::Shutdown).await;
    Ok(())
}
