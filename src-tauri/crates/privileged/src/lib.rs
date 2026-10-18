//! Everything ForceConnect needs administrator rights for.
//!
//! ForceConnect used to ship two executables: an unelevated interface and a
//! small elevated helper it talked to over a named pipe. That bought a smaller
//! privileged surface at the cost of a second binary, a wire protocol, a
//! squatting nonce, a pipe DACL, an integrity label, a caller-pinning check,
//! and a second UAC prompt in the middle of the user's first connect.
//!
//! It is now one application that asks for elevation once, at launch. What
//! survives from that design is the part that carried the weight: **the
//! privileged operations are enumerated in one place.** There is no
//! `run_command`, no `write_registry`, no general-purpose escape hatch — this
//! crate exposes a short, fixed set of verbs (set DNS, clear DNS, flush the
//! resolver cache), plus putting back what they changed — to the user's chosen
//! destination — and every argument is validated against live system state
//! before it is acted on.
//!
//! The rule for contributors is unchanged: if you find yourself wanting to add
//! a general-purpose privileged operation "just for debugging", add a specific
//! validated one instead.
//!
//! # Platforms
//!
//! On Windows the whole process is elevated and the verbs run in it. On Linux
//! and macOS the interface runs as the user and the verbs run in a root
//! [agent] started at launch — the same binary, serving the same verbs, for
//! reasons the module explains. [`Privileged`] hides the difference: it is
//! what the application holds, and it has the same methods everywhere.

pub mod agent;
pub mod cache;
pub mod dns_settings;
pub mod journal;
mod session;

#[cfg(any(unix, test))]
mod exec;

pub use session::Privileged;

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use forceconnect_net::{DnsConfig, Family, InterfaceLuid, adapters};

/// The most resolvers worth configuring for one address family. Windows will
/// not usefully take more, and glibc reads only the first three.
const MAX_SERVERS_PER_FAMILY: usize = 4;

/// Serializable because it crosses the agent connection on Linux and macOS:
/// a refusal there reaches the interface as the same variant it would have
/// been in-process.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum PrivilegedError {
    #[error("ForceConnect is not running with administrator rights")]
    NotElevated,

    #[error("no network adapter matches {0:?}")]
    UnknownInterface(InterfaceLuid),

    #[error("{0}")]
    Invalid(String),

    #[error("the operating system rejected the change: {0}")]
    Os(String),
}

impl From<forceconnect_net::NetError> for PrivilegedError {
    fn from(value: forceconnect_net::NetError) -> Self {
        Self::Os(value.to_string())
    }
}

/// What an adapter's DNS looked like before we touched it, per address family.
///
/// Keyed by GUID rather than LUID because the DNS settings API is keyed on the
/// GUID, and resolving LUID → GUID means consulting live state — which is the
/// step that keeps a stale or invented adapter id from reaching the OS.
type Applied = HashMap<(String, Family), DnsConfig>;

/// Where an adapter goes when ForceConnect lets go of it — on disconnect, on
/// exit, and after a crash.
///
/// One policy covers all three on purpose. If disconnecting put the user on
/// their chosen fallback but a crash put them somewhere else, the setting would
/// only hold for the exits that did not matter.
/// Serializable because it crosses the agent connection on Linux and macOS.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ReleasePolicy {
    /// Back to what Windows had before ForceConnect started.
    #[default]
    Restore,
    /// DHCP-assigned DNS, whatever it was before.
    Automatic,
    /// A fixed set of servers the user chose as their everyday DNS.
    ///
    /// A family with no servers in the list falls back to [`Self::Restore`]
    /// for that family. Writing nothing would leave IPv6 on our resolver, and
    /// inventing an IPv6 server the user never picked would be worse.
    Fallback(Vec<IpAddr>),
}

/// The one object in the application that changes system DNS settings.
///
/// It remembers what it changed so it can put everything back, and mirrors that
/// memory to disk before each change so a hard crash is recoverable on the next
/// launch. Both matter for the same reason: an adapter left pointing at a
/// resolver that is no longer running means a machine with no working DNS, and
/// the user has no reason to connect that to this app.
pub struct DnsWriter {
    applied: Applied,
    /// Every adapter's DNS as it stood when the app started, before anything
    /// here touched it. What [`ReleasePolicy::Restore`] puts back.
    baseline: Applied,
    policy: ReleasePolicy,
    journal_path: PathBuf,
}

