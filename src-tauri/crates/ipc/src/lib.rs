//! Wire protocol between the unelevated ForceConnect UI and its elevated helper.
//!
//! The two processes talk over a named pipe. Everything that requires
//! administrator rights lives behind this protocol, which keeps the privileged
//! surface small enough to audit in one sitting — a deliberate goal, since the
//! helper is the only part of the app that runs elevated.
//!
//! # The helper is write-only
//!
//! Every *read* — enumerating adapters, reading current DNS settings, route
//! lookups, change notifications — is unprivileged and stays in the UI process.
//! The helper exists solely to perform writes the user could already perform
//! through Windows Settings. Two rules follow, and both are load-bearing:
//!
//! 1. No general-purpose escape hatch. There is no `RunCommand`, no `WriteFile`,
//!    no `WriteRegistry { path, value }`. The absence of one *is* the security
//!    boundary: same-user processes are not a boundary on Windows, so a fully
//!    compromised UI process must still be unable to do more than the UI itself
//!    offers.
//! 2. Every argument is validated against live system state before it is acted
//!    on — an adapter LUID must match a currently present adapter, and server
//!    lists are length- and family-checked.
//!
//! Note that autostart is deliberately absent: it writes to `HKCU\…\Run`, which
//! the unelevated process can do itself.

pub mod launch;

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

pub use launch::{Launch, LaunchArgsError};

/// Bumped whenever `Request` or `Response` changes shape. The helper refuses a
/// client that does not match, so a stale helper left over from a previous
/// version can never act on a message it would misinterpret.
pub const PROTOCOL_VERSION: u32 = 1;

/// Base name of the pipe. The full path is suffixed with the session nonce so
/// two instances (or a stale helper) cannot collide.
pub const PIPE_PREFIX: &str = r"\\.\pipe\forceconnect";

/// Largest frame we will read. Requests are tiny; anything larger is either a
/// bug or someone probing the pipe.
pub const MAX_FRAME_BYTES: usize = 64 * 1024;

/// Which address family a DNS change applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Family {
    V4,
    V6,
}

/// Identifies an adapter by LUID rather than by name. Friendly names are
/// localized, user-renameable, and may contain spaces — all of which broke the
/// previous `netsh`-based implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceLuid(pub u64);

/// What an adapter's DNS configuration looked like before we touched it, so it
/// can be restored exactly — including whether it was DHCP-assigned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsSnapshot {
    pub luid: InterfaceLuid,
    pub family: Family,
    /// `true` when the adapter was taking DNS from DHCP. Restoring then means
    /// clearing the static list, not writing these addresses back.
    pub from_dhcp: bool,
    pub servers: Vec<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    /// First message on every connection. Carries the nonce the UI passed to
    /// the helper on its command line, proving the caller is the process that
    /// spawned it.
    Hello {
        protocol_version: u32,
        nonce: String,
    },
    /// Point an adapter at a fixed list of resolvers.
    SetDns {
        luid: InterfaceLuid,
        family: Family,
        servers: Vec<IpAddr>,
    },
    /// Return an adapter to DHCP-assigned DNS.
    ClearDns {
        luid: InterfaceLuid,
        family: Family,
    },
    /// Restore a previously captured snapshot verbatim.
    Restore {
        snapshot: DnsSnapshot,
    },
    /// Revert every change this helper has applied. Runs on parent death as
    /// well as on request.
    RestoreAll,
    FlushCache,
    /// Keepalive. Also how the UI notices the helper died.
    Ping,
    /// Ask the helper to restore any outstanding changes and exit.
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    /// Handshake accepted.
    Welcome {
        helper_version: String,
    },
    /// The operation succeeded and produced nothing to report.
    Ok,
    /// A DNS change succeeded; the caller should persist this so the previous
    /// state can be restored after a crash.
    Applied {
        previous: DnsSnapshot,
    },
    Pong,
    Error(ProtocolError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum ProtocolError {
    #[error("protocol version mismatch: helper speaks {helper}, client speaks {client}")]
    VersionMismatch { helper: u32, client: u32 },

    #[error("authentication failed")]
    Unauthenticated,

    #[error("expected a Hello message first")]
    HandshakeRequired,

    #[error("no adapter with LUID {0:?}")]
    UnknownInterface(InterfaceLuid),

    #[error("the operating system rejected the change: {0}")]
    Os(String),

    #[error("{0}")]
    Other(String),
}

/// Framing errors, kept separate from protocol-level ones.
#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("frame of {0} bytes exceeds the {MAX_FRAME_BYTES} byte limit")]
    TooLarge(usize),

    #[error("malformed frame: {0}")]
    Malformed(#[from] serde_json::Error),
}

/// Encode a message as a length-prefixed JSON frame.
///
/// JSON rather than a compact binary format on purpose: the volume is a handful
/// of messages per session, and a protocol a reviewer can read off the wire is
/// worth more here than the bytes saved.
pub fn encode<T: Serialize>(message: &T) -> Result<Vec<u8>, FrameError> {
    let body = serde_json::to_vec(message)?;
    if body.len() > MAX_FRAME_BYTES {
        return Err(FrameError::TooLarge(body.len()));
    }

    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(
        &u32::try_from(body.len())
            .expect("checked above")
            .to_le_bytes(),
    );
    frame.extend_from_slice(&body);
    Ok(frame)
}

/// Decode a frame body (the bytes following the 4-byte length prefix).
pub fn decode<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, FrameError> {
    Ok(serde_json::from_slice(body)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn frame_roundtrips() {
        let request = Request::SetDns {
            luid: InterfaceLuid(0x8000_0000_0000_0001),
            family: Family::V4,
            servers: vec![
                IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)),
            ],
        };

        let frame = encode(&request).expect("encodes");
        let len = u32::from_le_bytes(frame[..4].try_into().unwrap()) as usize;
        assert_eq!(len, frame.len() - 4);

        let decoded: Request = decode(&frame[4..]).expect("decodes");
        assert_eq!(decoded, request);
    }

    #[test]
    fn oversized_frames_are_rejected() {
        let oversized = Response::Error(ProtocolError::Other("x".repeat(MAX_FRAME_BYTES)));

        match encode(&oversized) {
            Err(FrameError::TooLarge(n)) => assert!(n > MAX_FRAME_BYTES),
            other => panic!("expected TooLarge, got {other:?}"),
        }
    }

    #[test]
    fn dhcp_snapshots_survive_a_roundtrip() {
        // Restoring must distinguish "was on DHCP" from "had no servers", or
        // disconnecting silently converts an adapter to a static empty list.
        let snapshot = DnsSnapshot {
            luid: InterfaceLuid(42),
            family: Family::V4,
            from_dhcp: true,
            servers: vec![],
        };

        let frame = encode(&snapshot).expect("encodes");
        let decoded: DnsSnapshot = decode(&frame[4..]).expect("decodes");
        assert!(decoded.from_dhcp);
    }
}
