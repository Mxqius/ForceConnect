# Architecture

This document explains the shape of ForceConnect and, more importantly, why it
is that shape. A tool that asks for administrator rights and intercepts every
DNS query on the machine owes its users a legible design.

## The constraint everything follows from

Windows accepts only plain IP addresses as an adapter's DNS servers. There is no
way to tell the OS "resolve over DNS-over-HTTPS to this provider" for an
arbitrary server. Windows 11 has native DoH, but only for servers in its own
template table, and it has nothing at all for DoT, DoQ, or DoH3.

So supporting encrypted DNS means running a resolver locally, pointing the
adapter at `127.0.0.1`, and translating: plain DNS in from the OS, encrypted DNS
out to the provider.

That single fact produces most of what follows.

## Two processes

```
┌──────────────────────────────────────────────┐
│  forceconnect.exe          (asInvoker)       │
│                                              │
│  WebView2 ── React UI                        │
│  Tauri commands                              │
│  ├─ DNS forwarder ── binds 127.0.0.1:53      │
│  ├─ adapter enumeration, route lookups       │
│  ├─ latency probing                          │
│  └─ config store (%APPDATA%)                 │
└───────────────────┬──────────────────────────┘
                    │ named pipe
                    │ length-prefixed JSON
┌───────────────────┴──────────────────────────┐
│  forceconnect-helper.exe                     │
│                        (requireAdministrator)│
│  ├─ set / clear interface DNS                │
│  ├─ flush resolver cache                     │
│  └─ crash-safe restore journal               │
└──────────────────────────────────────────────┘
```

Changing an adapter's DNS requires administrator rights. Running the entire
application elevated to get them would mean a UAC prompt at every launch, a
WebView2 stack and an auto-updater running as administrator, and broken
drag-and-drop from Explorer. Instead the interface runs unelevated and delegates
the handful of privileged operations to a helper launched on demand.

### The helper is write-only

Every **read** — enumerating adapters, reading current DNS settings, route
lookups, change notifications — is unprivileged and stays in the UI process.
The helper exists solely to perform writes.

This is not merely tidy. It means the elevated process has a surface small
enough to review in one sitting, and that the UI's view of system state comes
from one place rather than two that can disagree.

### There is no escape hatch, and that is the point

The protocol has no `RunCommand`, no `WriteFile`, no
`WriteRegistry { path, value }`. Every request names a specific operation with
validated arguments.

This matters because **same-user processes are not a security boundary on
Windows**. Any process running as the user can open the UI process, read its
memory, and inject into it. So the helper cannot defend itself by authenticating
its caller — it can only limit what a caller is able to ask for. The design
target is that a fully compromised UI process still cannot do anything the user
could not already do through Windows Settings.

The consequence for contributors: if you ever find yourself wanting to add a
general-purpose privileged operation "just for debugging", that is the moment
this architecture stops being worth its complexity. Add a specific, validated
operation instead.

### What the nonce is and is not

The helper is launched with a nonce on its command line and will not serve a
client that cannot echo it back. This is a **squatting guard**, not
authentication — command lines are readable by any process running as the same
user.

The stronger check is that the helper pins its client to the exact process ID
_and_ process creation time of the process that triggered the UAC prompt. A
recycled PID cannot impersonate it, and a second copy of our own signed binary
is rejected.

## Crates

| Crate                 | Role                                                                                                           |
| --------------------- | -------------------------------------------------------------------------------------------------------------- |
| `forceconnect`        | Tauri shell: command surface, plugin wiring, window. Deliberately thin.                                        |
| `forceconnect-dns`    | Server catalog, rules engine, forwarder. **No Tauri dependency.**                                              |
| `forceconnect-net`    | Adapter enumeration and DNS configuration. Writes gated behind a `privileged` feature only the helper enables. |
| `forceconnect-ipc`    | Wire protocol and helper command line, shared by both processes.                                               |
| `forceconnect-helper` | The elevated binary.                                                                                           |

Two of these boundaries carry weight:

**`forceconnect-dns` has no Tauri dependency** so the resolver can be exercised
with plain `cargo test`. It is the most intricate part of the project and the
part most likely to attract outside contributions; it should not require a
running GUI to work on.

**`forceconnect-net` is shared** rather than duplicated because the UI reads
adapters to display them and the helper writes DNS to one of them. If each side
marshalled adapter identity independently, they could disagree about which
adapter the user picked — a bug that surfaces as "I clicked connect and nothing
happened". The `privileged` feature makes the read/write split a compile-time
property rather than a convention.

Adapters are identified by **LUID**, not by interface index or friendly name.
Indices are explicitly not persistent across adapter disable/enable, and
friendly names are localized and user-renameable.

## Things that are easy to get wrong

These are recorded because each one produces a bug that is hard to diagnose from
the symptom.

**DHCP is not the same as "no servers".** An adapter routinely has a DHCP-assigned
IP address _with_ statically configured DNS — that is exactly the state this app
creates. Restoring a DHCP adapter means clearing the static list, not writing the
previously observed servers back as a static configuration. `DnsConfig` models
this distinction explicitly.

**IPv6 will bypass you.** Setting IPv4 DNS to `127.0.0.1` while leaving IPv6 DNS
on DHCP means Windows may prefer the IPv6 resolver and route around the
forwarder entirely. Both families must be configured.

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
responsible, not shown an errno.

**Failing closed is worse than failing open.** If the app dies while the adapter
points at `127.0.0.1`, the machine loses all name resolution. The helper watches
its parent and reverts on death; before applying any change it writes a restore
journal so a hard crash or power loss is recoverable on next launch.