impl Default for DnsWriter {
    fn default() -> Self {
        Self::new(journal::default_path())
    }
}

impl DnsWriter {
    pub fn new(journal_path: PathBuf) -> Self {
        Self {
            applied: Applied::new(),
            baseline: Applied::new(),
            policy: ReleasePolicy::default(),
            journal_path,
        }
    }

    /// Record every adapter's current DNS as the configuration to go back to.
    ///
    /// Called once at startup, after [`recover`] — so a run that crashed while
    /// pointing an adapter at a dead resolver has already been undone, and what
    /// is captured is the user's real setup rather than our leftovers.
    ///
    /// Only a read. Adapters whose settings cannot be read are left out, and
    /// fall back to whatever was observed when they were first changed.
    pub fn capture_baseline(&mut self) {
        let adapters = match adapters::list() {
            Ok(list) => list,
            Err(err) => {
                tracing::warn!("could not capture the launch DNS baseline: {err}");
                return;
            }
        };

        self.baseline.clear();
        for adapter in adapters.iter().filter(|a| a.is_eligible()) {
            for family in [Family::V4, Family::V6] {
                if let Ok(config) = adapters::dns_config(&adapter.guid, family) {
                    self.baseline.insert((adapter.guid.clone(), family), config);
                }
            }
        }
    }

    /// The configuration captured at launch for one adapter and family.
    pub fn baseline(&self, guid: &str, family: Family) -> Option<&DnsConfig> {
        self.baseline.get(&(guid.to_owned(), family))
    }

    /// Change where adapters go when they are released.
    ///
    /// The journal is rewritten straight away, so a crash after this point
    /// lands on the new choice rather than the old one.
    pub fn set_policy(&mut self, policy: ReleasePolicy) -> Result<(), PrivilegedError> {
        if let ReleasePolicy::Fallback(servers) = &policy {
            validate_fallback(servers)?;
        }

        self.policy = policy;
        self.persist();
        Ok(())
    }

    pub fn policy(&self) -> &ReleasePolicy {
        &self.policy
    }

    /// Every captured launch configuration, for crossing the agent connection.
    pub fn baseline_entries(&self) -> Vec<(String, Family, DnsConfig)> {
        self.baseline
            .iter()
            .map(|((guid, family), config)| (guid.clone(), *family, config.clone()))
            .collect()
    }

    /// Where this writer keeps its journal.
    pub fn journal_path(&self) -> &Path {
        &self.journal_path
    }

    /// Point an adapter at a list of resolvers.
    ///
    /// Both address families present in `servers` are written, because setting
    /// IPv4 alone while IPv6 DNS stays on DHCP lets the OS prefer the IPv6
    /// resolver and route around the change entirely.
    pub fn set_dns(
        &mut self,
        luid: InterfaceLuid,
        servers: &[IpAddr],
    ) -> Result<(), PrivilegedError> {
        require_elevation()?;
        validate(servers)?;

        let guid = resolve_guid(luid)?;

        for (family, list) in split_by_family(servers) {
            if list.is_empty() {
                continue;
            }

            self.remember(&guid, family);
            dns_settings::set_dns(&guid, family, &list)?;
            tracing::info!(%guid, ?family, count = list.len(), "DNS applied");
        }

        Ok(())
    }

    /// Return an adapter to DHCP-assigned DNS for both address families.
    pub fn clear_dns(&mut self, luid: InterfaceLuid) -> Result<(), PrivilegedError> {
        require_elevation()?;

        let guid = resolve_guid(luid)?;

        for family in [Family::V4, Family::V6] {
            self.remember(&guid, family);
            dns_settings::clear_dns(&guid, family)?;
        }

        // The adapter is back on DHCP, so there is nothing left to revert for
        // it. Keeping the entries would mean a later restore writes the user's
        // pre-connect servers over a configuration they have since changed.
        self.forget(&guid);
        tracing::info!(%guid, "DNS cleared");

        Ok(())
    }

