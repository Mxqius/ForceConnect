//! Pipe client and session state.

use std::sync::Arc;
use std::time::Duration;

use forceconnect_ipc::{
    Launch, MAX_FRAME_BYTES, PROTOCOL_VERSION, Request, Response, decode, encode,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient};
use tokio::sync::Mutex;

use crate::elevate::{ElevationError, launcher};

/// A connected, authenticated helper session.
///
/// Held behind a mutex because the protocol is strictly request/response over a
/// single byte stream: two concurrent callers would interleave frames.
pub struct HelperSession {
    pipe: Mutex<NamedPipeClient>,
    pub helper_version: String,
    pub pid: u32,
}

impl HelperSession {
    /// Launch the helper, connect, and complete the handshake.
    pub async fn establish(
        helper_path: std::path::PathBuf,
        parent_hwnd: isize,
    ) -> Result<Arc<Self>, ElevationError> {
        let nonce = super::generate_nonce();
        let launch = Launch {
            pipe: super::pipe_name(&nonce),
            nonce,
            parent_pid: std::process::id(),
            parent_start: own_creation_time()?,
        };

        // ShellExecuteExW blocks on the modal UAC prompt.
        let spawn = {
            let launch = launch.clone();
            let path = helper_path.clone();
            tokio::task::spawn_blocking(move || launcher::launch(&path, &launch, parent_hwnd))
                .await
                .map_err(|err| ElevationError::Launch(err.to_string()))?
        };
        let pid = spawn?;

        let pipe = connect_with_retry(&launch.pipe).await?;
        let session = Self {
            pipe: Mutex::new(pipe),
            helper_version: String::new(),
            pid,
        };

        let response = session
            .request(Request::Hello {
                protocol_version: PROTOCOL_VERSION,
                nonce: launch.nonce.clone(),
            })
            .await?;

        let helper_version = match response {
            Response::Welcome { helper_version } => helper_version,
            Response::Error(err) => return Err(ElevationError::Rejected(err.to_string())),
            other => {
                return Err(ElevationError::Rejected(format!(
                    "unexpected handshake reply: {other:?}"
                )));
            }
        };

        Ok(Arc::new(Self {
            helper_version,
            ..session
        }))
    }

    /// Send one request and await its reply.
    pub async fn request(&self, request: Request) -> Result<Response, ElevationError> {
        let frame =
            encode(&request).map_err(|err| ElevationError::Connect(format!("encoding: {err}")))?;

        let mut pipe = self.pipe.lock().await;
        pipe.write_all(&frame).await?;
        pipe.flush().await?;

        let mut length = [0u8; 4];
        pipe.read_exact(&mut length).await?;

        let size = u32::from_le_bytes(length) as usize;
        if size > MAX_FRAME_BYTES {
            return Err(ElevationError::Connect(format!(
                "the helper sent an oversized frame of {size} bytes"
            )));
        }

        let mut body = vec![0u8; size];
        pipe.read_exact(&mut body).await?;

        decode(&body).map_err(|err| ElevationError::Connect(format!("decoding: {err}")))
    }

    /// Convenience wrapper that turns a protocol-level error into a Rust error.
    pub async fn call(&self, request: Request) -> Result<Response, ElevationError> {
        match self.request(request).await? {
            Response::Error(err) => Err(ElevationError::Remote(err.to_string())),
            other => Ok(other),
        }
    }
}

/// Connect, tolerating the window between spawning the helper and it creating
/// the pipe — which includes however long the user spends at the UAC prompt.
async fn connect_with_retry(name: &str) -> Result<NamedPipeClient, ElevationError> {
    const ATTEMPTS: u32 = 60;
    const DELAY: Duration = Duration::from_millis(250);

    let mut last = String::from("no attempt was made");

    for _ in 0..ATTEMPTS {
        match ClientOptions::new().open(name) {
            Ok(client) => return Ok(client),
            Err(err) => {
                last = err.to_string();
                tokio::time::sleep(DELAY).await;
            }
        }
    }

    Err(ElevationError::Connect(format!(
        "the helper never opened {name}: {last}"
    )))
}

/// Our own process creation time, passed to the helper so it can pin us and
/// reject a recycled PID.
#[cfg(windows)]
fn own_creation_time() -> Result<u64, ElevationError> {
    use windows::Win32::Foundation::FILETIME;
    use windows::Win32::System::Threading::{GetCurrentProcess, GetProcessTimes};

    let mut created = FILETIME::default();
    let mut exited = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();

    // SAFETY: the pseudo-handle from GetCurrentProcess needs no cleanup, and
    // all out-parameters are valid zeroed structs.
    unsafe {
        GetProcessTimes(
            GetCurrentProcess(),
            &mut created,
            &mut exited,
            &mut kernel,
            &mut user,
        )
        .map_err(|err| ElevationError::Launch(format!("reading our own start time: {err}")))?;
    }

    Ok(((created.dwHighDateTime as u64) << 32) | created.dwLowDateTime as u64)
}

#[cfg(not(windows))]
fn own_creation_time() -> Result<u64, ElevationError> {
    Ok(0)
}
