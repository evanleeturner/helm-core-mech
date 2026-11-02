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
    mech_trace::enter!("10010ac1", param_1);
    // SAFETY: all callees and globals are the game's own; the text pointer
    // returned by FUN_10010b46 is passed through unchanged to the OS and
    // back to the game's release routine.
    unsafe {
        FUN_1000ed10();
        let lp_text = mech_trace::cr!(
            "10010ac1",
            "10010b46",
            FUN_10010b46(
                mech_trace::gr!("10010ac1", "100a9e14", DAT_100a9e14),
                param_1
            )
        );
        FUN_10065cdb();
        FUN_10088e1a();
        loop {
            let i_var1 = mech_trace::cr!("10010ac1", "ShowCursor", ShowCursor(1));
            if i_var1 >= 0 {
                break;
            }
        }
        mech_trace::cr!(
            "10010ac1",
            "ShowWindow",
            ShowWindow(mech_trace::gr!("10010ac1", "100adf6c", DAT_100adf6c), 7)
        );
        mech_trace::cr!(
            "10010ac1",
            "MessageBoxA",
            MessageBoxA(
                core::ptr::null_mut(),
                lp_text,
                mech_trace::gr!("10010ac1", "100a9e70", DAT_100a9e70),
                0x12010
            )
        );
        FUN_10002168(lp_text);
        _exit(mech_trace::gr!("10010ac1", "100a9e1c", DAT_100a9e1c));
    }
}