    /// Let go of an adapter: put it where the release policy says.
    ///
    /// This is what disconnecting does. It differs from [`Self::clear_dns`] —
    /// which always means DHCP — in that the destination is the user's
    /// choice: their DNS from before launch, DHCP, or a default of their own.
    ///
    /// Returns the configuration written per family, so the UI can say where
    /// the adapter went.
    pub fn release(
        &mut self,
        luid: InterfaceLuid,
    ) -> Result<Vec<(Family, DnsConfig)>, PrivilegedError> {
        require_elevation()?;

        let guid = resolve_guid(luid)?;
        let mut written = Vec::with_capacity(2);

        for family in [Family::V4, Family::V6] {
            let key = (guid.clone(), family);
            // An adapter this session never changed — one a previous run left
            // on a static default, say — has no recorded previous state. DHCP
            // stands in for it, and `target` still prefers the launch baseline.
            let previous = self.applied.get(&key).cloned().unwrap_or(DnsConfig::Dhcp);
            let target = self.target(&guid, family, &previous);

            dns_settings::restore(&guid, family, &target)?;
            written.push((family, target));
        }

        // Released: nothing is owed a revert any more, and keeping the entries
        // would make a later exit overwrite whatever the user does next.
        self.forget(&guid);
        tracing::info!(%guid, policy = ?self.policy, "DNS released");

        Ok(written)
    }

    /// Drop every entry in the system resolver cache.
    pub fn flush_cache(&self) -> Result<(), PrivilegedError> {
        require_elevation()?;
        cache::flush().map_err(PrivilegedError::Os)
    }

    /// Put every adapter this writer touched back the way it was.
    ///
    /// Best-effort by design: one adapter failing to restore must not stop the
    /// others, and this runs on the way out of the process where there is
    /// nobody left to report an error to.
    pub fn restore_all(&mut self) {
        for ((guid, family), previous) in &self.applied {
            let target = self.target(guid, *family, previous);
            match dns_settings::restore(guid, *family, &target) {
                Ok(()) => tracing::info!(%guid, ?family, "reverted"),
                Err(err) => tracing::error!(%guid, ?family, "revert failed: {err}"),
            }
        }

        self.applied.clear();
        self.persist();
    }

    /// Whether anything is currently owed a revert.
    pub fn has_pending(&self) -> bool {
        !self.applied.is_empty()
    }

    /// Record an adapter's current configuration, if this is the first time we
    /// have touched it.
    ///
    /// Only the first observation is the user's real configuration: switching
    /// servers three times must still revert to what they started with, not to
    /// the second server.
    fn remember(&mut self, guid: &str, family: Family) {
        let key = (guid.to_owned(), family);
        if self.applied.contains_key(&key) {
            return;
        }

        // A read failure here is not a reason to refuse the change — treat it
        // as DHCP, which is the configuration the overwhelming majority of
        // adapters are in and the safer thing to restore to.
        let previous = adapters::dns_config(guid, family).unwrap_or(DnsConfig::Dhcp);
        self.applied.insert(key, previous);
        self.persist();
    }

    fn forget(&mut self, guid: &str) {
        self.applied.retain(|(applied, _), _| applied != guid);
        self.persist();
    }

    /// Where one adapter/family goes on release, under the current policy.
    ///
    /// `previous` is what it looked like when this session first changed it.
    fn target(&self, guid: &str, family: Family, previous: &DnsConfig) -> DnsConfig {
        resolve_target(
            &self.policy,
            family,
            self.baseline.get(&(guid.to_owned(), family)),
            previous,
        )
    }

    fn persist(&self) {
        // The journal stores where each adapter should *end up*, not merely
        // what it was: recovery after a crash has to honour the release policy
        // too, and by then there is no policy in memory to consult.
        let mut record = journal::Journal::default();
        for ((guid, family), previous) in &self.applied {
            record.record(guid, *family, self.target(guid, *family, previous));
        }

        if let Err(err) = journal::save(&self.journal_path, &record) {
            // Not fatal. The in-process revert still works; only a hard crash
            // needs the journal, and refusing to change DNS because a file
            // could not be written would be the worse failure.
            tracing::warn!("could not write the revert journal: {err}");
        }
    }
}

/// Undo changes left behind by a previous run that did not exit cleanly.
///
/// Called once at startup, before anything else touches DNS. Returns how many
/// adapter/family pairs were put back, so the caller can say so rather than
/// silently repairing the machine.
pub fn recover(journal_path: &Path) -> usize {
    let Some(record) = journal::load(journal_path) else {
        return 0;
    };

    let mut restored = 0;
    for entry in &record.entries {
        match dns_settings::restore(&entry.guid, entry.family, &entry.previous) {
            Ok(()) => {
                restored += 1;
                tracing::info!(guid = %entry.guid, ?entry.family, "recovered from journal");
            }
            Err(err) => {
                tracing::error!(guid = %entry.guid, "recovery failed: {err}");
            }
        }
    }

    if let Err(err) = journal::clear(journal_path) {
        tracing::warn!("could not clear the revert journal: {err}");
    }

    restored
}

