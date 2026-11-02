// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[repr(C)]
pub struct tagPOINT {
    pub x: i32,
    pub y: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100adf78: u32;
    static mut DAT_100adf7c: u32;
    static mut DAT_100adf80: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetCursorPos(param_1: *mut c_void) -> i32;
    fn ScreenToClient(param_1: *mut c_void, param_2: *mut c_void) -> i32;
    fn ClientToScreen(param_1: *mut c_void, param_2: *mut c_void) -> i32;
    fn SetCursorPos(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ed0c(_param_1: u32, param_2: i32) -> u32 {
    let mut local_c = tagPOINT { x: 0, y: 0 };
    // SAFETY: the globals and the Win32 calls are the game's own; local_c is a
    // valid POINT on this stack frame for each call.
    unsafe {
        if DAT_100adf80 != 0 {
            let p = (&raw mut local_c).cast::<c_void>();
            GetCursorPos(p);
            ScreenToClient(DAT_100adf6c, p);
            if param_2 == 0 {
                local_c.y = (DAT_100adf7c / 2) as i32;
            } else if param_2 == 1 {
                local_c.x = (DAT_100adf78 / 2) as i32;
            }
            ClientToScreen(DAT_100adf6c, p);
            SetCursorPos(local_c.x, local_c.y);
        }
    }
    0
}
