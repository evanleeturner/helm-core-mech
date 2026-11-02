// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098f88: *mut c_void;
    static mut DAT_10098f8c: *mut c_void;
    static mut DAT_10098f90: *mut c_void;
    static mut DAT_10098f94: *mut c_void;
    static mut DAT_10098f98: *mut c_void;
    static mut DAT_10098fa0: *mut c_void;
    static mut DAT_10098fa4: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f7d0(this: *mut c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10046900() {
    mech_trace::enter!("10046900");
    // SAFETY: the callee and the globals are the game's own; each global
    // holds the object pointer the original passes as `this`.
    unsafe {
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098f88", DAT_10098f88), 0)
        );
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098f8c", DAT_10098f8c), 0)
        );
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098f90", DAT_10098f90), 0)
        );
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098f94", DAT_10098f94), 0)
        );
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098f98", DAT_10098f98), 0)
        );
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098fa0", DAT_10098fa0), 0)
        );
        mech_trace::cr!(
            "10046900",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10046900", "10098fa4", DAT_10098fa4), 0)
        );
    }
}
