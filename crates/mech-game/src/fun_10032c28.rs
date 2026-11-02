// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adab0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032c28(param_1: i32) {
    mech_trace::enter!("10032c28", param_1);
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("10032c28", "100adab0", DAT_100adab0, param_1);
    }
}
