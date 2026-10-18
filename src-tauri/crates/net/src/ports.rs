//! Finding out who already holds a port.
//!
//! "Address already in use" is a useless thing to show a user. Port 53 is
//! commonly taken by Internet Connection Sharing on Windows, by dnsmasq or a
//! Pi-hole-alike on Linux, and by Docker Desktop or a local DNS server role on
//! any of them — each with a different remedy. Naming the process is the
//! difference between a fixable message and a bug report.
//!
//! What "holds the port" means is the same everywhere: a socket bound to the
//! port on the loopback address the forwarder wants, or on the wildcard
//! address, which blocks a later loopback bind. A socket on a *different*
//! loopback address does not conflict. That matters on Linux, where
//! systemd-resolved's stub listener sits on `127.0.0.53:53` on almost every
//! desktop and leaves `127.0.0.1:53` free.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::udp_port_owner;
#[cfg(target_os = "macos")]
pub use macos::udp_port_owner;
#[cfg(windows)]
pub use windows::udp_port_owner;

/// A process holding a UDP port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortOwner {
    /// Zero when the socket is visible but its process is not — on Linux an
    /// unprivileged process can see every socket in `/proc/net/udp` but can
    /// only map one to a pid when it owns it.
    pub pid: u32,
    pub process: Option<String>,
    /// Service name when the holder is a service: a svchost-hosted service on
    /// Windows, a systemd unit on Linux.
    pub service: Option<String>,
}

impl PortOwner {
    /// A description suited to putting in front of a user.
    pub fn describe(&self) -> String {
        match (&self.service, &self.process, self.pid) {
            (Some(service), _, 0) => service.clone(),
            (Some(service), _, pid) => format!("{service} (pid {pid})"),
            (None, Some(process), 0) => process.clone(),
            (None, Some(process), pid) => format!("{process} (pid {pid})"),
            (None, None, 0) => "another process".into(),
            (None, None, pid) => format!("process {pid}"),
        }
    }

    /// Whether this is Internet Connection Sharing, which is the single most
    /// common holder of port 53 on Windows and has a specific remedy.
    pub fn is_ics(&self) -> bool {
        self.service
            .as_deref()
            .is_some_and(|s| s.eq_ignore_ascii_case("SharedAccess"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_prefers_the_service_name() {
        let ics = PortOwner {
            pid: 4560,
            process: Some("svchost.exe".into()),
            service: Some("SharedAccess".into()),
        };
        // "svchost.exe holds port 53" tells the user nothing actionable;
        // "SharedAccess" points straight at the service to stop.
        assert_eq!(ics.describe(), "SharedAccess (pid 4560)");
        assert!(ics.is_ics());

        let plain = PortOwner {
            pid: 99,
            process: Some("dnscrypt-proxy.exe".into()),
            service: None,
        };
        assert_eq!(plain.describe(), "dnscrypt-proxy.exe (pid 99)");
        assert!(!plain.is_ics());
    }

    #[test]
    fn an_unknown_pid_is_not_shown_as_pid_zero() {
        let unknown = PortOwner {
            pid: 0,
            process: None,
            service: None,
        };
        assert_eq!(unknown.describe(), "another process");

        let unit = PortOwner {
            pid: 0,
            process: None,
            service: Some("dnsmasq.service".into()),
        };
        assert_eq!(unit.describe(), "dnsmasq.service");
    }
}
