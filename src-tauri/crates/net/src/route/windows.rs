//! The routing-table lookup on Windows: `GetBestRoute2`, which is what the
//! stack itself consults.

use std::net::{Ipv4Addr, Ipv6Addr};

use windows::Win32::Foundation::NO_ERROR;
use windows::Win32::NetworkManagement::IpHelper::{GetBestRoute2, MIB_IPFORWARD_ROW2};
use windows::Win32::Networking::WinSock::{
    AF_INET, AF_INET6, IN_ADDR, IN_ADDR_0, IN6_ADDR, IN6_ADDR_0, SOCKADDR_IN, SOCKADDR_IN6,
    SOCKADDR_INET,
};

use super::{PROBE_V4, PROBE_V6};
use crate::{Family, InterfaceLuid, NetError};

/// Ask the routing table which interface would carry traffic to the internet.
pub fn best_route_luid(family: Family) -> Result<InterfaceLuid, NetError> {
    let destination = match family {
        Family::V4 => sockaddr_v4(PROBE_V4),
        Family::V6 => sockaddr_v6(PROBE_V6),
    };

    let mut row = MIB_IPFORWARD_ROW2::default();
    let mut source = SOCKADDR_INET::default();

    // SAFETY: both out-parameters are valid, zeroed structs of the expected
    // type, and `destination` lives for the duration of the call.
    //
    // Passing no interface hint is the documented-adjacent idiom rather than an
    // explicitly documented one: the reference says at least one of
    // InterfaceLuid or InterfaceIndex "must be initialized", which a zeroed
    // LUID arguably satisfies. If a future Windows build disagrees it will
    // return ERROR_INVALID_PARAMETER, and the caller falls back to ranking.
    let status = unsafe { GetBestRoute2(None, 0, None, &destination, 0, &mut row, &mut source) };

    if status != NO_ERROR {
        return Err(NetError::Api(format!(
            "GetBestRoute2 failed with {}",
            status.0
        )));
    }

    // SAFETY: NET_LUID_LH is a union of a u64 and a bitfield view of the same
    // storage; reading the integer view is always valid.
    Ok(InterfaceLuid(unsafe { row.InterfaceLuid.Value }))
}

fn sockaddr_v4(addr: Ipv4Addr) -> SOCKADDR_INET {
    let mut out = SOCKADDR_INET::default();
    out.Ipv4 = SOCKADDR_IN {
        sin_family: AF_INET,
        sin_port: 0,
        sin_addr: IN_ADDR {
            S_un: IN_ADDR_0 {
                S_addr: u32::from_ne_bytes(addr.octets()),
            },
        },
        sin_zero: [0; 8],
    };
    out
}

fn sockaddr_v6(addr: Ipv6Addr) -> SOCKADDR_INET {
    let mut out = SOCKADDR_INET::default();
    out.Ipv6 = SOCKADDR_IN6 {
        sin6_family: AF_INET6,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: IN6_ADDR {
            u: IN6_ADDR_0 {
                Byte: addr.octets(),
            },
        },
        Anonymous: Default::default(),
    };
    out
}
