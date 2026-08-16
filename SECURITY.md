# Security Policy

ForceConnect runs with administrator rights and acts as the machine's DNS
resolver. Both are worth scrutiny, and reports are welcome.

## Reporting a vulnerability

Please report privately rather than opening a public issue: use GitHub's
[private vulnerability reporting](https://github.com/mxqius/forceconnect/security/advisories/new)
on this repository.

Include the Windows version and steps to reproduce. If you have a proof of
concept, please attach it rather than publishing it.

You should get an acknowledgement within a week. Please give a reasonable window
for a fix before disclosing publicly.

## Scope

Most relevant to this project:

- **Elevation of privilege** — any way to make the app perform a privileged
  operation it does not expose, or to perform an exposed one against a target
  the user did not choose.
- **DNS interception or redirection** by an unprivileged local process — for
  example binding the forwarder's port first, or getting queries answered by
  something other than the configured upstream.
- **Failure to restore DNS**, leaving the machine without name resolution or
  silently pointed somewhere unexpected.
- **Certificate validation flaws** in the encrypted transports.

## Known and accepted limitations

These are deliberate design positions, not oversights. Reports about them will
be closed as working-as-intended, though arguments that the reasoning is wrong
are genuinely welcome.

**The whole process is elevated, including WebView2.** ForceConnect used to run
the interface unelevated and delegate privileged work to a helper process over a
named pipe. That is no longer the case, and the reasoning is in
[`ARCHITECTURE.md`](ARCHITECTURE.md). The short version: **same-user processes
are not a security boundary on Windows** — any process running as the user could
open the unelevated UI, read its memory, and inject into it — so the separation
never provided the isolation its shape suggested. What it did provide, an
enumerated set of privileged operations with no escape hatch, is a property of
the API and survives unchanged.

The consequence is real and worth stating: a remote-code-execution bug in
WebView2 or in the frontend now lands in an elevated process. The app loads no
remote content and its CSP forbids network access from the web view, which is
what that mitigation rests on.

**An administrator can already do everything the app does.** The privileged
surface is three operations — set an adapter's DNS servers, clear them, flush
the resolver cache — each validated against live system state. There is no
`run_command`, no arbitrary file write, no arbitrary registry write. None of it
grants capability the user lacks.

**The consent prompt appears at every launch, not once.** A genuine
approve-once would require a Windows service or a logon scheduled task running
elevated in the background. That is a permanently installed, permanently
elevated component and a materially larger thing to trust than a foreground app
that asks each time and leaves nothing behind.

## Verifying what you run

Releases are built from tagged commits by GitHub Actions and the application is
signed. There is exactly one executable and exactly one UAC prompt: if you see a
prompt for anything other than `ForceConnect`, or one from an unknown publisher,
something is wrong — please report it.

You can always build from source; see the [README](README.md).
