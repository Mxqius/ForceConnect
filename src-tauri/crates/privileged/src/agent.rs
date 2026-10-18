//! The privileged agent: how ForceConnect holds administrator rights on Linux
//! and macOS.
//!
//! On Windows the whole application runs elevated, because Windows offers a
//! plain desktop app exactly one way to ask: a manifest, honoured once at
//! launch. Neither of the other platforms has that, and both actively
//! discourage the equivalent — a GUI running as root. On Linux it does not
//! even work: a root process cannot reach the user's session bus, which the
//! tray icon and single-instance handling both need, and Wayland compositors
//! refuse root clients outright.
//!
//! So there, the interface runs as the user, and at launch it starts *the same
//! binary* a second time, as root, through the platform's own administrator
//! prompt — `pkexec` on Linux, `osascript … with administrator privileges` on
//! macOS. That process is the agent. It serves this crate's verbs and nothing
//! else, and lives exactly as long as the interface does.
//!
//! # What this is not
//!
//! It is not the named-pipe helper the Windows app used to have. That design
//! was retired because of costs that came from Windows itself: a pipe anyone
//! can squat, so a nonce and a DACL; a caller that cannot be authenticated, so
//! an integrity label and caller pinning; a second UAC prompt mid-connect.
//! Here the connection is made the other way round. The interface listens on a
//! socket in a directory only the user can enter, the agent connects to it,
//! and the kernel reports the peer's uid — so the interface knows it is
//! talking to root, and there is nothing for anyone else to connect to. The
//! prompt still happens once, at launch.
//!
//! # The protocol
//!
//! One JSON [`Request`] per line in, one JSON [`Response`] per line out, over
//! a connection only the two processes hold. The verbs are the crate's own —
//! set DNS, clear DNS, flush the cache, restore, recover the journal — and
//! every argument is validated on the agent's side, against live system
//! state, exactly as it is when the verbs are called in-process. The
//! interface is not trusted any more than the frontend is.
//!
//! When the connection closes for any reason — a clean quit, a crash, a
//! `kill -9` — the agent restores whatever it still owes before it exits. That
//! is a stronger guarantee than the in-process design can give, where a hard
//! crash leaves the restore to the next launch.

use std::io::{self, BufRead, Read, Write};
use std::net::IpAddr;

use forceconnect_net::{DnsConfig, Family, InterfaceLuid};
use serde::{Deserialize, Serialize};

use crate::{DnsWriter, PrivilegedError, ReleasePolicy};

/// The argument that turns the application binary into the agent.
pub const AGENT_FLAG: &str = "--privileged-agent";

/// Longest request line the agent reads. A real one is well under a kilobyte;
/// this only stops a runaway line from being buffered without bound.
const MAX_LINE: u64 = 64 * 1024;

