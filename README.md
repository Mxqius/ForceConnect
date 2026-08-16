# ForceConnect

A fast, modern DNS manager for Windows. Switch DNS servers, manage network
interfaces, and route queries through encrypted DNS protocols from a clean,
lightweight interface.

> **Status: in development.** This is a ground-up rewrite of ForceConnect 1.x
> (C#/WinForms) on Tauri. The foundation is in place; features are landing
> incrementally. It is not yet ready for daily use.

## Why a rewrite

The 1.x app shelled out to `netsh` to point the active adapter at one of six
hardcoded IPv4 resolvers. That works, but it caps out quickly: no encrypted
protocols, no filtering, no visibility into what is actually being resolved.

Version 2 keeps the job the same and changes how it is done:

- **Encrypted DNS, natively.** DoH, DoT, DoQ, and DoH3 through a resolver that
  runs inside the app — no bundled third-party binaries.
- **Query-level control.** Because every query passes through our own resolver,
  blocking, redirecting, and logging are a property of the design rather than
  features bolted on.
- **The UI does not run as administrator.** A small helper process holds the
  privileged operations; the interface and the web view never do.

## Architecture

Two processes, deliberately:

```
forceconnect.exe (asInvoker)          forceconnect-helper.exe (requireAdministrator)
├─ React UI (WebView2)                ├─ set / clear interface DNS
├─ DNS forwarder → 127.0.0.1:53  ←──→ ├─ flush resolver cache
├─ adapter enumeration (read-only)    └─ crash-safe restore journal
└─ latency probing                       (named pipe, nonce-authenticated)
```

The helper is **write-only**: every read — adapters, current DNS, routes —
happens unprivileged in the UI process. There is no general-purpose escape
hatch in the protocol, which is the actual security boundary. See
[`ARCHITECTURE.md`](ARCHITECTURE.md) for the reasoning.

### Layout

| Path                       | Contents                                                     |
| -------------------------- | ------------------------------------------------------------ |
| `src/`                     | React frontend — design system, features, generated bindings |
| `src-tauri/`               | Tauri application shell and commands                         |
| `src-tauri/crates/dns/`    | Server catalog, rules, forwarder. No Tauri dependency.       |
| `src-tauri/crates/net/`    | Adapter enumeration and DNS configuration                    |
| `src-tauri/crates/ipc/`    | Wire protocol shared by app and helper                       |
| `src-tauri/crates/helper/` | The elevated helper binary                                   |

## Building from source

Requires [Rust](https://rustup.rs) 1.85+, [Node](https://nodejs.org) 20+,
[pnpm](https://pnpm.io) 10+, and the Visual Studio C++ build tools.

Windows 10 build 19041 (2004) is the supported floor. That is not arbitrary:
`SetInterfaceDnsSettings`, the API used to configure adapter DNS without
shelling out to `netsh`, was introduced in that build.

```bash
pnpm install
pnpm tauri dev
```

To produce an installer:

```bash
pnpm tauri build
```

### Checks

```bash
pnpm check
```

That runs typecheck, ESLint, Prettier, and the frontend tests. For the Rust
side:

```bash
cargo test --workspace --exclude forceconnect --manifest-path src-tauri/Cargo.toml
```

Two exclusions are deliberate and worth knowing about:

- **`forceconnect`** (the Tauri app crate) — a test harness linked against it
  fails to load on Windows before any test runs. The crate is kept as a thin
  shell for this reason; logic lives in the crates beside it.
- **`forceconnect-helper`'s binary target** — its embedded manifest requests
  administrator rights, so Windows refuses to launch a test harness built from
  it. Its logic lives in `forceconnect-ipc`.

### Checking the elevation path

The UAC prompt is modal, so the helper cannot be exercised by anything
unattended. This runs the whole path — launch, pipe, handshake, caller pinning —
in one command and one click, and changes nothing:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin helper-check
```

Add `-- --apply` to also write DNS to the active adapter and revert it, which is
the only way to confirm the restore path really returns a DHCP adapter to DHCP
rather than pinning it to whatever the router was handing out.

### Regenerating bindings

`src/lib/bindings.ts` is generated from the Rust command signatures and is
committed so a fresh clone typechecks without a Rust toolchain. `pnpm tauri dev`
regenerates it automatically; to do it headlessly:

```bash
pnpm bindings
```

## Troubleshooting

**The dev server fails with `EACCES` on a port that is not in use.** Hyper-V and
WSL reserve large blocks of low ports on Windows. Check with:

```bash
netsh int ipv4 show excludedportrange protocol=tcp
```

This is why the dev server runs on 5173 rather than Tauri's default 1420.

## Contributing

Adding a DNS provider needs no Rust: edit
[`src-tauri/crates/dns/servers.json`](src-tauri/crates/dns/servers.json) and the
test suite will validate it. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

[MIT](LICENSE)
