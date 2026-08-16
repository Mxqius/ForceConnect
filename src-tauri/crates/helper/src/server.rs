//! The pipe server: one client, served sequentially, then exit.

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use forceconnect_ipc::{
    DnsSnapshot, Family, InterfaceLuid, Launch, MAX_FRAME_BYTES, PROTOCOL_VERSION, ProtocolError,
    Request, Response, decode, encode,
};
use forceconnect_net::{DnsConfig, adapters};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::{NamedPipeServer, PipeMode, ServerOptions};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Pipes::GetNamedPipeClientProcessId;

use crate::{HelperError, dns_settings, journal, security, watchdog};

/// How long to wait for the client that triggered the UAC prompt.
///
/// If nobody connects, something went wrong on the other side and an idle
/// elevated process is the last thing we want lying around.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// Applied changes, keyed by adapter and family, holding the prior state.
type Applied = HashMap<(String, Family), DnsConfig>;

pub fn serve(launch: &Launch) -> Result<(), HelperError> {
    // Current-thread runtime: exactly one client, served sequentially. Smaller
    // and easier to reason about, which matters most in the elevated process.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async { serve_async(launch).await })
}

async fn serve_async(launch: &Launch) -> Result<(), HelperError> {
    // Pin the parent before anything else: if the PID has been recycled we
    // must not serve, and we must not revert someone else's settings either.
    let actual_start = watchdog::creation_time(launch.parent_pid)?;
    if actual_start != launch.parent_start {
        return Err(HelperError::Rejected(format!(
            "parent {} started at {actual_start}, expected {} — refusing to serve",
            launch.parent_pid, launch.parent_start
        )));
    }

    let logon_sid = security::logon_sid_of(launch.parent_pid)?;
    let mut descriptor = security::PipeSecurity::new(&logon_sid)?;

    // SAFETY: `descriptor` outlives the call and points at a valid
    // SECURITY_ATTRIBUTES whose descriptor was built from our own SDDL.
    let server = unsafe {
        ServerOptions::new()
            // Fail rather than join a pipe someone else already published
            // under our name.
            .first_pipe_instance(true)
            .max_instances(1)
            .reject_remote_clients(true)
            .pipe_mode(PipeMode::Byte)
            .in_buffer_size(MAX_FRAME_BYTES as u32)
            .out_buffer_size(MAX_FRAME_BYTES as u32)
            .create_with_security_attributes_raw(&launch.pipe, descriptor.as_raw())
    }
    .map_err(|e| HelperError::Pipe(format!("creating {}: {e}", launch.pipe)))?;

    // The watchdog runs on its own thread and only signals; the async side
    // owns all the state.
    let (tx, rx) = std::sync::mpsc::channel();
    let (parent_pid, parent_start) = (launch.parent_pid, launch.parent_start);
    std::thread::spawn(move || {
        if let Err(err) = watchdog::watch(parent_pid, parent_start, tx) {
            tracing::error!("watchdog stopped: {err}");
        }
    });

    let journal_path = journal::default_path();
    let mut applied: Applied = HashMap::new();

    let outcome = tokio::time::timeout(CONNECT_TIMEOUT, server.connect()).await;
    match outcome {
        Err(_) => {
            tracing::warn!("no client connected within {CONNECT_TIMEOUT:?}; exiting");
            return Ok(());
        }
        Ok(Err(err)) => return Err(HelperError::Pipe(format!("accepting: {err}"))),
        Ok(Ok(())) => {}
    }

    verify_client(&server, launch.parent_pid)?;

    let session = session_loop(server, launch, &mut applied, &journal_path, &rx).await;

    // Whatever ended the session — clean shutdown, disconnect, parent death,
    // or an error — anything we changed goes back.
    revert_all(&mut applied, &journal_path);

    session
}

/// Confirm the connected client is the exact process that triggered elevation.
fn verify_client(server: &NamedPipeServer, expected_pid: u32) -> Result<(), HelperError> {
    use std::os::windows::io::AsRawHandle;

    let handle = HANDLE(server.as_raw_handle());
    let mut pid = 0u32;

    // SAFETY: `handle` is the live, connected pipe owned by `server`.
    unsafe {
        GetNamedPipeClientProcessId(handle, &mut pid)
            .map_err(|e| HelperError::Pipe(format!("reading the client PID: {e}")))?;
    }

    if pid != expected_pid {
        return Err(HelperError::Rejected(format!(
            "client PID {pid} is not the process that requested elevation ({expected_pid})"
        )));
    }

    Ok(())
}

