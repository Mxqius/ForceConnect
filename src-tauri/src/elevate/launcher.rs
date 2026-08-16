//! Starting the helper with a UAC prompt.

use std::path::Path;

use forceconnect_ipc::Launch;

use crate::elevate::ElevationError;

/// Launch the helper elevated and return its process id.
///
/// Blocking: `ShellExecuteExW` is synchronous with the flags below, and the UAC
/// prompt is modal. Call it from a blocking task, never from an async context.
#[cfg(windows)]
pub fn launch(helper: &Path, launch: &Launch, parent_hwnd: isize) -> Result<u32, ElevationError> {
    use windows::Win32::Foundation::{ERROR_CANCELLED, HWND};
    use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
    use windows::Win32::System::Threading::GetProcessId;
    use windows::Win32::UI::Shell::{
        SEE_MASK_FLAG_NO_UI, SEE_MASK_NOASYNC, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW,
        ShellExecuteExW,
    };
    use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
    use windows::core::{HRESULT, HSTRING, PCWSTR};

    let verb = HSTRING::from("runas");
    let file = HSTRING::from(helper.as_os_str());
    let parameters = HSTRING::from(launch.to_args().join(" "));
    let directory = HSTRING::from(
        helper
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .as_os_str(),
    );

    // Elevation goes through the AppInfo service over COM. A blocking thread
    // has no message loop, so initialise explicitly rather than relying on
    // ShellExecuteEx to be forgiving about it.
    //
    // SAFETY: paired with CoUninitialize below on every path.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }

    let mut info = SHELLEXECUTEINFOW {
        cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
        // NOCLOSEPROCESS is what populates hProcess at all; NOASYNC is required
        // when the calling thread has no message loop; FLAG_NO_UI suppresses
        // shell error dialogs — the UAC prompt itself is explicitly exempt, so
        // the user still sees it and we render our own failures.
        fMask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC | SEE_MASK_FLAG_NO_UI,
        hwnd: HWND(parent_hwnd as *mut _),
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(file.as_ptr()),
        lpParameters: PCWSTR(parameters.as_ptr()),
        lpDirectory: PCWSTR(directory.as_ptr()),
        nShow: SW_HIDE.0,
        ..Default::default()
    };

    // SAFETY: `info` is fully initialized and every HSTRING outlives the call.
    let result = unsafe { ShellExecuteExW(&mut info) };

    let outcome = match result {
        Ok(()) => {
            // SAFETY: hProcess is valid because SEE_MASK_NOCLOSEPROCESS was set;
            // we own it and close it here — the helper's lifetime is managed by
            // its own watchdog, not by this handle.
            let pid = unsafe { GetProcessId(info.hProcess) };
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(info.hProcess);
            }

            if pid == 0 {
                Err(ElevationError::Launch(
                    "the helper started but reported no process id".into(),
                ))
            } else {
                Ok(pid)
            }
        }
        Err(err) if err.code() == HRESULT::from_win32(ERROR_CANCELLED.0) => {
            // 0x800704C7. A decision, not a failure: no toast, no error state.
            Err(ElevationError::Declined)
        }
        Err(err) => Err(ElevationError::Launch(err.to_string())),
    };

    // SAFETY: balances the CoInitializeEx above.
    unsafe { CoUninitialize() };

    outcome
}

#[cfg(not(windows))]
pub fn launch(
    _helper: &Path,
    _launch: &Launch,
    _parent_hwnd: isize,
) -> Result<u32, ElevationError> {
    Err(ElevationError::Launch(
        "elevation is only implemented for Windows".into(),
    ))
}
