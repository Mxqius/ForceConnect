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

## One process, elevated once

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

`forceconnect-privileged` exposes three verbs — set DNS, clear DNS, flush the
resolver cache. There is no `run_command`, no `write_file`, no
`write_registry { path, value }`, and adding one is the moment to stop and
reconsider. Every argument is validated against live system state before it is
acted on: a LUID must resolve to a currently present adapter, and server lists
are checked for family, count, and addresses that are not resolvers at all.

The design target is unchanged: ForceConnect should not be able to do anything
the user could not already do through Windows Settings.

### The prompt is per launch, not once forever

`requireAdministrator` means Windows asks every time the app starts, not once on
first run. Getting to a genuine "approve once, never again" would mean
installing a Windows service or a scheduled task that runs elevated at logon —
a permanently installed, permanently elevated component, which is a materially
larger thing to trust than a foreground app that asks each time and leaves
nothing behind. That trade is available and deliberately not taken.

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

**`forceconnect-privileged` is where every write lives.** Now that the whole
process is elevated, the crate boundary is no longer a capability boundary — it
is a review boundary, which is the thing that was actually load-bearing. The
question "what does ForceConnect do with administrator rights" is answered by
reading four files.

An earlier version instead put the writes in `forceconnect-net` behind a
`privileged` feature. That does not work, and it is worth knowing why: Cargo
unifies features across workspace members sharing a dependency, so
`cargo build --workspace` turned the feature on everywhere and the gate compiled
away. **Crate boundaries hold where feature flags do not.**

Adapters are identified by **LUID**, not by interface index or friendly name.
Indices are explicitly not persistent across adapter disable/enable, and
friendly names are localized and user-renameable.

## The manifest is attached to one binary, not the package

`tauri-build` embeds an `asInvoker` manifest into every binary a package
produces. Ours is built by hand in `build.rs` instead, with
`cargo:rustc-link-arg-bin=forceconnect=…`, for a reason that is easy to hit and
annoying to diagnose: `export-bindings` is a headless developer tool CI runs, and
with `requireAdministrator` attached it cannot start at all in a non-interactive
shell — it fails with `ERROR_ELEVATION_REQUIRED` (740).

So there are two manifest files, differing in one line. Two details in the
second one are load-bearing:

- **Common-Controls v6 must be declared in both.** The exporter links the whole
  application library, which imports comctl32 v6 entry points by way of the
  dialog plugin. Without the dependency Windows binds the process to comctl32
  v5, those exports are missing, and the binary dies at load with
  `STATUS_ENTRYPOINT_NOT_FOUND` before `main` runs.
- **`/MANIFESTUAC:NO` is required alongside `/MANIFEST:EMBED`.** The linker
  contributes an `asInvoker` fragment of its own otherwise, and `mt.exe` refuses
  to merge two snippets that disagree:
  `manifest authoring error c1010001: Values of attribute "level" not equal`.

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
points at `127.0.0.1`, the machine loses all name resolution — and nothing about
that symptom points a user back at this app. So there are two nets. The process
reverts everything it changed on `RunEvent::Exit`, which covers every ordinary
way of quitting. And before applying any change it writes a restore journal to
`%PROGRAMDATA%\ForceConnect`, which covers the ways it does not get to observe:
a hard crash, a forced shutdown, a power loss. `recover()` replays that journal
at startup, before the window is shown and before anything else touches DNS.

The journal is written to a temporary file and renamed rather than written in
place, so a crash midway through leaves either the old journal or the new one —
never a truncated file that fails to parse and loses the revert entirely.
