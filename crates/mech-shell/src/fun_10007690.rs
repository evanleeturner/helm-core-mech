// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad4c8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005f810(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10007690() {
    // SAFETY: the global and the callee are the game's own; the pointer is
    // passed on only when non-null.
    unsafe {
        if !DAT_100ad4c8.is_null() {
            FUN_1005f810(DAT_100ad4c8);
        }
    }
}
