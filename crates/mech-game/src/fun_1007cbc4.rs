// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f727(param_1: *mut i32);
    fn FUN_1002f7f7(param_1: *mut i32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007cbc4(param_1: *mut i32) {
    mech_trace::enter!("1007cbc4", param_1);
    if !param_1.is_null() {
        // SAFETY: the callees are the game's own functions; param_1 is
        // non-null and passed through unchanged, in the original order.
        unsafe {
            FUN_1002f727(param_1);
            FUN_1002f7f7(param_1);
            FUN_1002fbc0(param_1);
        }
    }
}
