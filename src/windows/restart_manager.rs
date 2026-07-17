use windows::{
    Win32::{
        Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS},
        System::RestartManager::{
            CCH_RM_SESSION_KEY, RM_PROCESS_INFO, RmEndSession, RmForceShutdown,
            RmGetList, RmRegisterResources, RmShutdown, RmStartSession,
        },
    },
    core::{HSTRING, PCWSTR, PWSTR},
};

/// https://learn.microsoft.com/en-us/windows/win32/rstmgr/restart-manager-portal
/// Release file locking by seamlessly restart the process which lock the file
/// Most of the times the process will keep running smoothly after restart
/// It might take some time up to a minute
pub fn release_file_lock(file_path: &str) -> bool {
    let file_path = HSTRING::from(file_path);
    let mut session: u32 = 0;
    let mut session_key_buffer = [0_u16; (CCH_RM_SESSION_KEY as usize) + 1];
    let session_key = PWSTR(session_key_buffer.as_mut_ptr());
    let result = unsafe { RmStartSession(&mut session, Some(0), session_key) };
    if result.is_ok() {
        let result = unsafe {
            RmRegisterResources(
                session,
                Some(&[PCWSTR(file_path.as_ptr())]),
                None,
                None,
            )
        };
        if result.is_ok() {
            let mut pnprocinfoneeded: u32 = 0;
            let mut rgaffectedapps: [RM_PROCESS_INFO; 1] = [RM_PROCESS_INFO {
                ..Default::default()
            }];
            let mut lpdwrebootreasons: u32 = 0;
            let mut pnprocinfo: u32 = 0;
            let result = unsafe {
                RmGetList(
                    session,
                    &mut pnprocinfoneeded,
                    &mut pnprocinfo,
                    Some(rgaffectedapps.as_mut_ptr()),
                    &mut lpdwrebootreasons,
                )
            };
            if result == ERROR_SUCCESS || result == ERROR_MORE_DATA {
                if pnprocinfoneeded > 0 {
                    // If current process does not have enough privileges to
                    // close one of the "offending"
                    // processes, you'll get ERROR_FAIL_NOACTION_REBOOT
                    if unsafe {
                        RmShutdown(session, RmForceShutdown.0 as u32, None)
                    }
                    .is_ok()
                    {
                        // success
                        unsafe { RmEndSession(session) };
                        return true;
                    }
                } else {
                    // success
                    unsafe { RmEndSession(session) };
                    return true;
                }
            }
        }
        unsafe { RmEndSession(session) };
        return false;
    }
    false
}
