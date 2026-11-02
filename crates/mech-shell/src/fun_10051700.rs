// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[repr(C)]
struct tagPOINT {
    x: i32,
    y: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
    static mut DAT_100d08d4: u32;
    static mut DAT_100d08d8: u32;
    static mut DAT_100d08e8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetCursorPos(p: *mut c_void) -> i32;
    fn ScreenToClient(h: *mut c_void, p: *mut c_void) -> i32;
    fn ClientToScreen(h: *mut c_void, p: *mut c_void) -> i32;
    fn SetCursorPos(x: i32, y: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051700(param_1: u32, param_2: i32) -> u32 {
    mech_trace::enter!("10051700", param_1, param_2);
    let _ = param_1;
    let mut local_8 = tagPOINT { x: 0, y: 0 };
    // SAFETY: the globals and the Win32 calls are the game's own; local_8 is
    // a valid POINT on our stack for the calls to read and write.
    unsafe {
        if mech_trace::gr!("10051700", "10098204", DAT_10098204) != 0 {
            mech_trace::cr!(
                "10051700",
                "GetCursorPos",
                GetCursorPos((&raw mut local_8).cast())
            );
            mech_trace::cr!(
                "10051700",
                "ScreenToClient",
                ScreenToClient(
                    mech_trace::gr!("10051700", "100d08e8", DAT_100d08e8),
                    (&raw mut local_8).cast()
                )
            );
            if param_2 == 0 {
                local_8.y = (mech_trace::gr!("10051700", "100d08d4", DAT_100d08d4) / 2) as i32;
            } else if param_2 == 1 {
                local_8.x = (mech_trace::gr!("10051700", "100d08d8", DAT_100d08d8) / 2) as i32;
            }
            mech_trace::cr!(
                "10051700",
                "ClientToScreen",
                ClientToScreen(
                    mech_trace::gr!("10051700", "100d08e8", DAT_100d08e8),
                    (&raw mut local_8).cast()
                )
            );
            mech_trace::cr!(
                "10051700",
                "SetCursorPos",
                SetCursorPos(local_8.x, local_8.y)
            );
        }
    }
    mech_trace::rt!("10051700", 0)
}
