// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adab0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032c10(param_1: i32) -> i32 {
    mech_trace::enter!("10032c10", param_1);
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "10032c10",
            mech_trace::gr!("10032c10", "100adab0", DAT_100adab0).wrapping_add(param_1)
        )
    }
}