/// The release destination for one family, as a pure function of its inputs.
///
/// The launch baseline wins over the first-touch observation: it is the
/// configuration the user had before opening the app, which is what "restore"
/// promises, even if something else changed the adapter in between.
fn resolve_target(
    policy: &ReleasePolicy,
    family: Family,
    baseline: Option<&DnsConfig>,
    previous: &DnsConfig,
) -> DnsConfig {
    let restored = || baseline.unwrap_or(previous).clone();

    match policy {
        ReleasePolicy::Restore => restored(),
        ReleasePolicy::Automatic => DnsConfig::Dhcp,
        ReleasePolicy::Fallback(servers) => {
            let [(_, v4), (_, v6)] = split_by_family(servers);
            let list = if family == Family::V4 { v4 } else { v6 };
            if list.is_empty() {
                restored()
            } else {
                DnsConfig::Static(list)
            }
        }
    }
}

/// A default DNS has to work with ForceConnect gone, since that is when it is
/// used — so on top of the usual checks, nothing on loopback.
fn validate_fallback(servers: &[IpAddr]) -> Result<(), PrivilegedError> {
    validate(servers)?;

    if servers.iter().any(IpAddr::is_loopback) {
        return Err(PrivilegedError::Invalid(
            "a default DNS cannot be a loopback address: nothing answers there once ForceConnect closes".into(),
        ));
    }

    Ok(())
}

/// Map a LUID to an adapter GUID by consulting live state.
///
/// Never take the caller's word for which adapter exists: this is the
/// validation step that keeps a stale id from reaching the OS.
fn resolve_guid(luid: InterfaceLuid) -> Result<String, PrivilegedError> {
    adapters::list()?
        .into_iter()
        .find(|adapter| adapter.luid == luid)
        .map(|adapter| adapter.guid)
        .ok_or(PrivilegedError::UnknownInterface(luid))
}

/// Reject a server list before any of it reaches the adapter.
///
/// A bad list here is not hypothetical — custom servers are user-entered, and
/// an unspecified or multicast address written onto an adapter produces a
/// machine that cannot resolve anything and gives no clue why.
fn validate(servers: &[IpAddr]) -> Result<(), PrivilegedError> {
    if servers.is_empty() {
        return Err(PrivilegedError::Invalid("no servers were given".into()));
    }

    if servers
        .iter()
        .any(|ip| ip.is_multicast() || ip.is_unspecified())
    {
        return Err(PrivilegedError::Invalid(
            "multicast and unspecified addresses are not valid resolvers".into(),
        ));
    }

    for (family, list) in split_by_family(servers) {
        if list.len() > MAX_SERVERS_PER_FAMILY {
            return Err(PrivilegedError::Invalid(format!(
                "{} {family:?} servers given; at most {MAX_SERVERS_PER_FAMILY} are supported",
                list.len()
            )));
        }
    }

    Ok(())
}

/// Group servers by address family, since each family is a separate write.
fn split_by_family(servers: &[IpAddr]) -> [(Family, Vec<IpAddr>); 2] {
    [
        (
            Family::V4,
            servers.iter().copied().filter(IpAddr::is_ipv4).collect(),
        ),
        (
            Family::V6,
            servers.iter().copied().filter(IpAddr::is_ipv6).collect(),
        ),
    ]
}

fn require_elevation() -> Result<(), PrivilegedError> {
    if is_elevated() {
        Ok(())
    } else {
        Err(PrivilegedError::NotElevated)
    }
}

/// Whether this process is running with an elevated token.
///
/// The shipped executable embeds a `requireAdministrator` manifest, so in
/// normal operation this is always true. It is checked anyway: a build with the
/// manifest stripped would otherwise fail deep inside a Win32 call with a bare
/// error code instead of saying what is actually wrong.
#[cfg(windows)]
pub fn is_elevated() -> bool {
    use std::mem::size_of;

    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Security::{
        GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    // SAFETY: the token handle is closed on every path, and GetTokenInformation
    // is given a correctly sized buffer for the class requested.
    unsafe {
        let mut token = Default::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut returned = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&raw mut elevation as *mut _),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned,
        )
        .is_ok();

        let _ = CloseHandle(token);
        ok && elevation.TokenIsElevated != 0
    }
}

