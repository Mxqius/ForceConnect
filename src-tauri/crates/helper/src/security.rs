//! Making an elevated-owned pipe reachable by exactly one unelevated process.
//!
//! There are **two** independent barriers here, and fixing only the first is
//! the classic mistake — it appears to work in development, where both
//! processes were started elevated, and fails in production.
//!
//! 1. **The DACL.** With null security attributes, the default descriptor grants
//!    full control to SYSTEM, Administrators, and the creator, and *read* access
//!    to Everyone. Our client runs on the filtered token, where Administrators
//!    is deny-only, so it matches only the Everyone entry: it can read and not
//!    write.
//!
//! 2. **Mandatory Integrity Control.** An object created by a High-integrity
//!    process is labelled High, and the default policy is `NO_WRITE_UP`. A
//!    Medium-integrity client therefore cannot write **regardless of the DACL**,
//!    because MIC is evaluated first. Lowering the label to Medium is what
//!    actually makes the pipe usable.

use std::ffi::c_void;

use windows::Win32::Foundation::{HANDLE, HLOCAL, LocalFree};
use windows::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows::Win32::Security::{
    GetTokenInformation, PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES, TOKEN_GROUPS, TOKEN_QUERY,
    TokenGroups,
};
use windows::Win32::System::Threading::{
    OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::core::{HSTRING, PCWSTR};

use crate::HelperError;

/// `FILE_GENERIC_READ | FILE_GENERIC_WRITE`, minus `FILE_CREATE_PIPE_INSTANCE`.
///
/// Spelled as a literal because the obvious `FR|FW` or `0x0012019F` would
/// include it: the docs note that `FILE_APPEND_DATA` and
/// `FILE_CREATE_PIPE_INSTANCE` share a bit, so granting generic write lets the
/// grantee stand up **another instance of our pipe name** and impersonate the
/// helper to a later client.
const CLIENT_ACCESS: &str = "0x0012019B";

/// `SE_GROUP_LOGON_ID`. Spelled out because the `windows` crate exposes it as a
/// signed constant under `System::SystemServices`, and the attribute field it
/// is tested against is unsigned.
const SE_GROUP_LOGON_ID: u32 = 0xC000_0000;

/// Build the security descriptor for the helper's pipe.
///
/// `logon_sid` scopes access to one interactive logon session, which the docs
/// specifically recommend over the user SID: it keeps a different Terminal
/// Services session belonging to the same user from reaching the pipe.
pub fn pipe_sddl(logon_sid: &str) -> String {
    format!(
        // SYSTEM and Administrators: full control (the latter is us).
        // The client: read/write, but not create-instance.
        // S: a Medium mandatory label with NoWriteUp — without this line a
        // medium-integrity client is denied before the DACL is consulted.
        "D:P(A;;GA;;;SY)(A;;GA;;;BA)(A;;{CLIENT_ACCESS};;;{logon_sid})S:(ML;;NW;;;ME)"
    )
}

/// Owns a security descriptor allocated by the SDDL converter.
///
/// The descriptor must outlive every call that references it, and must be
/// released with `LocalFree` rather than dropped — hence a guard type rather
/// than a bare pointer.
pub struct PipeSecurity {
    descriptor: PSECURITY_DESCRIPTOR,
    attributes: SECURITY_ATTRIBUTES,
}

impl PipeSecurity {
    pub fn new(logon_sid: &str) -> Result<Self, HelperError> {
        let sddl = HSTRING::from(pipe_sddl(logon_sid));
        let mut descriptor = PSECURITY_DESCRIPTOR::default();

        // SAFETY: `sddl` is a valid null-terminated wide string that outlives
        // the call; `descriptor` receives a LocalAlloc'd buffer we free in Drop.
        unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                PCWSTR(sddl.as_ptr()),
                SDDL_REVISION_1,
                &mut descriptor,
                None,
            )
            .map_err(|e| HelperError::Security(format!("building the pipe descriptor: {e}")))?;
        }

        let attributes = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: descriptor.0,
            bInheritHandle: false.into(),
        };

        Ok(Self {
            descriptor,
            attributes,
        })
    }

    /// Pointer for `ServerOptions::create_with_security_attributes_raw`.
    pub fn as_raw(&mut self) -> *mut c_void {
        &raw mut self.attributes as *mut c_void
    }
}

