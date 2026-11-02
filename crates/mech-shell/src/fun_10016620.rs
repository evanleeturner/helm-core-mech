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
    mech_trace::enter!("10016620", menu);
    // SAFETY: the Win32 calls and the globals are the game's own; the menu
    // handle is passed through unchanged from the caller.
    unsafe {
        mech_trace::cr!("10016620", "EnableMenuItem", EnableMenuItem(menu, 1, 0x400));
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c41, 0)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c99, 0)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c97, 0)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c72, 0)
        );
        let flags = (((mech_trace::gr!("10016620", "1009820c", DAT_1009820c) == 0) as i32)
            .wrapping_sub(1)
            & 8) as u32;
        mech_trace::cr!(
            "10016620",
            "CheckMenuItem",
            CheckMenuItem(menu, 0x9c72, flags)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c94, 0)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c4b, 0)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c96, 0)
        );
        mech_trace::cr!(
            "10016620",
            "EnableMenuItem",
            EnableMenuItem(menu, 0x9c9a, 0)
        );
        mech_trace::cr!(
            "10016620",
            "DrawMenuBar",
            DrawMenuBar(mech_trace::gr!("10016620", "100d08e8", DAT_100d08e8))
        );
    }
}
