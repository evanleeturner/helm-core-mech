// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad4c8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005f880(param_1: *mut c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100092e0() {
    mech_trace::enter!("100092e0");
    // SAFETY: the global and the callee belong to the game; the pointer is
    // released once and then cleared, as in the original.
    unsafe {
        if !mech_trace::gr!("100092e0", "100ad4c8", DAT_100ad4c8).is_null() {
            FUN_1005f880(mech_trace::gr!("100092e0", "100ad4c8", DAT_100ad4c8));
            mech_trace::gw!("100092e0", "100ad4c8", DAT_100ad4c8, core::ptr::null_mut());
        }
    }
}
