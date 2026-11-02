// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086973(param_1: *mut u32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100897ef(param_1: *mut u32, param_2: *mut u32) {
    mech_trace::enter!("100897ef", param_1, param_2);
    // SAFETY: the callee is the game's own; the pointers are passed through
    // unchanged with the swapped order the original uses.
    unsafe {
        FUN_10086973(param_2, param_1, 0x30);
    }
}
