// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c54: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10047052(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100470be() {
    // SAFETY: the global and the callee are the game's own; the global's
    // value is passed through unchanged.
    unsafe {
        FUN_10047052(DAT_100b3c54);
    }
}
