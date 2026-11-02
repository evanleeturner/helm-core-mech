// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082d08: *mut i32;
    static mut DAT_100ad4f8: *mut c_void;
    static mut DAT_100ad504: *mut c_void;
    static mut DAT_100ad6d0: *mut c_void;
    static mut DAT_100ae10c: *mut c_void;
    static mut DAT_100ae114: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005f890(param_1: *mut c_void);
    fn FUN_1005f810(param_1: *mut c_void);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063ea0(param_1: *mut i32);
    fn FUN_10063d30(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10005a60(param_1: i32) {
    mech_trace::enter!("10005a60", param_1);
    // SAFETY: param_1 is the caller's object address; the globals and callees
    // are the game's own, used in the original's order.
    unsafe {
        let pi_var1 = mech_trace::rd!(
            "10005a60",
            (param_1.wrapping_add(0x28) as *const *mut i32),
            read_unaligned
        );
        FUN_1005f890(mech_trace::gr!("10005a60", "100ae114", DAT_100ae114));
        FUN_1005f890(mech_trace::gr!("10005a60", "100ad4f8", DAT_100ad4f8));
        FUN_1005f890(mech_trace::gr!("10005a60", "100ad504", DAT_100ad504));
        FUN_1005f890(mech_trace::gr!("10005a60", "100ae10c", DAT_100ae10c));
        FUN_1005f810(mech_trace::gr!("10005a60", "100ad6d0", DAT_100ad6d0));
        FUN_10063ea0(mech_trace::gr!("10005a60", "10082d08", DAT_10082d08));
        mech_trace::gw!("10005a60", "10082d08", DAT_10082d08, pi_var1);
        FUN_10063d30(pi_var1);
    }
}
