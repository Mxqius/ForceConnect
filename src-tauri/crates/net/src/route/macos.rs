//! The routing-table lookup on macOS: `route -n get`, which asks the kernel
//! the same question `GetBestRoute2` does on Windows.

use super::{PROBE_V4, PROBE_V6};
use crate::macos::parse_route_get;
use crate::unix;
use crate::{Family, InterfaceLuid, NetError};

/// Ask the routing table which interface would carry traffic to the internet.
pub fn best_route_luid(family: Family) -> Result<InterfaceLuid, NetError> {
    let destination = match family {
        Family::V4 => PROBE_V4.to_string(),
        Family::V6 => PROBE_V6.to_string(),
    };

    let mut args = vec!["-n", "get"];
    if family == Family::V6 {
        args.push("-inet6");
    }
    args.push(&destination);

    let body = unix::run("route", &args)?;
    parse_route_get(&body)
        .map(InterfaceLuid::from_name)
        .ok_or_else(|| NetError::Api(format!("no route to {destination}")))
}
