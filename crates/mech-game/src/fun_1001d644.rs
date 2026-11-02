// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003dce5(param_1: *mut core::ffi::c_void);
    fn FUN_1001c989(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d644(param_1: *mut core::ffi::c_void) {
    mech_trace::enter!("1001d644", param_1);
    if !param_1.is_null() {
        // SAFETY: the callees are the game's own functions; param_1 is
        // non-null and passed through unchanged, in the original order.
        unsafe {
            FUN_1003dce5(param_1);
            FUN_1001c989(param_1);
        }
    }
}