/// One call into the privileged surface. Nothing else can be expressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "verb", rename_all = "snake_case")]
pub enum Request {
    SetDns {
        luid: InterfaceLuid,
        servers: Vec<IpAddr>,
    },
    ClearDns {
        luid: InterfaceLuid,
    },
    FlushCache,
    RestoreAll,
    /// Replay the journal a previous run left behind. Sent once, by the
    /// interface, after the single-instance check — never on the agent's own
    /// initiative, or a second launch would revert the first one's changes.
    Recover,
    HasPending,
    /// Snapshot every adapter's DNS as the "before ForceConnect" state. Sent
    /// once, straight after `Recover`.
    CaptureBaseline,
    Baseline,
    SetPolicy {
        policy: ReleasePolicy,
    },
    /// Disconnect: put one adapter where the release policy says.
    Release {
        luid: InterfaceLuid,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reply {
    Done,
    Recovered(usize),
    Pending(bool),
    Baseline(Vec<(String, Family, DnsConfig)>),
    Released(Vec<(Family, DnsConfig)>),
}

pub type Response = Result<Reply, PrivilegedError>;

/// What the agent does with a request.
///
/// Implemented by [`DnsWriter`]; a trait so the protocol can be tested without
/// touching a real adapter.
pub trait Verbs {
    fn set_dns(&mut self, luid: InterfaceLuid, servers: &[IpAddr]) -> Result<(), PrivilegedError>;
    fn clear_dns(&mut self, luid: InterfaceLuid) -> Result<(), PrivilegedError>;
    fn flush_cache(&self) -> Result<(), PrivilegedError>;
    fn restore_all(&mut self);
    fn recover(&mut self) -> usize;
    fn has_pending(&self) -> bool;
    fn capture_baseline(&mut self);
    fn baseline(&self) -> Vec<(String, Family, DnsConfig)>;
    fn set_policy(&mut self, policy: ReleasePolicy) -> Result<(), PrivilegedError>;
    fn release(&mut self, luid: InterfaceLuid) -> Result<Vec<(Family, DnsConfig)>, PrivilegedError>;
}

impl Verbs for DnsWriter {
    fn set_dns(&mut self, luid: InterfaceLuid, servers: &[IpAddr]) -> Result<(), PrivilegedError> {
        DnsWriter::set_dns(self, luid, servers)
    }

    fn clear_dns(&mut self, luid: InterfaceLuid) -> Result<(), PrivilegedError> {
        DnsWriter::clear_dns(self, luid)
    }

    fn flush_cache(&self) -> Result<(), PrivilegedError> {
        DnsWriter::flush_cache(self)
    }

    fn restore_all(&mut self) {
        DnsWriter::restore_all(self);
    }

    fn recover(&mut self) -> usize {
        crate::recover(self.journal_path())
    }

    fn has_pending(&self) -> bool {
        DnsWriter::has_pending(self)
    }

    fn capture_baseline(&mut self) {
        DnsWriter::capture_baseline(self);
    }

    fn baseline(&self) -> Vec<(String, Family, DnsConfig)> {
        self.baseline_entries()
    }

    fn set_policy(&mut self, policy: ReleasePolicy) -> Result<(), PrivilegedError> {
        DnsWriter::set_policy(self, policy)
    }

    fn release(&mut self, luid: InterfaceLuid) -> Result<Vec<(Family, DnsConfig)>, PrivilegedError> {
        DnsWriter::release(self, luid)
    }
}

/// Carry out one request.
pub fn handle(verbs: &mut impl Verbs, request: Request) -> Response {
    match request {
        Request::SetDns { luid, servers } => verbs.set_dns(luid, &servers).map(|()| Reply::Done),
        Request::ClearDns { luid } => verbs.clear_dns(luid).map(|()| Reply::Done),
        Request::FlushCache => verbs.flush_cache().map(|()| Reply::Done),
        Request::RestoreAll => {
            verbs.restore_all();
            Ok(Reply::Done)
        }
        Request::Recover => Ok(Reply::Recovered(verbs.recover())),
        Request::HasPending => Ok(Reply::Pending(verbs.has_pending())),
        Request::CaptureBaseline => {
            verbs.capture_baseline();
            Ok(Reply::Done)
        }
        Request::Baseline => Ok(Reply::Baseline(verbs.baseline())),
        Request::SetPolicy { policy } => verbs.set_policy(policy).map(|()| Reply::Done),
        Request::Release { luid } => verbs.release(luid).map(Reply::Released),
    }
}

/// Serve requests until the connection closes, then put back anything still
/// owed.
///
/// The restore runs however the loop ends — end of stream, a read error, an
/// oversized line — because every one of those means the interface is gone,
/// and an adapter left pointing at a resolver nobody is running is the failure
/// this whole crate exists to prevent.
pub fn serve(verbs: &mut impl Verbs, reader: impl BufRead, writer: impl Write) {
    if let Err(err) = serve_until_closed(verbs, reader, writer) {
        tracing::debug!("agent connection ended: {err}");
    }

    // Only when something is owed: an empty restore still rewrites the
    // journal, and that file may belong to another running instance.
    if verbs.has_pending() {
        verbs.restore_all();
    }
}

fn serve_until_closed(
    verbs: &mut impl Verbs,
    mut reader: impl BufRead,
    mut writer: impl Write,
) -> io::Result<()> {
    let mut line = String::new();

    loop {
        line.clear();
        let read = reader.by_ref().take(MAX_LINE).read_line(&mut line)?;
        if read == 0 {
            return Ok(());
        }
        if !line.ends_with('\n') && read as u64 == MAX_LINE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "request line too long",
            ));
        }

        let response: Response = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle(verbs, request),
            Err(err) => Err(PrivilegedError::Invalid(format!(
                "malformed request: {err}"
            ))),
        };

        serde_json::to_writer(&mut writer, &response)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
}

/// Why a call through the agent did not produce a reply.
#[derive(Debug)]
pub enum CallError {
    /// The agent carried out the request and refused it.
    Refused(PrivilegedError),
    /// The connection is gone. Nothing more can be sent on it.
    Lost(String),
}

