// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9e14: *mut i8;
    static mut DAT_100adf6c: *mut core::ffi::c_void;
    static mut DAT_100a9e1c: i32;
    static mut DAT_100a9e70: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000ed10();
    fn FUN_10010b46(param_1: *mut i8, param_2: u32) -> *mut i8;
    fn FUN_10065cdb();
    fn FUN_10088e1a();
    fn FUN_10002168(param_1: *mut i8);
    fn _exit(code: i32) -> !;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ShowCursor(param_1: i32) -> i32;
    fn ShowWindow(param_1: *mut core::ffi::c_void, param_2: i32) -> i32;
    fn MessageBoxA(
        param_1: *mut core::ffi::c_void,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10010ac1(param_1: u32) {
    // SAFETY: all callees and globals are the game's own; the text pointer
    // returned by FUN_10010b46 is passed through unchanged to the OS and
    // back to the game's release routine.
    unsafe {
        FUN_1000ed10();
        let lp_text = FUN_10010b46(DAT_100a9e14, param_1);
        FUN_10065cdb();
        FUN_10088e1a();
        loop {
            let i_var1 = ShowCursor(1);
            if i_var1 >= 0 {
                break;
            }
        }
        ShowWindow(DAT_100adf6c, 7);
        MessageBoxA(core::ptr::null_mut(), lp_text, DAT_100a9e70, 0x12010);
        FUN_10002168(lp_text);
        _exit(DAT_100a9e1c);
    }
}
