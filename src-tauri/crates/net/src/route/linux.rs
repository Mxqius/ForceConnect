//! The routing-table lookup on Linux: `ip route get`, which applies policy
//! rules as well as the main table.

use super::{PROBE_V4, PROBE_V6};
use crate::linux::parse_ip_route_get;
use crate::unix;
use crate::{Family, InterfaceLuid, NetError};

/// Ask the routing table which interface would carry traffic to the internet.
pub fn best_route_luid(family: Family) -> Result<InterfaceLuid, NetError> {
    let (flag, destination) = match family {
        Family::V4 => ("-4", PROBE_V4.to_string()),
        Family::V6 => ("-6", PROBE_V6.to_string()),
    };

    let body = unix::run("ip", &["-o", flag, "route", "get", &destination])?;
    parse_ip_route_get(&body)
        .map(InterfaceLuid::from_name)
        .ok_or_else(|| NetError::Api(format!("no route to {destination}")))
}