/// The interface's end of an agent connection.
pub struct Connection<R, W> {
    reader: R,
    writer: W,
}

impl<R: BufRead, W: Write> Connection<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    /// Send one request and wait for its reply.
    pub fn call(&mut self, request: &Request) -> Result<Reply, CallError> {
        let lost = |err: io::Error| CallError::Lost(err.to_string());

        serde_json::to_writer(&mut self.writer, request).map_err(|e| lost(e.into()))?;
        self.writer.write_all(b"\n").map_err(lost)?;
        self.writer.flush().map_err(lost)?;

        let mut line = String::new();
        if self.reader.read_line(&mut line).map_err(lost)? == 0 {
            return Err(CallError::Lost("the privileged agent exited".into()));
        }

        match serde_json::from_str::<Response>(&line) {
            Ok(Ok(reply)) => Ok(reply),
            Ok(Err(refused)) => Err(CallError::Refused(refused)),
            Err(err) => Err(CallError::Lost(format!("unreadable reply: {err}"))),
        }
    }
}

/// Shell-quote one word for `/bin/sh`: single quotes, with any single quote
/// inside closed, escaped and reopened.
#[cfg(any(target_os = "macos", test))]
fn shell_quote(word: &str) -> String {
    format!("'{}'", word.replace('\'', r"'\''"))
}

/// Escape text for the inside of an AppleScript string literal.
#[cfg(any(target_os = "macos", test))]
fn applescript_string(text: &str) -> String {
    text.replace('\\', r"\\").replace('"', "\\\"")
}

/// The script `osascript` runs to start the agent as root.
///
/// The command is quoted twice over — once for the shell `do shell script`
/// hands it to, then once more for the AppleScript string it travels in —
/// because both paths come from the filesystem and either could contain a
/// quote. Output goes to `/dev/null`: `do shell script` would otherwise hold
/// it all in memory for the agent's entire life.
#[cfg(any(target_os = "macos", test))]
fn elevation_script(exe: &str, socket: &str) -> String {
    const PROMPT: &str = "ForceConnect needs administrator rights to change DNS settings.";

    let command = format!(
        "{} {AGENT_FLAG} {} >/dev/null 2>&1",
        shell_quote(exe),
        shell_quote(socket)
    );
    format!(
        "do shell script \"{}\" with prompt \"{}\" with administrator privileges",
        applescript_string(&command),
        applescript_string(PROMPT)
    )
}

#[cfg(unix)]
pub use unix::{AgentConnection, launch, run_if_requested};

#[cfg(unix)]
mod unix {
    use std::io::BufReader;
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::DirBuilderExt;
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::{Path, PathBuf};
    use std::process::{Child, Command, Stdio};
    use std::time::Duration;

    use super::{AGENT_FLAG, Connection};
    use crate::DnsWriter;

    pub type AgentConnection = Connection<BufReader<UnixStream>, UnixStream>;

    /// Become the agent, if that is what this process was started as.
    ///
    /// Returns the exit code when it was, so `main` can exit before anything
    /// of the GUI is initialised; `None` means carry on as the application.
    pub fn run_if_requested() -> Option<i32> {
        let mut args = std::env::args_os().skip(1);
        if args.next()? != AGENT_FLAG {
            return None;
        }
        let Some(socket) = args.next() else {
            eprintln!("{AGENT_FLAG} needs the socket path to connect to");
            return Some(2);
        };

        if !crate::is_elevated() {
            eprintln!("{AGENT_FLAG} must be started as root");
            return Some(1);
        }

        let stream = match UnixStream::connect(&socket) {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!(
                    "could not connect to {}: {err}",
                    Path::new(&socket).display()
                );
                return Some(1);
            }
        };
        let reader = match stream.try_clone() {
            Ok(clone) => BufReader::new(clone),
            Err(err) => {
                eprintln!("could not duplicate the agent socket: {err}");
                return Some(1);
            }
        };