async fn session_loop(
    mut pipe: NamedPipeServer,
    launch: &Launch,
    applied: &mut Applied,
    journal_path: &Path,
    parent_died: &std::sync::mpsc::Receiver<()>,
) -> Result<(), HelperError> {
    let mut greeted = false;

    loop {
        if parent_died.try_recv().is_ok() {
            tracing::info!("parent died; ending session");
            return Ok(());
        }

        let request: Request = match read_frame(&mut pipe).await {
            Ok(Some(request)) => request,
            // A closed pipe is the normal end of a session.
            Ok(None) => return Ok(()),
            Err(err) => return Err(err),
        };

        // Nothing is acted on before a valid handshake.
        if !greeted {
            match &request {
                Request::Hello {
                    protocol_version,
                    nonce,
                } => {
                    if *protocol_version != PROTOCOL_VERSION {
                        write_frame(
                            &mut pipe,
                            &Response::Error(ProtocolError::VersionMismatch {
                                helper: PROTOCOL_VERSION,
                                client: *protocol_version,
                            }),
                        )
                        .await?;
                        return Err(HelperError::Rejected("protocol version mismatch".into()));
                    }

                    if nonce != &launch.nonce {
                        write_frame(&mut pipe, &Response::Error(ProtocolError::Unauthenticated))
                            .await?;
                        return Err(HelperError::Rejected("nonce mismatch".into()));
                    }

                    greeted = true;
                    write_frame(
                        &mut pipe,
                        &Response::Welcome {
                            helper_version: env!("CARGO_PKG_VERSION").to_owned(),
                        },
                    )
                    .await?;
                    continue;
                }
                _ => {
                    write_frame(
                        &mut pipe,
                        &Response::Error(ProtocolError::HandshakeRequired),
                    )
                    .await?;
                    return Err(HelperError::Rejected("no handshake".into()));
                }
            }
        }

        if matches!(request, Request::Shutdown) {
            write_frame(&mut pipe, &Response::Ok).await?;
            return Ok(());
        }

        let response = handle(request, applied, journal_path);
        write_frame(&mut pipe, &response).await?;
    }
}

fn handle(request: Request, applied: &mut Applied, journal_path: &Path) -> Response {
    match request {
        Request::Ping => Response::Pong,

        Request::SetDns {
            luid,
            family,
            servers,
        } => match resolve_guid(luid) {
            Err(response) => response,
            Ok(guid) => {
                // Validate against live state before touching anything: a
                // compromised client must not be able to aim this at an
                // adapter that does not exist or at a bogus address.
                if servers.is_empty() || servers.len() > 4 {
                    return Response::Error(ProtocolError::Other(
                        "expected between one and four servers".into(),
                    ));
                }
                if servers
                    .iter()
                    .any(|ip| ip.is_multicast() || ip.is_unspecified())
                {
                    return Response::Error(ProtocolError::Other(
                        "multicast and unspecified addresses are not valid resolvers".into(),
                    ));
                }

                let previous = match adapters::dns_config(&guid, family) {
                    Ok(config) => config,
                    Err(err) => return Response::Error(ProtocolError::Os(err.to_string())),
                };

                remember(applied, journal_path, &guid, family, previous.clone());

                match dns_settings::set_dns(&guid, family, &servers) {
                    Ok(()) => Response::Applied {
                        previous: snapshot(luid, family, &previous),
                    },
                    Err(err) => Response::Error(ProtocolError::Os(err.to_string())),
                }
            }
        },

        Request::ClearDns { luid, family } => match resolve_guid(luid) {
            Err(response) => response,
            Ok(guid) => {
                let previous = adapters::dns_config(&guid, family).unwrap_or(DnsConfig::Dhcp);
                remember(applied, journal_path, &guid, family, previous.clone());

                match dns_settings::clear_dns(&guid, family) {
                    Ok(()) => Response::Applied {
                        previous: snapshot(luid, family, &previous),
                    },
                    Err(err) => Response::Error(ProtocolError::Os(err.to_string())),
                }
            }
        },

        Request::Restore { snapshot } => match resolve_guid(snapshot.luid) {
            Err(response) => response,
            Ok(guid) => {
                let previous = if snapshot.from_dhcp {
                    DnsConfig::Dhcp
                } else {
                    DnsConfig::Static(snapshot.servers.clone())
                };

                match dns_settings::restore(&guid, snapshot.family, &previous) {
                    Ok(()) => {
                        applied.remove(&(guid.clone(), snapshot.family));
                        persist(applied, journal_path);
                        Response::Ok
                    }
                    Err(err) => Response::Error(ProtocolError::Os(err.to_string())),
                }
            }
        },

        Request::RestoreAll => {
            revert_all(applied, journal_path);
            Response::Ok
        }

        Request::FlushCache => Response::Ok,

        // Handled by the session loop.
        Request::Hello { .. } | Request::Shutdown => Response::Ok,
    }
}

