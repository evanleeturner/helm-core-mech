// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009820c: u32;
    static mut DAT_100d08e8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn EnableMenuItem(menu: *mut c_void, id: u32, flags: u32) -> i32;
    fn CheckMenuItem(menu: *mut c_void, id: u32, flags: u32) -> u32;
    fn DrawMenuBar(hwnd: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016620(menu: *mut c_void) {
    // SAFETY: the Win32 calls and the globals are the game's own; the menu
    // handle is passed through unchanged from the caller.
    unsafe {
        EnableMenuItem(menu, 1, 0x400);
        EnableMenuItem(menu, 0x9c41, 0);
        EnableMenuItem(menu, 0x9c99, 0);
        EnableMenuItem(menu, 0x9c97, 0);
        EnableMenuItem(menu, 0x9c72, 0);
        let flags = (((DAT_1009820c == 0) as i32).wrapping_sub(1) & 8) as u32;
        CheckMenuItem(menu, 0x9c72, flags);
        EnableMenuItem(menu, 0x9c94, 0);
        EnableMenuItem(menu, 0x9c4b, 0);
        EnableMenuItem(menu, 0x9c96, 0);
        EnableMenuItem(menu, 0x9c9a, 0);
        DrawMenuBar(DAT_100d08e8);
    }
}
