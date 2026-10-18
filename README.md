# ForceConnect

<img width="1774" height="887" alt="ForceConnect v2 0 Release Banner-1" src="https://github.com/user-attachments/assets/64c60387-ae52-4e3c-8735-269dee1b35ad" />

A fast, modern DNS manager for Windows, macOS and Linux. Switch DNS servers,
manage network interfaces, and route queries through encrypted DNS protocols
from a clean, lightweight interface.

> **Status: in development.** This is a ground-up rewrite of ForceConnect 1.x
> (C#/WinForms) on Tauri. The foundation is in place; features are landing
> incrementally. It is not yet ready for daily use.
>
> Looking for 1.x? The C#/WinForms version lives on the
> [`legacy-csharp`](https://github.com/Mxqius/ForceConnect/tree/legacy-csharp)
> branch, with its full history and the `v1.x` release tags intact.

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
- **One prompt, at launch.** ForceConnect asks for administrator rights once,
  when it starts, and holds them for that session. There is no service, no
  scheduled task, no daemon, and nothing left running when you close it.

## Architecture

On Windows, one process:

```
forceconnect.exe (requireAdministrator)
├─ React UI (WebView2)
├─ DNS forwarder → 127.0.0.1:53
├─ adapter enumeration, routes, latency probing   (read-only)
└─ forceconnect-privileged
   ├─ set / clear interface DNS
   ├─ flush resolver cache
   └─ crash-safe restore journal
```

On macOS and Linux the interface runs as you, and the same executable is started
a second time as root through the system's own password prompt (`osascript` on
macOS, polkit's `pkexec` on Linux). That second process serves the
`forceconnect-privileged` verbs above and nothing else, and exits with the app.

Everything that needs administrator rights lives in one small crate that
exposes three verbs and no general-purpose escape hatch — no `run_command`, no
arbitrary file or registry write. That, rather than process separation, is the
actual boundary; ForceConnect cannot do anything you could not already do
through your system's network settings.

Earlier versions split this into an unelevated UI and an elevated helper talking
over a named pipe. [`ARCHITECTURE.md`](ARCHITECTURE.md) explains why that was
merged on Windows, what it cost, and why macOS and Linux still split it.

### Layout

| Path                           | Contents                                                                                         |
| ------------------------------ | ------------------------------------------------------------------------------------------------ |
| `src/`                         | React frontend — design system, features, generated bindings                                     |
| `src-tauri/`                   | Tauri application shell and commands                                                             |
| `src-tauri/crates/dns/`        | Server catalog, rules, forwarder. No Tauri dependency.                                           |
| `src-tauri/crates/net/`        | Adapter enumeration and route lookups. Read-only.                                                |
| `src-tauri/crates/privileged/` | Every operation that needs administrator rights, and the agent that runs them on macOS and Linux |

## Building from source

Requires [Rust](https://rustup.rs) 1.85+, [Node](https://nodejs.org) 20+,
[pnpm](https://pnpm.io) 10+, and the platform toolchain Tauri needs: the Visual
Studio C++ build tools on Windows, the Xcode command-line tools on macOS, and on
Linux the WebKitGTK development packages — on Debian and Ubuntu,

```bash
sudo apt install libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev
```

Supported floors:

- **Windows 10 build 19041 (2004).** Not arbitrary: `SetInterfaceDnsSettings`,
  the API used to configure adapter DNS without shelling out to `netsh`, was
  introduced in that build.
- **macOS 10.15.**
- **Linux** with NetworkManager or systemd-resolved managing DNS — every
  mainstream desktop distribution — and polkit for the administrator prompt.

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

The exclusion is deliberate: a test harness linked against **`forceconnect`**
(the Tauri app crate) fails to load on Windows before any test runs. The crate is
kept as a thin shell for exactly this reason — logic lives in the crates beside
it, which test normally.

### Running the app during development

`pnpm tauri dev` triggers an administrator prompt — UAC on Windows, the password
sheet on macOS, polkit on Linux — because the app it builds asks for
administrator rights. That is the same prompt users get, and it is not
avoidable: the app cannot write DNS settings without it. On macOS and Linux,
cancelling it leaves the app running without them, which is a quick way to
work on everything that does not change DNS.

For frontend work there is a faster path that needs no elevation and no Rust
rebuild — `pnpm dev` and open <http://localhost:5173> in a browser. Outside
Tauri the IPC layer serves typed fixtures instead of real commands, so layout,
theming, and loading states can all be worked on directly.

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