/// Whether this process is running as root.
///
/// True only inside the privileged agent. The interface itself never runs as
/// root on these platforms; it asks [`Privileged::is_elevated`] instead.
#[cfg(unix)]
pub fn is_elevated() -> bool {
    // SAFETY: `geteuid` has no preconditions and cannot fail.
    unsafe { libc::geteuid() == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ips(values: &[&str]) -> Vec<IpAddr> {
        values.iter().map(|v| v.parse().unwrap()).collect()
    }

    #[test]
    fn families_are_split_for_separate_writes() {
        let servers = ips(&["1.1.1.1", "2606:4700:4700::1111", "1.0.0.1"]);

        let [(v4_family, v4), (v6_family, v6)] = split_by_family(&servers);
        assert_eq!(v4_family, Family::V4);
        assert_eq!(v4.len(), 2);
        assert_eq!(v6_family, Family::V6);
        assert_eq!(v6.len(), 1);
    }

    #[test]
    fn unusable_addresses_are_rejected() {
        assert!(validate(&[]).is_err());
        assert!(validate(&ips(&["0.0.0.0"])).is_err());
        assert!(validate(&ips(&["224.0.0.251"])).is_err());
        assert!(validate(&ips(&["::"])).is_err());
        assert!(validate(&ips(&["1.1.1.1", "2606:4700:4700::1111"])).is_ok());
    }

    #[test]
    fn restore_prefers_the_launch_baseline() {
        let launch = DnsConfig::Static(ips(&["192.168.1.1"]));
        let first_touch = DnsConfig::Static(ips(&["10.0.0.1"]));

        assert_eq!(
            resolve_target(
                &ReleasePolicy::Restore,
                Family::V4,
                Some(&launch),
                &first_touch
            ),
            launch
        );
        assert_eq!(
            resolve_target(&ReleasePolicy::Restore, Family::V4, None, &first_touch),
            first_touch
        );
    }

    #[test]
    fn automatic_always_means_dhcp() {
        let launch = DnsConfig::Static(ips(&["192.168.1.1"]));
        assert_eq!(
            resolve_target(
                &ReleasePolicy::Automatic,
                Family::V4,
                Some(&launch),
                &launch
            ),
            DnsConfig::Dhcp
        );
    }

    #[test]
    fn a_fallback_only_covers_the_families_it_lists() {
        // An IPv4-only default must not leave IPv6 pointing at our resolver,
        // and must not invent an IPv6 server either: that family is restored.
        let policy = ReleasePolicy::Fallback(ips(&["1.1.1.1", "1.0.0.1"]));
        let v6_launch = DnsConfig::Dhcp;

        assert_eq!(
            resolve_target(&policy, Family::V4, None, &DnsConfig::Dhcp),
            DnsConfig::Static(ips(&["1.1.1.1", "1.0.0.1"]))
        );
        assert_eq!(
            resolve_target(&policy, Family::V6, Some(&v6_launch), &DnsConfig::Dhcp),
            v6_launch
        );
    }

    #[test]
    fn a_fallback_cannot_point_at_loopback() {
        assert!(validate_fallback(&ips(&["127.0.0.1"])).is_err());
        assert!(validate_fallback(&ips(&["::1"])).is_err());
        assert!(validate_fallback(&ips(&["9.9.9.9"])).is_ok());
    }

    #[test]
    fn the_per_family_limit_is_per_family_not_overall() {
        // Four of each is fine; five of one is not. Counting the combined list
        // would reject a perfectly valid dual-stack configuration.
        let dual = ips(&[
            "1.1.1.1",
            "1.0.0.1",
            "9.9.9.9",
            "8.8.8.8",
            "2606:4700:4700::1111",
            "2606:4700:4700::1001",
        ]);
        assert!(validate(&dual).is_ok());

        let too_many = ips(&["1.1.1.1", "1.0.0.1", "9.9.9.9", "8.8.8.8", "8.8.4.4"]);
        assert!(validate(&too_many).is_err());
    }
}
