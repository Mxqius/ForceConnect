# Releasing

A checklist, not an explanation — see [ARCHITECTURE.md](ARCHITECTURE.md#updates)
for why the updater is shaped this way.

**Current state:** [`.github/workflows/release.yml`](.github/workflows/release.yml)
builds the Windows installer, the macOS `.app`/`.dmg` (universal), and the
Linux `.deb`, `.rpm` and AppImage, and publishes the GitHub release whenever a
`v*` tag is pushed. It does not sign updater artifacts yet — that needs the
one-time key setup below, after which the two commented secrets in the
workflow are uncommented.

The macOS build is ad-hoc signed (`signingIdentity: "-"` in
[`src-tauri/tauri.macos.conf.json`](src-tauri/tauri.macos.conf.json)), not
signed with a Developer ID or notarized. That is what lets it open at all on
Apple Silicon, but Gatekeeper still asks the user to confirm on first launch.
Notarizing needs an Apple Developer account and the `APPLE_*` secrets
tauri-action documents.

## One-time: generate the signing key

Only ever done once per project (or again if the key is ever compromised —
see [Rotating the key](#rotating-the-key)).

```bash
pnpm tauri signer generate -w %USERPROFILE%\.tauri\forceconnect.key
```

This prints a public key and writes two files: the private key at the `-w`
path, and `<path>.pub` next to it. **Back up both files somewhere outside this
machine** (a password manager, not a repo) — losing the private key means
every future release has to ship under a new one, and every installed copy of
the app stops trusting updates until it is manually reinstalled.

Then:

1. Paste the printed **public** key into `plugins.updater.pubkey` in
   [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json). It is currently
   `""` on purpose — see below for why that's safe to commit.
2. Set `bundle.createUpdaterArtifacts` to `true` in the same file, so
   `tauri build` also emits the `.nsis.zip` + `.sig` pair the updater needs.
   Leaving it `false` (the current value) is what keeps an ordinary
   `pnpm tauri build` working for anyone building from source without a key.
3. Add two GitHub Actions repository secrets (Settings → Secrets and
   variables → Actions):
   - `TAURI_SIGNING_PRIVATE_KEY` — contents of the private key file.
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — the passphrase you gave the
     signer prompt (empty string if you left it blank).

The public key is meant to be public and safe to commit — it can only verify
signatures, not create them. The private key must never be committed; it only
ever lives in your local backup and in CI secrets.

## Every release

1. Bump the version — `workspace.package.version` in
   [`src-tauri/Cargo.toml`](src-tauri/Cargo.toml) and `"version"` in
   [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) — and
   `"version"` in [`package.json`](package.json). All three should match.
2. `pnpm check` and `cargo test --manifest-path src-tauri/Cargo.toml` clean.
3. Tag the commit: `git tag v<version>` and push the tag.
4. Build with the signing key in the environment:
   ```bash
   set TAURI_SIGNING_PRIVATE_KEY=<contents of the private key file>
   set TAURI_SIGNING_PRIVATE_KEY_PASSWORD=<passphrase>
   pnpm tauri build
   ```
   (Once CI exists, this step is what the workflow does instead, reading the
   same two secrets.)
5. This produces, alongside the installer:
   - `forceconnect_<version>_x64-setup.nsis.zip`
   - `forceconnect_<version>_x64-setup.nsis.zip.sig`
   - the plain `.exe` installer for people downloading directly.
6. Create a GitHub release for the `v<version>` tag and attach all three
   files, plus a `latest.json` (Tauri's `tauri build` prints the exact JSON
   to generate, or `tauri-action` does this automatically in CI). The
   endpoint in `tauri.conf.json` expects it at:
   `https://github.com/mxqius/forceconnect/releases/latest/download/latest.json`
7. Sanity check: open a build from the _previous_ version and use
   Settings → "Check now". It should find this release, download, verify, and
   install without a signature error.

## Rotating the key

If the private key is ever lost or exposed, existing installs cannot be
silently migrated — a compromised key means an attacker could sign a
malicious update, so the old key has to stop being trusted, not be quietly
replaced.

1. Generate a new key pair (same command as above) and update
   `plugins.updater.pubkey` and the CI secrets to the new pair.
2. Ship one more release signed with the _old_ key — this is the last update
   existing installs can trust automatically.
3. From the next release on, sign with the new key. Installs still on very
   old, unpatched builds will stop auto-updating at that point and need a
   fresh manual download.
