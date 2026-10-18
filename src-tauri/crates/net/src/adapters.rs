//! Adapter enumeration and per-adapter DNS configuration reads.
//!
//! Two functions with the same contract on every platform:
//!
//! - [`list`] enumerates every adapter the system knows about.
//! - [`dns_config`] reads how one adapter is getting its DNS servers for one
//!   address family, keyed by [`Adapter::guid`](crate::Adapter::guid).
//!
//! All of this is unprivileged. Nothing here changes system state.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{dns_config, list};
#[cfg(target_os = "macos")]
pub use macos::{dns_config, list};
#[cfg(windows)]
pub use windows::{dns_config, list, parse_guid};

#[cfg(all(test, unix))]
mod tests {
    use crate::{Family, OperStatus};

    /// Enumerates the machine this runs on, so it is opt-in:
    /// `cargo test -p forceconnect-net -- --ignored --nocapture`
    #[test]
    #[ignore = "reads live system state"]
    fn enumerate_this_machine() {
        let adapters = super::list().expect("enumeration should succeed");
        assert!(
            adapters
                .iter()
                .any(|a| a.kind == crate::AdapterKind::Loopback),
            "every machine has a loopback"
        );

        for a in &adapters {
            println!(
                "[{:>3}] {:<24} {:?} {:?} metric v4={} v6={} key={}",
                a.index, a.friendly_name, a.kind, a.status, a.metric_v4, a.metric_v6, a.guid
            );
            println!("      addrs={:?} gateways={:?}", a.unicast, a.gateways);
            println!(
                "      dns v4={:?} v6={:?} mac={:?}",
                a.dns_v4, a.dns_v6, a.mac
            );
            if a.status == OperStatus::Up {
                println!(
                    "      v4 config={:?}",
                    super::dns_config(&a.guid, Family::V4)
                );
            }
        }

        // Same id for the same name, every time: the frontend and the journal
        // both depend on it.
        let again = super::list().expect("enumeration should succeed");
        for (a, b) in adapters.iter().zip(&again) {
            assert_eq!((a.luid, &a.friendly_name), (b.luid, &b.friendly_name));
        }

        println!(
            "\nroute lookup: {:?}",
            crate::route::best_route_luid(Family::V4)
        );
        let best = crate::route::best_adapter(&adapters, Family::V4);
        println!("best v4 adapter: {:?}", best.map(|a| &a.friendly_name));
        println!("port 53 held by: {:?}", crate::ports::udp_port_owner(53));
    }
}
