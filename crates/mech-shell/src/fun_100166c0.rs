// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d08e8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn EnableMenuItem(menu: *mut core::ffi::c_void, id: u32, flags: u32) -> i32;
    fn DrawMenuBar(hwnd: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100166c0(menu: *mut core::ffi::c_void) {
    // SAFETY: the Win32 calls and the global window handle are the game's own;
    // the menu handle is passed through unchanged.
    unsafe {
        EnableMenuItem(menu, 1, 0x401);
        EnableMenuItem(menu, 0x9c41, 1);
        EnableMenuItem(menu, 0x9c99, 1);
        EnableMenuItem(menu, 0x9c97, 1);
        EnableMenuItem(menu, 0x9c72, 1);
        EnableMenuItem(menu, 0x9c94, 1);
        EnableMenuItem(menu, 0x9c4b, 1);
        EnableMenuItem(menu, 0x9c96, 1);
        EnableMenuItem(menu, 0x9c9a, 1);
        DrawMenuBar(DAT_100d08e8);
    }
}
