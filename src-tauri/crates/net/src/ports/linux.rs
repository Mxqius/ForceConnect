//! Port ownership on Linux, from procfs.
//!
//! `/proc/net/udp` lists every socket on the machine to any user, but maps
//! each only to an inode. Finding the process means finding which
//! `/proc/<pid>/fd` links to that inode, and an unprivileged process can only
//! look inside its own — so for a socket owned by root the answer is "another
//! process" unless this is the privileged agent asking.

use super::PortOwner;
use crate::linux::{UdpSocket, blocks_loopback, parse_proc_udp, systemd_unit};

/// Who, if anyone, holds the given UDP port on an address that would keep the
/// forwarder off loopback.
pub fn udp_port_owner(port: u16) -> Option<PortOwner> {
    let socket = ["/proc/net/udp", "/proc/net/udp6"]
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .flat_map(|body| parse_proc_udp(&body))
        .find(|s: &UdpSocket| s.port == port && blocks_loopback(s.local))?;

    let Some(pid) = pid_for_inode(socket.inode) else {
        return Some(PortOwner {
            pid: 0,
            process: None,
            service: None,
        });
    };

    let read = |file: &str| std::fs::read_to_string(format!("/proc/{pid}/{file}")).ok();
    Some(PortOwner {
        pid,
        process: read("comm").map(|s| s.trim().to_owned()),
        service: read("cgroup").and_then(|s| systemd_unit(&s)),
    })
}

fn pid_for_inode(inode: u64) -> Option<u32> {
    let wanted = format!("socket:[{inode}]");

    std::fs::read_dir("/proc")
        .ok()?
        .flatten()
        .find_map(|entry| {
            let pid: u32 = entry.file_name().to_str()?.parse().ok()?;
            let fds = std::fs::read_dir(entry.path().join("fd")).ok()?;

            fds.flatten()
                .any(|fd| {
                    std::fs::read_link(fd.path())
                        .is_ok_and(|target| target.as_os_str() == wanted.as_str())
                })
                .then_some(pid)
        })
}
