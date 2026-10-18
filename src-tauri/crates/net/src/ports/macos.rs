//! Port ownership on macOS, from `lsof`.
//!
//! macOS has no procfs, and the `proc_pidinfo` socket walk `lsof` performs is
//! private API. Unprivileged, `lsof` only sees this user's sockets; the
//! privileged agent sees all of them.

use super::PortOwner;
use crate::macos::parse_lsof;
use crate::unix;

/// Who, if anyone, holds the given UDP port on an address that would keep the
/// forwarder off loopback.
pub fn udp_port_owner(port: u16) -> Option<PortOwner> {
    // `lsof` exits 1 when nothing matches, which is the common case.
    let selector = format!("-iUDP:{port}");
    let body = unix::run("lsof", &["-nP", &selector, "-Fpcn"]).ok()?;
    let (pid, command) = parse_lsof(&body, port)?;

    Some(PortOwner {
        pid,
        process: Some(command),
        service: None,
    })
}
