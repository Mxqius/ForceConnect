# Security Policy

ForceConnect runs an elevated helper process and acts as the machine's DNS
resolver. Both are worth scrutiny, and reports are welcome.

## Reporting a vulnerability

Please report privately rather than opening a public issue: use GitHub's
[private vulnerability reporting](https://github.com/mxqius/forceconnect/security/advisories/new)
on this repository.

Include the Windows version, whether the helper process was involved, and steps
to reproduce. If you have a proof of concept, please attach it rather than
publishing it.

You should get an acknowledgement within a week. Please give a reasonable window
for a fix before disclosing publicly.

## Scope

Most relevant to this project:

- **Elevation of privilege** through the helper process, its named pipe, or its
  command line.
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

**Same-user processes are not treated as a boundary.** On Windows, any process
running as the user can open the UI process, read its memory, and inject code.
The helper therefore does not try to authenticate its caller cryptographically.
Instead it pins the client to the exact process ID and creation time of the
process that triggered the UAC prompt, and — more importantly — exposes no
operation the user could not already perform through Windows Settings. There is
no `RunCommand`, no arbitrary file write, no arbitrary registry write.

**The helper's nonce is readable by same-user processes.** It travels on the
command line. It is a squatting guard against stale or duplicate instances, not
an authentication mechanism, and is not described as one anywhere in the code.

**An administrator can already do everything the helper does.** The helper
lowers the number of UAC prompts; it does not grant capability that the user
lacks.

## Verifying what you run

Releases are built from tagged commits by GitHub Actions. Both the application
and the helper binary are signed. If a UAC prompt for
`forceconnect-helper.exe` shows an unknown publisher, something is wrong —
please report it.

You can always build from source; see the [README](README.md).
