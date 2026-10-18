# Architecture

This document explains the shape of ForceConnect and, more importantly, why it
is that shape. A tool that asks for administrator rights and intercepts every
DNS query on the machine owes its users a legible design.

## The constraint everything follows from

Windows accepts only plain IP addresses as an adapter's DNS servers. There is no
way to tell the OS "resolve over DNS-over-HTTPS to this provider" for an
arbitrary server. Windows 11 has native DoH, but only for servers in its own
template table, and it has nothing at all for DoT, DoQ, or DoH3. macOS and
Linux are in the same position: `networksetup` and NetworkManager take
addresses, and systemd-resolved, where it is present, speaks DNS-over-TLS and
none of the others.

So supporting encrypted DNS means running a resolver locally, pointing the
adapter at `127.0.0.1`, and translating: plain DNS in from the OS, encrypted DNS
out to the provider.

That single fact produces most of what follows.

## Windows: one process, elevated once

```
┌────────────────────────────────────────────────┐
│  forceconnect.exe      (requireAdministrator)  │
│                                                │
│  WebView2 ── React UI                          │
│  Tauri commands                                │
│  ├─ DNS forwarder ── binds 127.0.0.1:53        │
│  ├─ adapter enumeration, route lookups         │
│  ├─ latency probing                            │
│  ├─ config store (%APPDATA%)                   │
│  └─ forceconnect-privileged                    │
│     ├─ set / clear interface DNS               │
│     ├─ flush resolver cache                    │
│     └─ crash-safe restore journal              │
└────────────────────────────────────────────────┘
```

Changing an adapter's DNS and flushing the resolver cache both require
administrator rights. The app asks Windows for them the only way a plain desktop
application can: a `requireAdministrator` manifest, which produces exactly one
consent prompt, at launch, before the window appears.

### Why this replaced a separate helper

Earlier versions shipped two executables — an unelevated interface and an
elevated helper it drove over a named pipe. The goal was a smaller privileged
surface, and the price was a wire protocol, a launch nonce, a hand-built pipe
DACL, a mandatory-integrity label, a caller-pinning check, a watchdog thread,
and a second UAC prompt landing in the middle of the user's first connect.

The price was not worth what it bought, for a reason that is worth stating
plainly: **same-user processes are not a security boundary on Windows.** Any
process running as the user can open the unelevated UI process, read its memory,
and inject code into it. So the helper could never authenticate its caller in
any meaningful sense; the only real protection was that it exposed nothing
dangerous to ask for.

That protection is not a property of process separation. It is a property of the
API, and it survives the merge intact — see below.

What genuinely is lost: the WebView2 stack now runs elevated, and drag-and-drop
from Explorer into the window does not work, because Windows refuses drops from
a lower-integrity process. Both are real, and both were judged smaller than an
architecture whose subtleties took four screens of this document to explain.

### The privileged surface is still enumerated

`forceconnect-privileged` exposes four verbs — set DNS, clear DNS, release an
adapter to the user's chosen fallback, flush the resolver cache. There is no `run_command`, no `write_file`, no
`write_registry { path, value }`, and adding one is the moment to stop and
reconsider. Every argument is validated against live system state before it is
acted on: a LUID must resolve to a currently present adapter, and server lists
are checked for family, count, and addresses that are not resolvers at all.

The design target is unchanged: ForceConnect should not be able to do anything
the user could not already do through their system's network settings.

### The prompt is per launch, not once forever

`requireAdministrator` means Windows asks every time the app starts, not once on
first run. Getting to a genuine "approve once, never again" would mean
installing a Windows service or a scheduled task that runs elevated at logon —
a permanently installed, permanently elevated component, which is a materially
larger thing to trust than a foreground app that asks each time and leaves
nothing behind. That trade is available and deliberately not taken — on any
platform: macOS's `SMAppService` daemons and a polkit rule granting the app
standing rights are the same trade, and are not taken either.

## Linux and macOS: the same verbs, in an agent

Neither platform has a manifest a plain desktop app can carry, and running the
whole GUI as root is the equivalent the other two discourage — on Linux it does
not even work. A root process cannot authenticate to the user's session bus,
which the tray icon and the single-instance check both use, and Wayland
compositors refuse root clients. So the elevated part is split off, and it is
deliberately the smallest thing that can be:

