# Contributing

Thanks for taking a look. This is a Windows desktop app, so contributing needs a
Windows machine — most of what it does is Windows networking API calls.

## Setup

Install [Rust](https://rustup.rs) 1.85+, [Node](https://nodejs.org) 20+,
[pnpm](https://pnpm.io) 10+, and the Visual Studio C++ build tools.

```bash
pnpm install
pnpm tauri dev
```

Read [`ARCHITECTURE.md`](ARCHITECTURE.md) before changing anything under
`src-tauri/`. It explains the two-process split and the failure modes that are
easy to reintroduce.

## Adding a DNS provider

The most common contribution, and it needs no Rust. Edit
[`src-tauri/crates/dns/servers.json`](src-tauri/crates/dns/servers.json):

```json
{
  "id": "example-family",
  "name": "Example DNS",
  "variant": "Family",
  "provider": "example",
  "tags": ["family", "ad-blocking"],
  "homepage": "https://example.com",
  "endpoints": [
    { "protocol": "udp", "addrs": ["203.0.113.1", "203.0.113.2"] },
    {
      "protocol": "doh",
      "addrs": ["https://dns.example.com/dns-query"],
      "bootstrap": ["203.0.113.1"]
    }
  ]
}
```

Rules the tests enforce:

- `id` must be unique and stable. It is persisted in user settings, so renaming
  one silently resets people's selection.
- Plain `udp`/`tcp` endpoints take **IP addresses only**. A hostname would need
  DNS to resolve, which is the thing being configured.
- `doh`/`doh3` addresses are full HTTPS URLs including the path.
- `dot`/`doq` addresses are hostnames — the name is validated against the
  server's TLS certificate, so it has to be the real one.
- Any encrypted endpoint named by hostname **must** carry `bootstrap` IPs.
  Without them the first lookup after connecting would try to resolve the
  upstream through the forwarder that is trying to reach it.

Verify with:

```bash
cargo test -p forceconnect-dns --manifest-path src-tauri/Cargo.toml
```

Please only add providers you can point at published documentation for, and do
not add a protocol to an existing provider without checking they actually offer
it — an endpoint that silently fails is worse than one that is absent.

## Before opening a pull request

```bash
pnpm check
cargo test --workspace --exclude forceconnect --manifest-path src-tauri/Cargo.toml
cargo clippy --workspace --all-targets --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo fmt --all --check --manifest-path src-tauri/Cargo.toml
```

The two test exclusions are explained in the [README](README.md#checks); they
are not oversights.

## Style

- Commits follow [Conventional Commits](https://www.conventionalcommits.org).
  Keep them small and scoped.
- Comments explain **why**, not what. If a line needs a comment to say what it
  does, the line is usually the problem.
- Prefer adding a specific, validated privileged operation over a general one.
  See the escape-hatch section of [`ARCHITECTURE.md`](ARCHITECTURE.md).
- New logic belongs in `src-tauri/crates/`, not in the Tauri app crate. The app
  crate cannot run unit tests on Windows, and keeping it thin is what makes the
  rest testable.

## Reporting bugs

DNS problems are extremely environment-specific. A useful report includes your
Windows version, whether a VPN is active, and the output of:

```bash
Get-DnsClientServerAddress
netsh int ipv4 show excludedportrange protocol=tcp
```

For anything involving elevation or the helper process, please read
[`SECURITY.md`](SECURITY.md) first — some issues are better reported privately.