impl Drop for PipeSecurity {
    fn drop(&mut self) {
        if !self.descriptor.is_invalid() {
            // SAFETY: allocated by ConvertStringSecurityDescriptorToSecurityDescriptorW,
            // which documents LocalFree as the matching release.
            unsafe {
                let _ = LocalFree(Some(HLOCAL(self.descriptor.0)));
            }
        }
    }
}

/// Read a process's logon SID.
///
/// Deliberately taken from the *client* rather than from ourselves. Under
/// over-the-shoulder elevation — a standard user entering an administrator's
/// credentials — the helper runs in a different logon session with a different
/// user SID, and building the DACL from our own token would lock the client
/// out of the pipe it is meant to use.
pub fn logon_sid_of(pid: u32) -> Result<String, HelperError> {
    // SAFETY: every handle is closed via the guards below; buffer sizes come
    // from the API's own two-call sizing protocol.
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
            .map_err(|e| HelperError::Security(format!("opening the client process: {e}")))?;
        let _process = HandleGuard(process);

        let mut token = HANDLE::default();
        OpenProcessToken(process, TOKEN_QUERY, &mut token)
            .map_err(|e| HelperError::Security(format!("opening the client token: {e}")))?;
        let _token = HandleGuard(token);

        let mut needed = 0u32;
        // First call sizes the buffer and is expected to fail.
        let _ = GetTokenInformation(token, TokenGroups, None, 0, &mut needed);
        if needed == 0 {
            return Err(HelperError::Security(
                "token groups reported zero size".into(),
            ));
        }

        let mut buffer = vec![0u8; needed as usize];
        GetTokenInformation(
            token,
            TokenGroups,
            Some(buffer.as_mut_ptr().cast()),
            needed,
            &mut needed,
        )
        .map_err(|e| HelperError::Security(format!("reading token groups: {e}")))?;

        let groups = &*(buffer.as_ptr() as *const TOKEN_GROUPS);
        let entries =
            std::slice::from_raw_parts(groups.Groups.as_ptr(), groups.GroupCount as usize);

        for entry in entries {
            if entry.Attributes & SE_GROUP_LOGON_ID != 0 {
                return sid_to_string(entry.Sid);
            }
        }

        Err(HelperError::Security(
            "the client token has no logon SID".into(),
        ))
    }
}

unsafe fn sid_to_string(sid: windows::Win32::Security::PSID) -> Result<String, HelperError> {
    use windows::Win32::Security::Authorization::ConvertSidToStringSidW;

    // SAFETY: `sid` points into the token-groups buffer, which is still alive.
    unsafe {
        let mut raw = windows::core::PWSTR::null();
        ConvertSidToStringSidW(sid, &mut raw)
            .map_err(|e| HelperError::Security(format!("formatting the logon SID: {e}")))?;

        let text = raw.to_string().unwrap_or_default();
        let _ = LocalFree(Some(HLOCAL(raw.0.cast())));

        if text.is_empty() {
            Err(HelperError::Security("empty logon SID".into()))
        } else {
            Ok(text)
        }
    }
}

struct HandleGuard(HANDLE);

impl Drop for HandleGuard {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            // SAFETY: the handle came from OpenProcess/OpenProcessToken and is
            // closed exactly once.
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(self.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sddl_carries_the_integrity_label() {
        let sddl = pipe_sddl("S-1-5-5-0-123456");

        // Without the SACL line a medium-integrity client is refused before
        // the DACL is even evaluated, which is the failure mode that looks
        // like "the pipe exists but access is denied".
        assert!(
            sddl.contains("S:(ML;;NW;;;ME)"),
            "missing the mandatory label: {sddl}"
        );
        assert!(sddl.starts_with("D:P"), "DACL must be protected: {sddl}");
        assert!(sddl.contains("S-1-5-5-0-123456"));
    }

    #[test]
    fn client_is_not_granted_create_instance() {
        let sddl = pipe_sddl("S-1-5-5-0-1");

        // 0x0012019F would include FILE_CREATE_PIPE_INSTANCE, letting any
        // process with client access publish a rogue instance of our pipe name.
        assert!(sddl.contains("0x0012019B"));
        assert!(!sddl.contains("0x0012019F"));
        assert!(!sddl.contains(";;;WD)"), "Everyone must not appear: {sddl}");
    }
}