        let mut writer = DnsWriter::default();
        super::serve(&mut writer, reader, stream);
        Some(0)
    }

    /// Start the agent through the platform's administrator prompt and wait
    /// for it to connect.
    ///
    /// Blocks for as long as the prompt is up. Returns an error if the user
    /// declines, if there is no way to ask (no polkit agent in the session),
    /// or if whatever connects is not root.
    pub fn launch() -> Result<AgentConnection, String> {
        let dir =
            private_dir().map_err(|err| format!("could not create a socket directory: {err}"))?;
        let result = launch_in(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        result
    }

    fn launch_in(dir: &Path) -> Result<AgentConnection, String> {
        let socket = dir.join("a.sock");
        let listener =
            UnixListener::bind(&socket).map_err(|err| format!("could not listen: {err}"))?;
        listener
            .set_nonblocking(true)
            .map_err(|err| err.to_string())?;

        let exe = agent_executable()?;
        let mut child = elevate(&exe, &socket)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| format!("could not ask for administrator rights: {err}"))?;

        let stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
                        return Err(declined(status, &mut child));
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(err) => {
                    let _ = child.kill();
                    return Err(format!("the agent could not connect: {err}"));
                }
            }
        };

        // Inherited from the listener on macOS, where accepted sockets keep
        // the listening socket's flags. Linux does not, but saying so costs
        // nothing.
        stream
            .set_nonblocking(false)
            .map_err(|err| err.to_string())?;

        match peer_uid(&stream) {
            Some(0) => {}
            other => {
                let _ = child.kill();
                return Err(format!(
                    "refusing an agent connection from uid {other:?}; only root may connect"
                ));
            }
        }

        // The child — pkexec, or osascript and its shell — lives as long as
        // the agent does. Reap it whenever that is.
        std::thread::spawn(move || {
            let _ = child.wait();
        });

        let reader = stream.try_clone().map_err(|err| err.to_string())?;
        Ok(Connection::new(BufReader::new(reader), stream))
    }

    /// A fresh directory only this user can enter, for the socket.
    ///
    /// Short on purpose: a socket path is limited to about 104 bytes on macOS,
    /// and `$TMPDIR` there already spends half of it.
    fn private_dir() -> std::io::Result<PathBuf> {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or_default();
        let dir = std::env::temp_dir().join(format!("fc-{}-{nonce:x}", std::process::id()));

        std::fs::DirBuilder::new().mode(0o700).create(&dir)?;
        Ok(dir)
    }

    /// What to start as root.
    ///
    /// Inside an AppImage the running executable lives on a FUSE mount that
    /// root is not allowed to read, so the agent is started from the AppImage
    /// file itself, which the runtime exports as `$APPIMAGE`.
    fn agent_executable() -> Result<PathBuf, String> {
        #[cfg(target_os = "linux")]
        if let Some(appimage) = std::env::var_os("APPIMAGE").map(PathBuf::from)
            && appimage.is_absolute()
            && appimage.is_file()
        {
            return Ok(appimage);
        }

        std::env::current_exe().map_err(|err| format!("could not locate this executable: {err}"))
    }

    #[cfg(target_os = "linux")]
    fn elevate(exe: &Path, socket: &Path) -> Command {
        // pkexec authenticates through the desktop's polkit agent, then execs
        // the program directly: no shell, so nothing here is quoted.
        let pkexec = crate::exec::locate("pkexec").unwrap_or_else(|| PathBuf::from("pkexec"));
        let mut command = Command::new(pkexec);
        command.arg(exe).arg(AGENT_FLAG).arg(socket);
        command
    }

    #[cfg(target_os = "macos")]
    fn elevate(exe: &Path, socket: &Path) -> Command {
        let script = super::elevation_script(&exe.to_string_lossy(), &socket.to_string_lossy());
        let mut command = Command::new("/usr/bin/osascript");
        command.arg("-e").arg(script);
        command
    }

    /// Say why the prompt ended without an agent.
    fn declined(status: std::process::ExitStatus, child: &mut Child) -> String {
        let mut stderr = String::new();
        if let Some(mut pipe) = child.stderr.take() {
            let _ = std::io::Read::read_to_string(&mut pipe, &mut stderr);
        }

        // pkexec: 126 is a dismissed dialog, 127 is "not authorized" or no
        // authentication agent. osascript: -128 is the user pressing Cancel.
        let declined_by_user = matches!(status.code(), Some(126)) || stderr.contains("(-128)");
        if declined_by_user {
            "administrator rights were not granted".into()
        } else {
            format!(
                "administrator rights could not be requested ({status}): {}",
                stderr.trim()
            )
        }
    }

    /// The uid of the process at the other end of a Unix socket, as the
    /// kernel reports it.
    #[cfg(target_os = "linux")]
    fn peer_uid(stream: &UnixStream) -> Option<u32> {
        let mut cred = libc::ucred {
            pid: 0,
            uid: u32::MAX,
            gid: u32::MAX,
        };
        let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;

        // SAFETY: `cred` and `len` are valid for writes of the sizes given,
        // and the descriptor is a live socket for the duration of the call.
        let rc = unsafe {
            libc::getsockopt(
                stream.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_PEERCRED,
                (&raw mut cred).cast(),
                &mut len,
            )
        };
        (rc == 0).then_some(cred.uid)
    }

    #[cfg(target_os = "macos")]
    fn peer_uid(stream: &UnixStream) -> Option<u32> {
        let mut uid: libc::uid_t = u32::MAX;
        let mut gid: libc::gid_t = u32::MAX;

        // SAFETY: both out-parameters are valid, and the descriptor is a live
        // connected socket for the duration of the call.
        let rc = unsafe { libc::getpeereid(stream.as_raw_fd(), &mut uid, &mut gid) };
        (rc == 0).then_some(uid)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    /// Records what reached it and answers like a writer that is not elevated
    /// for one adapter.
    #[derive(Default)]
    struct Fake {
        calls: Vec<String>,
        pending: bool,
        restored: bool,
    }

    impl Verbs for Fake {
        fn set_dns(
            &mut self,
            luid: InterfaceLuid,
            servers: &[IpAddr],
        ) -> Result<(), PrivilegedError> {
            self.calls.push(format!("set {} {servers:?}", luid.0));
            if luid.0 == 13 {
                return Err(PrivilegedError::UnknownInterface(luid));
            }
            self.pending = true;
            Ok(())
        }

        fn clear_dns(&mut self, luid: InterfaceLuid) -> Result<(), PrivilegedError> {
            self.calls.push(format!("clear {}", luid.0));
            Ok(())
        }

        fn flush_cache(&self) -> Result<(), PrivilegedError> {
            Ok(())
        }

        fn restore_all(&mut self) {
            self.restored = true;
            self.pending = false;
        }

        fn recover(&mut self) -> usize {
            2
        }

        fn has_pending(&self) -> bool {
            self.pending
        }

        fn capture_baseline(&mut self) {}

        fn baseline(&self) -> Vec<(String, Family, DnsConfig)> {
            vec![("eth0".into(), Family::V4, DnsConfig::Dhcp)]
        }

        fn set_policy(&mut self, policy: ReleasePolicy) -> Result<(), PrivilegedError> {
            self.calls.push(format!("policy {policy:?}"));
            Ok(())
        }

        fn release(
            &mut self,
            luid: InterfaceLuid,
        ) -> Result<Vec<(Family, DnsConfig)>, PrivilegedError> {
            self.calls.push(format!("release {}", luid.0));
            self.pending = false;
            Ok(vec![(Family::V4, DnsConfig::Dhcp)])
        }
    }

    fn line(request: &Request) -> String {
        serde_json::to_string(request).unwrap() + "\n"
    }

    fn responses(output: &[u8]) -> Vec<Response> {
        String::from_utf8_lossy(output)
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect()
    }

    #[test]
    fn requests_are_carried_out_and_answered_in_order() {
        let input = [
            line(&Request::SetDns {
                luid: InterfaceLuid(7),
                servers: vec!["1.1.1.1".parse().unwrap()],
            }),
            line(&Request::HasPending),
            line(&Request::Recover),
            line(&Request::SetDns {
                luid: InterfaceLuid(13),
                servers: vec!["1.1.1.1".parse().unwrap()],
            }),
        ]
        .concat();

        let mut fake = Fake::default();
        let mut output = Vec::new();
        serve(&mut fake, Cursor::new(input), &mut output);

        assert_eq!(
            responses(&output),
            vec![
                Ok(Reply::Done),
                Ok(Reply::Pending(true)),
                Ok(Reply::Recovered(2)),
                // A refusal travels back as the same error, not a string.
                Err(PrivilegedError::UnknownInterface(InterfaceLuid(13))),
            ]
        );
    }

    #[test]
    fn the_release_policy_and_release_cross_the_connection() {
        // The disconnect destination is decided on the agent's side, so the
        // policy has to reach it intact and the outcome has to come back.
        let input = [
            line(&Request::SetPolicy {
                policy: ReleasePolicy::Fallback(vec!["9.9.9.9".parse().unwrap()]),
            }),
            line(&Request::Release {
                luid: InterfaceLuid(7),
            }),
            line(&Request::Baseline),
        ]
        .concat();

        let mut fake = Fake::default();
        let mut output = Vec::new();
        serve(&mut fake, Cursor::new(input), &mut output);

        assert_eq!(
            responses(&output),
            vec![
                Ok(Reply::Done),
                Ok(Reply::Released(vec![(Family::V4, DnsConfig::Dhcp)])),
                Ok(Reply::Baseline(vec![(
                    "eth0".into(),
                    Family::V4,
                    DnsConfig::Dhcp
                )])),
            ]
        );
        assert_eq!(fake.calls, ["policy Fallback([9.9.9.9])", "release 7"]);
    }

    #[test]
    fn closing_the_connection_restores_what_is_owed() {
        // The interface crashing mid-session is exactly this: a set, then the
        // stream ends with nothing else said.
        let input = line(&Request::SetDns {
            luid: InterfaceLuid(7),
            servers: vec!["9.9.9.9".parse().unwrap()],
        });

        let mut fake = Fake::default();
        serve(&mut fake, Cursor::new(input), Vec::new());
        assert!(fake.restored);
    }

    #[test]
    fn closing_with_nothing_owed_touches_nothing() {
        // A restore with nothing pending still rewrites the journal — which,
        // during a second launch, belongs to the first instance.
        let mut fake = Fake::default();
        serve(
            &mut fake,
            Cursor::new(line(&Request::HasPending)),
            Vec::new(),
        );
        assert!(!fake.restored);
    }

    #[test]
    fn anything_that_is_not_a_verb_is_refused() {
        let input = "{\"verb\":\"run_command\",\"command\":\"rm -rf /\"}\nnot json\n";

        let mut fake = Fake::default();
        let mut output = Vec::new();
        serve(&mut fake, Cursor::new(input), &mut output);

        let replies = responses(&output);
        assert_eq!(replies.len(), 2);
        assert!(
            replies
                .iter()
                .all(|r| matches!(r, Err(PrivilegedError::Invalid(_))))
        );
        assert!(fake.calls.is_empty());
    }

    #[test]
    fn an_oversized_line_ends_the_session() {
        let input = "x".repeat(MAX_LINE as usize + 10);

        let mut fake = Fake {
            pending: true,
            ..Fake::default()
        };
        let mut output = Vec::new();
        serve(&mut fake, Cursor::new(input), &mut output);

        assert!(output.is_empty());
        assert!(fake.restored, "the session ending must still restore");
    }

    #[test]
    fn the_client_reads_replies_and_refusals() {
        let replies = [
            serde_json::to_string(&Response::Ok(Reply::Pending(true))).unwrap(),
            serde_json::to_string(&Response::Err(PrivilegedError::NotElevated)).unwrap(),
        ]
        .join("\n")
            + "\n";

        let mut sent = Vec::new();
        let mut connection = Connection::new(Cursor::new(replies), &mut sent);

        assert_eq!(
            connection.call(&Request::HasPending).unwrap(),
            Reply::Pending(true)
        );
        assert!(matches!(
            connection.call(&Request::FlushCache),
            Err(CallError::Refused(PrivilegedError::NotElevated))
        ));
        // Nothing left to read: the agent is gone.
        assert!(matches!(
            connection.call(&Request::FlushCache),
            Err(CallError::Lost(_))
        ));

        let sent = String::from_utf8(sent).unwrap();
        assert!(sent.starts_with("{\"verb\":\"has_pending\"}\n"), "{sent}");
    }

    #[test]
    fn the_macos_prompt_script_survives_hostile_paths() {
        let script = elevation_script(
            "/Applications/Force\"Connect's.app/Contents/MacOS/forceconnect",
            "/var/folders/x/T/fc-1-a/a.sock",
        );

        // The shell sees one single-quoted word, with the embedded single
        // quote closed and reopened around an escaped one...
        assert!(
            script.contains(r#"'/Applications/Force\"Connect'\\''s.app"#),
            "{script}"
        );
        // ...and AppleScript sees one string: every double quote inside it is
        // escaped, so the literal cannot be closed early.
        let body = script
            .strip_prefix("do shell script \"")
            .and_then(|rest| rest.split_once("\" with prompt"))
            .map(|(body, _)| body)
            .expect("the command is one string literal");
        assert!(!body.replace("\\\"", "").contains('"'), "{body}");
        assert!(script.ends_with("with administrator privileges"));
    }

    #[test]
    fn shell_quoting_round_trips_single_quotes() {
        assert_eq!(shell_quote("plain"), "'plain'");
        assert_eq!(shell_quote("it's"), r"'it'\''s'");
        assert_eq!(applescript_string(r#"a"b\c"#), r#"a\"b\\c"#);
    }
}
