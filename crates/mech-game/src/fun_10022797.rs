// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d114(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022797(param_1: *mut i32) {
    mech_trace::enter!("10022797", param_1);
    if !param_1.is_null() {
        // SAFETY: the callee is the game's own function; param_1 is non-null
        // and passed through unchanged.
        unsafe {
            FUN_1005d114(param_1);
        }
    }
}
