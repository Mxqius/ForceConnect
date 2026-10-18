//! What Linux and macOS share: `getifaddrs`, and running the system's own
//! network tools.

use std::ffi::CStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::process::Command;

use crate::NetError;

/// One interface as `getifaddrs` reports it, with its addresses gathered up.
///
/// `getifaddrs` returns one entry per *address*, so an interface with an IPv4
/// address, two IPv6 addresses and a link-layer address appears four times.
pub(crate) struct Interface {
    pub name: String,
    pub index: u32,
    pub flags: u32,
    pub unicast: Vec<IpAddr>,
    pub mac: Option<String>,
}

impl Interface {
    pub fn is_up_and_running(&self) -> bool {
        let wanted = (libc::IFF_UP | libc::IFF_RUNNING) as u32;
        self.flags & wanted == wanted
    }
}

/// Every interface, in the order the kernel lists them.
pub(crate) fn interfaces() -> Result<Vec<Interface>, NetError> {
    let mut head: *mut libc::ifaddrs = std::ptr::null_mut();

    // SAFETY: `head` is a valid out-pointer; on success the list it receives
    // is freed exactly once below, after the walk has copied everything out.
    if unsafe { libc::getifaddrs(&mut head) } != 0 {
        return Err(NetError::Api(format!(
            "getifaddrs failed: {}",
            std::io::Error::last_os_error()
        )));
    }

    let mut out: Vec<Interface> = Vec::new();
    let mut current = head;

    while !current.is_null() {
        // SAFETY: a non-null node of the list `getifaddrs` returned, which is
        // alive until `freeifaddrs`. `ifa_name` is always a valid C string.
        let entry = unsafe { &*current };
        let name = unsafe { CStr::from_ptr(entry.ifa_name) }
            .to_string_lossy()
            .into_owned();

        let at = match out.iter().position(|i| i.name == name) {
            Some(at) => at,
            None => {
                out.push(Interface {
                    // SAFETY: `ifa_name` is a valid C string, as above.
                    index: unsafe { libc::if_nametoindex(entry.ifa_name) },
                    name,
                    flags: entry.ifa_flags as u32,
                    unicast: Vec::new(),
                    mac: None,
                });
                out.len() - 1
            }
        };

        // SAFETY: `ifa_addr` is null or points at a sockaddr whose family
        // field describes the storage behind it.
        unsafe { read_address(entry.ifa_addr, &mut out[at]) };
        current = entry.ifa_next;
    }

    // SAFETY: `head` came from a successful `getifaddrs` and nothing borrowed
    // from it outlives this point.
    unsafe { libc::freeifaddrs(head) };

    Ok(out)
}

/// # Safety
///
/// `addr` must be null or point to a sockaddr whose `sa_family` correctly
/// describes the storage behind it.
unsafe fn read_address(addr: *const libc::sockaddr, into: &mut Interface) {
    if addr.is_null() {
        return;
    }

    unsafe {
        match i32::from((*addr).sa_family) {
            libc::AF_INET => {
                let v4 = &*(addr as *const libc::sockaddr_in);
                into.unicast
                    .push(IpAddr::V4(Ipv4Addr::from(u32::from_be(v4.sin_addr.s_addr))));
            }
            libc::AF_INET6 => {
                let v6 = &*(addr as *const libc::sockaddr_in6);
                into.unicast
                    .push(IpAddr::V6(Ipv6Addr::from(v6.sin6_addr.s6_addr)));
            }
            #[cfg(target_os = "linux")]
            libc::AF_PACKET => {
                let ll = &*(addr as *const libc::sockaddr_ll);
                let len = usize::from(ll.sll_halen).min(ll.sll_addr.len());
                into.mac = format_mac(&ll.sll_addr[..len]);
            }
            #[cfg(target_os = "macos")]
            libc::AF_LINK => {
                // The link-layer address follows the interface name inside
                // `sdl_data`, and may run past the declared array length — the
                // struct is variable-sized — so it is read through a pointer.
                let dl = &*(addr as *const libc::sockaddr_dl);
                let base = dl.sdl_data.as_ptr().cast::<u8>();
                let bytes = std::slice::from_raw_parts(
                    base.add(usize::from(dl.sdl_nlen)),
                    usize::from(dl.sdl_alen),
                );
                into.mac = format_mac(bytes);
            }
            _ => {}
        }
    }
}

/// `AA:BB:CC:DD:EE:FF`, the same shape the Windows reader produces. An absent
/// or all-zero address — every tunnel and loopback — is no address at all.
fn format_mac(bytes: &[u8]) -> Option<String> {
    if bytes.iter().all(|b| *b == 0) {
        return None;
    }

    Some(
        bytes
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<_>>()
            .join(":"),
    )
}

/// Where the system's network tools are looked for.
///
/// A fixed list rather than `$PATH`: the same code runs inside the privileged
/// agent, where an inherited `$PATH` is not something to trust.
const SYSTEM_DIRS: [&str; 4] = ["/usr/bin", "/bin", "/usr/sbin", "/sbin"];

pub(crate) fn locate(program: &str) -> Option<PathBuf> {
    SYSTEM_DIRS
        .iter()
        .map(|dir| PathBuf::from(dir).join(program))
        .find(|path| path.is_file())
}

/// Run a system tool and return its standard output.
///
/// `LC_ALL=C` because several of these tools translate their output, and the
/// parsers read the English.
pub(crate) fn run(program: &str, args: &[&str]) -> Result<String, NetError> {
    let path = locate(program).ok_or_else(|| NetError::Api(format!("{program} was not found")))?;

    let output = Command::new(path)
        .args(args)
        .env("LC_ALL", "C")
        .output()
        .map_err(|err| NetError::Api(format!("could not run {program}: {err}")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NetError::Api(format!(
            "{program} failed ({}): {}",
            output.status,
            stderr.trim()
        )))
    }
}
