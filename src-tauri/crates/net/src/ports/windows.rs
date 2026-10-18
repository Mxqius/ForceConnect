//! Port ownership on Windows, from the extended UDP table.

use std::mem::size_of;

use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedUdpTable, MIB_UDPROW_OWNER_PID, MIB_UDPTABLE_OWNER_PID, UDP_TABLE_OWNER_PID,
};
use windows::Win32::Networking::WinSock::AF_INET;

use super::PortOwner;

/// Who, if anyone, holds the given UDP port on IPv4.
///
/// Matches a wildcard (`0.0.0.0`) bind as well as an exact one: a wildcard
/// holder blocks a later loopback bind, which is precisely the ICS case.
pub fn udp_port_owner(port: u16) -> Option<PortOwner> {
    let table = udp_table()?;

    for row in table {
        // The API reports the port in network byte order in the low half.
        let held = u16::from_be((row.dwLocalPort & 0xFFFF) as u16);
        if held == port {
            return Some(PortOwner {
                pid: row.dwOwningPid,
                process: process_name(row.dwOwningPid),
                service: service_name(row.dwOwningPid),
            });
        }
    }

    None
}

fn udp_table() -> Option<Vec<MIB_UDPROW_OWNER_PID>> {
    let mut size = 0u32;

    // SAFETY: the first call is the documented sizing probe and writes only
    // `size`.
    let probe = unsafe {
        GetExtendedUdpTable(
            None,
            &mut size,
            false,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        )
    };

    if probe != ERROR_INSUFFICIENT_BUFFER.0 && probe != NO_ERROR.0 {
        return None;
    }

    let mut buffer = vec![0u8; size as usize];
    // SAFETY: `buffer` is at least `size` bytes and correctly aligned for the
    // table header by over-allocating a byte vector.
    let result = unsafe {
        GetExtendedUdpTable(
            Some(buffer.as_mut_ptr().cast()),
            &mut size,
            false,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        )
    };

    if result != NO_ERROR.0 {
        return None;
    }

    // SAFETY: the OS wrote a MIB_UDPTABLE_OWNER_PID followed by `dwNumEntries`
    // rows into this buffer.
    unsafe {
        let table = &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
        let count = table.dwNumEntries as usize;

        let rows = std::slice::from_raw_parts(table.table.as_ptr(), count);
        Some(rows.to_vec())
    }
}

fn process_name(pid: u32) -> Option<String> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{
        OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
        QueryFullProcessImageNameW,
    };
    use windows::core::PWSTR;

    // `QueryFullProcessImageNameW` rather than `GetModuleBaseNameW`: the latter
    // needs PROCESS_VM_READ, which an unelevated process is denied against
    // service hosts — exactly the processes most likely to hold port 53.
    //
    // SAFETY: handle closed on every path; `size` bounds the buffer write.
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;

        let mut buffer = [0u16; 260];
        let mut size = buffer.len() as u32;
        let result = QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );
        let _ = CloseHandle(process);

        result.ok()?;

        let path = String::from_utf16_lossy(&buffer[..size as usize]);
        Some(path.rsplit(['\\', '/']).next().unwrap_or(&path).to_owned())
    }
}

/// Which service runs in a given process, if any.
///
/// Worth the extra call: port 53 is usually held by `svchost.exe`, and telling
/// a user that tells them nothing. "SharedAccess" points straight at the
/// service to stop.
fn service_name(pid: u32) -> Option<String> {
    use windows::Win32::System::Services::{
        ENUM_SERVICE_STATUS_PROCESSW, EnumServicesStatusExW, OpenSCManagerW, SC_ENUM_PROCESS_INFO,
        SC_MANAGER_ENUMERATE_SERVICE, SERVICE_STATE_ALL, SERVICE_WIN32,
    };
    use windows::core::PCWSTR;

    // SAFETY: the SCM handle is closed via the guard; buffer sizing follows the
    // API's two-call protocol.
    unsafe {
        let manager = OpenSCManagerW(None, None, SC_MANAGER_ENUMERATE_SERVICE).ok()?;
        let _guard = ScmGuard(manager);

        let mut needed = 0u32;
        let mut returned = 0u32;
        let mut resume = 0u32;

        // Sizing probe; expected to fail with "more data".
        let _ = EnumServicesStatusExW(
            manager,
            SC_ENUM_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            None,
            &mut needed,
            &mut returned,
            Some(&mut resume),
            PCWSTR::null(),
        );

        if needed == 0 {
            return None;
        }

        let mut buffer = vec![0u8; needed as usize];
        resume = 0;
        EnumServicesStatusExW(
            manager,
            SC_ENUM_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            Some(&mut buffer),
            &mut needed,
            &mut returned,
            Some(&mut resume),
            PCWSTR::null(),
        )
        .ok()?;

        let entries = std::slice::from_raw_parts(
            buffer.as_ptr() as *const ENUM_SERVICE_STATUS_PROCESSW,
            returned as usize,
        );

        entries
            .iter()
            .find(|entry| entry.ServiceStatusProcess.dwProcessId == pid)
            .and_then(|entry| entry.lpServiceName.to_string().ok())
    }
}

struct ScmGuard(windows::Win32::System::Services::SC_HANDLE);

impl Drop for ScmGuard {
    fn drop(&mut self) {
        // SAFETY: opened by OpenSCManagerW and closed exactly once.
        unsafe {
            let _ = windows::Win32::System::Services::CloseServiceHandle(self.0);
        }
    }
}

/// Size assertion kept close to the unsafe slice construction above.
const _: () = assert!(size_of::<MIB_UDPROW_OWNER_PID>() > 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "reads live system state"]
    fn report_who_holds_port_53() {
        match udp_port_owner(53) {
            Some(owner) => println!("port 53 held by {}", owner.describe()),
            None => println!("port 53 is free"),
        }
    }
}
