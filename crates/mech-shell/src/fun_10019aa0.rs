// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f5f8: *mut core::ffi::c_void;
    static mut DAT_1008f5fc: *mut core::ffi::c_void;
    static mut DAT_1008f600: *mut core::ffi::c_void;
    static mut DAT_1008f604: *mut core::ffi::c_void;
    static mut DAT_1008f608: *mut core::ffi::c_void;
    static mut DAT_1008f610: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f7d0(this: *mut core::ffi::c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10019aa0() {
    mech_trace::enter!("10019aa0");
    // SAFETY: the callee and the globals are the game's own; each global
    // holds the object pointer the callee expects as its `this`.
    unsafe {
        mech_trace::cr!(
            "10019aa0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10019aa0", "1008f5f8", DAT_1008f5f8), 0)
        );
        mech_trace::cr!(
            "10019aa0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10019aa0", "1008f5fc", DAT_1008f5fc), 0)
        );
        mech_trace::cr!(
            "10019aa0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10019aa0", "1008f600", DAT_1008f600), 0)
        );
        mech_trace::cr!(
            "10019aa0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10019aa0", "1008f604", DAT_1008f604), 0)
        );
        mech_trace::cr!(
            "10019aa0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10019aa0", "1008f608", DAT_1008f608), 0)
        );
        mech_trace::cr!(
            "10019aa0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10019aa0", "1008f610", DAT_1008f610), 0)
        );
    }
}
