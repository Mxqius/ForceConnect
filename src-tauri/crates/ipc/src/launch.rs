//! The helper's command line.
//!
//! This is as much a part of the protocol as the pipe messages are: the app
//! builds this argument list, the helper parses it, and if the two ever
//! disagree the helper simply refuses to start. Defining both directions in one
//! place — with a round-trip test — is what keeps that from happening.

/// How the app identified itself when it launched the helper.
///
/// # On the nonce
///
/// The nonce is a squatting guard, not authentication. It travels on the
/// helper's command line, which any process running as the same user can read.
/// Real assurance comes from two other places:
///
/// 1. The helper pins its client to [`Launch::parent_pid`] *and*
///    [`Launch::parent_start`], so only the exact process instance that
///    triggered the UAC prompt can connect, and a recycled PID cannot
///    impersonate it.
/// 2. The protocol exposes no operation the user could not already perform
///    through Windows Settings.
///
/// Same-user processes are not a security boundary on Windows. Nothing here
/// pretends otherwise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launch {
    /// Full name of the pipe the helper should create.
    pub pipe: String,
    /// Shared secret the client echoes back in its first message.
    pub nonce: String,
    /// PID of the process that launched the helper.
    pub parent_pid: u32,
    /// Parent's creation time as a `FILETIME`, in 100ns ticks. Checked
    /// alongside the PID to defeat PID reuse.
    pub parent_start: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LaunchArgsError {
    #[error(
        "no arguments: this program is launched by ForceConnect and is not meant to be run directly"
    )]
    NoArguments,

    #[error("missing required argument --{0}")]
    Missing(&'static str),

    #[error("--{name} expects a value")]
    MissingValue { name: String },

    #[error("--{name} is not a valid number")]
    NotANumber { name: String },

    #[error("unrecognized argument '{0}'")]
    Unknown(String),
}

impl Launch {
    /// Render the argument list the app passes to `ShellExecuteExW`.
    pub fn to_args(&self) -> Vec<String> {
        vec![
            "--pipe".into(),
            self.pipe.clone(),
            "--nonce".into(),
            self.nonce.clone(),
            "--parent-pid".into(),
            self.parent_pid.to_string(),
            "--parent-start".into(),
            self.parent_start.to_string(),
        ]
    }

    /// Parse the helper's arguments, excluding argv[0].
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Self, LaunchArgsError> {
        let mut pipe = None;
        let mut nonce = None;
        let mut parent_pid = None;
        let mut parent_start = None;

        let mut args = args.into_iter().peekable();
        if args.peek().is_none() {
            return Err(LaunchArgsError::NoArguments);
        }

        while let Some(arg) = args.next() {
            let Some(name) = arg.strip_prefix("--").map(str::to_owned) else {
                return Err(LaunchArgsError::Unknown(arg));
            };

            let value = args
                .next()
                .ok_or_else(|| LaunchArgsError::MissingValue { name: name.clone() })?;

            let as_number = |v: &str| {
                v.parse::<u64>()
                    .map_err(|_| LaunchArgsError::NotANumber { name: name.clone() })
            };

            match name.as_str() {
                "pipe" => pipe = Some(value),
                "nonce" => nonce = Some(value),
                "parent-pid" => {
                    parent_pid = Some(
                        u32::try_from(as_number(&value)?)
                            .map_err(|_| LaunchArgsError::NotANumber { name: name.clone() })?,
                    );
                }
                "parent-start" => parent_start = Some(as_number(&value)?),
                _ => return Err(LaunchArgsError::Unknown(format!("--{name}"))),
            }
        }

        Ok(Self {
            pipe: pipe.ok_or(LaunchArgsError::Missing("pipe"))?,
            nonce: nonce.ok_or(LaunchArgsError::Missing("nonce"))?,
            parent_pid: parent_pid.ok_or(LaunchArgsError::Missing("parent-pid"))?,
            parent_start: parent_start.ok_or(LaunchArgsError::Missing("parent-start"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Launch {
        Launch {
            pipe: format!("{}-abc123", crate::PIPE_PREFIX),
            nonce: "deadbeefcafe".into(),
            parent_pid: 4242,
            parent_start: 133_700_000_000_000_000,
        }
    }

    #[test]
    fn args_round_trip() {
        // The property that matters: whatever the app writes, the helper reads
        // back identically.
        let launch = sample();
        assert_eq!(Launch::parse(launch.to_args()).unwrap(), launch);
    }

    #[test]
    fn bare_launch_is_refused() {
        // Double-clicking the helper in Explorer should produce a clear
        // refusal, not a UAC prompt for a process that then does nothing.
        assert_eq!(Launch::parse(vec![]), Err(LaunchArgsError::NoArguments));
    }

    #[test]
    fn every_argument_is_required() {
        for omit in ["--pipe", "--nonce", "--parent-pid", "--parent-start"] {
            let mut kept = Vec::new();
            let mut iter = sample().to_args().into_iter();
            while let Some(flag) = iter.next() {
                let value = iter.next().expect("flags come in pairs");
                if flag != omit {
                    kept.push(flag);
                    kept.push(value);
                }
            }

            assert!(
                matches!(Launch::parse(kept), Err(LaunchArgsError::Missing(_))),
                "omitting {omit} should be rejected"
            );
        }
    }

    #[test]
    fn malformed_arguments_are_rejected() {
        let args = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();

        assert!(matches!(
            Launch::parse(args(&["--wat", "1"])),
            Err(LaunchArgsError::Unknown(_))
        ));
        assert!(matches!(
            Launch::parse(args(&["positional"])),
            Err(LaunchArgsError::Unknown(_))
        ));
        assert!(matches!(
            Launch::parse(args(&["--pipe"])),
            Err(LaunchArgsError::MissingValue { .. })
        ));
        assert!(matches!(
            Launch::parse(args(&["--parent-pid", "nope"])),
            Err(LaunchArgsError::NotANumber { .. })
        ));
        // A PID wider than u32 is malformed, not merely large.
        assert!(matches!(
            Launch::parse(args(&["--parent-pid", "99999999999999"])),
            Err(LaunchArgsError::NotANumber { .. })
        ));
    }
}
