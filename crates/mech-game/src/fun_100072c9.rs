// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41e4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100072c9(param_1: u32) {
    mech_trace::enter!("100072c9", param_1);
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("100072c9", "100a41e4", DAT_100a41e4, param_1);
    }
}
