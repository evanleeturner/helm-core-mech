// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007440(param_1: *mut i32, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10008351(param_1: *mut i32) {
    mech_trace::enter!("10008351", param_1);
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_10007440(param_1, core::ptr::null_mut(), 0x50);
    }
}