```
┌──────────────────────────────────┐       ┌───────────────────────────────┐
│  forceconnect          (user)    │       │  forceconnect --privileged-   │
│                                  │       │  agent <socket>       (root)  │
│  WebKit ── React UI              │       │                               │
│  Tauri commands                  │ JSON  │  forceconnect-privileged      │
│  ├─ DNS forwarder                │ lines │  ├─ set / clear interface DNS │
│  ├─ adapter enumeration, routes  │◄─────►│  ├─ flush resolver cache      │
│  ├─ latency probing              │ Unix  │  ├─ restore journal           │
│  └─ Privileged (client)          │socket │  └─ nothing else              │
└──────────────────────────────────┘       └───────────────────────────────┘
```

At launch, after the single-instance check, the app starts **its own
executable** a second time through the platform's administrator prompt:
`pkexec` on Linux, which shows the desktop's polkit dialog, and
`osascript … with administrator privileges` on macOS, which shows the system
password sheet. That process is the agent. It connects back to the app over a
Unix socket, serves exactly the verbs `forceconnect-privileged` already had, and
exits when the app does. One prompt per launch, as on Windows. Declining it
leaves the app running unelevated, with the same banner a manifest-less Windows
build shows.

This is a helper process, which [the section above](#why-this-replaced-a-separate-helper)
argues against. The argument does not carry over, because nearly every cost it
lists came from Windows:

- **Nothing to squat.** The app listens in a fresh directory only the user can
  enter, and the agent is the one that connects — so there is no endpoint
  anyone else can reach, and no nonce or DACL to build.
- **The caller is authenticated by the kernel.** The app checks the peer's uid
  on the socket (`SO_PEERCRED`, `getpeereid`) and accepts only root. The agent
  needs no check in the other direction: the only way to start it is through
  the administrator prompt, and every argument it receives is validated against
  live system state anyway — the interface is trusted no more than the frontend
  is.
- **No second binary, no prompt mid-connect.** It is the same executable, and it
  is started at launch.

What it costs is a wire protocol, which is kept as small as the API it carries:
one JSON request per line, one response per line, a six-variant enum. It also
buys something the Windows design cannot offer: when the connection closes for
any reason — a clean quit, a crash, a `kill -9` — the agent restores whatever it
still owes before it exits, so a hard crash of the interface no longer waits for
the next launch to be repaired.

The agent replays the restore journal only when the app asks, which it does once
the single-instance check has passed. An agent that recovered on its own
initiative would let a second launch revert the first instance's live changes.

### How DNS is written on each platform

|                 | Read                                              | Write                                                                     | Flush                                                   |
| --------------- | ------------------------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------- |
| Windows         | `GetAdaptersAddresses`, `GetInterfaceDnsSettings` | `SetInterfaceDnsSettings`                                                 | `DnsFlushResolverCache`                                 |
| macOS           | `getifaddrs`, `networksetup`, `scutil --dns`      | `networksetup -setdnsservers` on the network service                      | `dscacheutil -flushcache`, `killall -HUP mDNSResponder` |
| Linux, NM       | `getifaddrs`, sysfs, procfs, `nmcli`              | `nmcli device modify` (applied connection only), `device reapply` to undo | `resolvectl flush-caches`, `nscd`                       |
| Linux, resolved | as above, `resolvectl dns`                        | `resolvectl dns` / `resolvectl revert`                                    | as above                                                |

On Linux and macOS these are the system's own tools, run with a fixed path and
arguments built from validated values — never through a shell, and never found
through `$PATH`, which inside the agent belongs to the user. They are used
instead of the underlying APIs (`SCPreferences`, the NetworkManager and resolved
D-Bus interfaces) because they are what those systems document and support, and
their output is a line or two; the parsers for it are pure functions with tests
that run on every platform.

**Linux has two owners of DNS, and it matters which one is written to.** On a
desktop, NetworkManager manages the device and re-pushes its DNS on every
connection and DHCP event — so a write _underneath_ it, straight to
systemd-resolved, is silently overwritten at the next lease renewal. A device
NetworkManager manages is therefore written through NetworkManager; only links
nothing else manages go to `resolvectl`. Which one applies is decided when the
adapters are enumerated and recorded in the adapter's write key, so the journal
still knows how to undo a change after a reboot.

NetworkManager changes use `device modify`, which changes the connection that is
_applied_ to the device and leaves the saved profile alone. Nothing the app does
is persisted, and undoing it is `device reapply`: the user's own profile, put
back exactly, rather than an approximation rebuilt from what was observed.

**macOS sets DNS on network services, not interfaces.** "Wi-Fi" is a service
bound to `en0`; the write key for an adapter is its service. Interfaces with no
service — the system's own `utun` tunnels, `awdl0`, bridges — are listed,
because routing decisions involve them, but their DNS cannot be set through any
supported interface and the app says so rather than guessing.

**Adapter identity** is the LUID on Windows. Linux and macOS have none, and
their interface indices are no more stable than Windows', so the id there is a
hash of the kernel's interface name (`wlp3s0`, `en0`), which is.

## Crates

| Crate                     | Role                                                                    |
| ------------------------- | ----------------------------------------------------------------------- |
| `forceconnect`            | Tauri shell: command surface, plugin wiring, window. Deliberately thin. |
| `forceconnect-dns`        | Server catalog, rules engine, forwarder. **No Tauri dependency.**       |
| `forceconnect-net`        | Adapter enumeration and route lookups. **Read-only.**                   |
| `forceconnect-privileged` | The only place system state is written, and the only place it can be.   |

Three of these boundaries carry weight:

**`forceconnect-dns` has no Tauri dependency** so the resolver can be exercised
with plain `cargo test`. It is the most intricate part of the project and the
part most likely to attract outside contributions; it should not require a
running GUI to work on.

**`forceconnect-net` is read-only** and owns adapter identity for the whole
application. Reads and writes agreeing about which adapter the user picked is
not automatic: if each side marshalled identity independently they could
disagree, a bug that surfaces as "I clicked connect and nothing happened".

**`forceconnect-privileged` is where every write lives.** On Windows, where the
whole process is elevated, the crate boundary is no longer a capability boundary
— it is a review boundary, which is the thing that was actually load-bearing.
The question "what does ForceConnect do with administrator rights" is answered
by reading one crate. On Linux and macOS it is both: the agent links this crate
and serves only its verbs.

An earlier version instead put the writes in `forceconnect-net` behind a
`privileged` feature. That does not work, and it is worth knowing why: Cargo
unifies features across workspace members sharing a dependency, so
`cargo build --workspace` turned the feature on everywhere and the gate compiled
away. **Crate boundaries hold where feature flags do not.**

Adapters are identified by **LUID**, not by interface index or friendly name.
Indices are explicitly not persistent across adapter disable/enable, and
friendly names are localized and user-renameable. (Linux and macOS derive the
same kind of id from the interface name; see above.)

## The manifest is attached to one binary, not the package

`tauri-build` embeds an `asInvoker` manifest into every binary a package
produces. Ours is built by hand in `build.rs` instead, with
`cargo:rustc-link-arg-bin=forceconnect=…`, for a reason that is easy to hit and
annoying to diagnose: `export-bindings` is a headless developer tool CI runs, and
with `requireAdministrator` attached it cannot start at all in a non-interactive
shell — it fails with `ERROR_ELEVATION_REQUIRED` (740).

So there are two manifest files, differing in one line. `build.rs` decides
whether to embed them from `CARGO_CFG_TARGET_OS`, not `cfg(windows)`: a build
script runs on the build machine, so `cfg` there describes the host, and a cross
build from Windows would otherwise try to put a Windows manifest into a macOS
binary. Two details in the second manifest are load-bearing:

- **Common-Controls v6 must be declared in both.** The exporter links the whole
  application library, which imports comctl32 v6 entry points by way of the
  dialog plugin. Without the dependency Windows binds the process to comctl32
  v5, those exports are missing, and the binary dies at load with
  `STATUS_ENTRYPOINT_NOT_FOUND` before `main` runs.
- **`/MANIFESTUAC:NO` is required alongside `/MANIFEST:EMBED`.** The linker
  contributes an `asInvoker` fragment of its own otherwise, and `mt.exe` refuses
  to merge two snippets that disagree:
  `manifest authoring error c1010001: Values of attribute "level" not equal`.

## Updates

`tauri-plugin-updater`, pointed at a `latest.json` published with each GitHub
release. The flow has two halves that are deliberately not symmetrical:

- **Checking** is automatic, if the user leaves "Check for updates on launch"
  on. It is a single HTTPS GET at startup and nothing else.
- **Installing** is never automatic. The app runs elevated and an update
  replaces the running binary, so it is a click every time. On Windows the
  plugin hands the downloaded package to the NSIS installer and exits the
  process; the installer restarts the app.

The update is verified against a minisign public key before anything is written
to disk. **That key is not in this repository** — `plugins.updater.pubkey` in
`tauri.conf.json` is deliberately empty, because its private half belongs in the
release pipeline's secrets and nowhere else. A build made from the repository as
it stands can therefore check for updates and will refuse to install one.

That is a real state a user can be in, so the app reports it rather than hiding
it: `updater_status` reads the configured key and endpoints back out, and the
Settings screen says "this build has no update signing key" instead of offering
a button that is guaranteed to fail at the last step.

Filling that in is a one-time setup plus a per-release checklist — see
[RELEASING.md](RELEASING.md) rather than duplicating the steps here.

## Things that are easy to get wrong

These are recorded because each one produces a bug that is hard to diagnose from
the symptom.

**DHCP is not the same as "no servers".** An adapter routinely has a DHCP-assigned
IP address _with_ statically configured DNS — that is exactly the state this app
creates. Restoring a DHCP adapter means clearing the static list, not writing the
previously observed servers back as a static configuration. `DnsConfig` models
this distinction explicitly.

**IPv6 will bypass you.** Setting IPv4 DNS to `127.0.0.1` while leaving IPv6 DNS
on DHCP means the OS may prefer the IPv6 resolver and route around the
forwarder entirely. Both families must be configured. `networksetup` and
`resolvectl` keep one list for both families, so there a write to one family
has to carry the other's current servers across, or each write wipes the other.

**Being written underneath is the same as not being written.** On Linux,
NetworkManager re-pushes DNS on every DHCP renewal. Anything written to
systemd-resolved for a device NetworkManager manages lasts until the lease
renews, then reverts with no trace of why. Write to whoever owns the device.

**The best adapter is not the one you would guess.** Ranking by adapter type and
gateway presence picks the wrong adapter on any machine with a VPN — a tunnel
routinely has a far lower interface metric than the physical adapter carrying
it. Worse, VPNs commonly install a split default route pair (`0.0.0.0/1` plus
`128.0.0.0/1`) that outranks the real default route without replacing it, so
scanning for a `0.0.0.0/0` entry misses them. Ask the routing table.

**Bootstrapping can deadlock.** Once the OS points at `127.0.0.1`, resolving a
DoH provider's hostname must not go through the forwarder. Every encrypted
endpoint carries explicit bootstrap IPs, and the catalog test enforces it.

**Port 53 is often taken.** Internet Connection Sharing binds `0.0.0.0:53` on
many machines; Hyper-V, WSL, and Docker reserve port ranges. These surface as
`WSAEACCES` or `WSAEADDRINUSE`, and the user needs to be told _which_ process is
responsible, not shown an errno. On Linux the usual suspect looks like it is
there and is not: systemd-resolved's stub holds `127.0.0.53:53`, which does not
collide with `127.0.0.1:53`, and resolved forwards to a link server on
`127.0.0.1` like any other. dnsmasq or a Pi-hole on the wildcard address does
collide, and is named. Binding port 53 on Linux also needs root or
`CAP_NET_BIND_SERVICE`, which the interface process does not have; when the
forwarder is wired up there it will need its socket from the agent. macOS has
let unprivileged processes bind low ports since 10.14.

**Failing closed is worse than failing open.** If the app dies while the adapter
points at `127.0.0.1`, the machine loses all name resolution — and nothing about
that symptom points a user back at this app. So there are two nets. The process
reverts everything it changed on `RunEvent::Exit`, which covers every ordinary
way of quitting. And before applying any change it writes a restore journal —
to `%PROGRAMDATA%\ForceConnect`, `/var/lib/forceconnect`, or
`/Library/Application Support/ForceConnect` — which covers the ways it does not
get to observe: a hard crash, a forced shutdown, a power loss. `recover()`
replays that journal at startup, before anything else touches DNS. On Linux and
macOS there is a third net under both: the agent restores on its own the moment
its connection to the app closes.

**Where "back" is, is the user's choice.** On disconnect, on exit, and in the
journal, an adapter goes to one destination chosen in Settings: the DNS it had
when the app launched (captured once at startup, after recovery), DHCP, or a
plain-DNS default of the user's own. The journal stores that destination rather
than the raw previous state, so a crash honours the choice too. A default may
not be a loopback address, because it is used exactly when nothing is
listening on loopback any more.

The journal is written to a temporary file and renamed rather than written in
place, so a crash midway through leaves either the old journal or the new one —
never a truncated file that fails to parse and loses the revert entirely.