/// Map a LUID to an adapter GUID by consulting live state.
///
/// Never trust the client's idea of which adapter exists — this is the
/// validation step that keeps the privileged surface honest.
fn resolve_guid(luid: InterfaceLuid) -> Result<String, Response> {
    let adapters =
        adapters::list().map_err(|err| Response::Error(ProtocolError::Os(err.to_string())))?;

    adapters
        .into_iter()
        .find(|a| a.luid == luid)
        .map(|a| a.guid)
        .ok_or(Response::Error(ProtocolError::UnknownInterface(luid)))
}

fn snapshot(luid: InterfaceLuid, family: Family, previous: &DnsConfig) -> DnsSnapshot {
    DnsSnapshot {
        luid,
        family,
        from_dhcp: previous.is_dhcp(),
        servers: previous.servers().to_vec(),
    }
}

fn remember(
    applied: &mut Applied,
    journal_path: &Path,
    guid: &str,
    family: Family,
    previous: DnsConfig,
) {
    // Only the first observation is the user's real configuration.
    applied.entry((guid.to_owned(), family)).or_insert(previous);
    persist(applied, journal_path);
}

fn persist(applied: &Applied, journal_path: &Path) {
    let mut record = journal::Journal::default();
    for ((guid, family), previous) in applied {
        record.record(guid, *family, previous.clone());
    }

    if let Err(err) = journal::save(journal_path, &record) {
        // Not fatal: the in-memory revert and the parent watchdog still work.
        // Only a hard crash would need the journal, and refusing to change DNS
        // because we could not write a file would be worse.
        tracing::warn!("could not write the revert journal: {err}");
    }
}

fn revert_all(applied: &mut Applied, journal_path: &Path) {
    for ((guid, family), previous) in applied.iter() {
        match dns_settings::restore(guid, *family, previous) {
            Ok(()) => tracing::info!(%guid, ?family, "reverted"),
            Err(err) => tracing::error!(%guid, ?family, "revert failed: {err}"),
        }
    }

    applied.clear();
    if let Err(err) = journal::clear(journal_path) {
        tracing::warn!("could not clear the revert journal: {err}");
    }
}

async fn read_frame(pipe: &mut NamedPipeServer) -> Result<Option<Request>, HelperError> {
    let mut length = [0u8; 4];
    match pipe.read_exact(&mut length).await {
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(err) if err.raw_os_error() == Some(109) => return Ok(None), // ERROR_BROKEN_PIPE
        Err(err) => return Err(HelperError::Io(err)),
    }

    let size = u32::from_le_bytes(length) as usize;
    if size > MAX_FRAME_BYTES {
        return Err(HelperError::Rejected(format!(
            "frame of {size} bytes exceeds the limit"
        )));
    }

    let mut body = vec![0u8; size];
    pipe.read_exact(&mut body).await?;

    decode(&body)
        .map(Some)
        .map_err(|err| HelperError::Rejected(format!("malformed frame: {err}")))
}

async fn write_frame(pipe: &mut NamedPipeServer, response: &Response) -> Result<(), HelperError> {
    let frame =
        encode(response).map_err(|err| HelperError::Pipe(format!("encoding a response: {err}")))?;
    pipe.write_all(&frame).await?;
    pipe.flush().await?;
    Ok(())
}
