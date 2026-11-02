// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab050: *mut c_void;
    static mut DAT_100ab054: *mut c_void;
    static mut DAT_100ab060: *mut c_void;
    static mut DAT_100ab068: *mut c_void;
    static mut DAT_100ab06c: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f7d0(this: *mut c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006dfd0() {
    mech_trace::enter!("1006dfd0");
    // SAFETY: the callee and the globals are the game's own; each global
    // holds the object pointer the original passes as `this`.
    unsafe {
        mech_trace::cr!(
            "1006dfd0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("1006dfd0", "100ab050", DAT_100ab050), 0)
        );
        mech_trace::cr!(
            "1006dfd0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("1006dfd0", "100ab054", DAT_100ab054), 0)
        );
        mech_trace::cr!(
            "1006dfd0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("1006dfd0", "100ab060", DAT_100ab060), 0)
        );
        mech_trace::cr!(
            "1006dfd0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("1006dfd0", "100ab06c", DAT_100ab06c), 0)
        );
        mech_trace::cr!(
            "1006dfd0",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("1006dfd0", "100ab068", DAT_100ab068), 0)
        );
    }
}
