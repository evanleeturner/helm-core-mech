// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41e8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100072dc(param_1: u32) {
    mech_trace::enter!("100072dc", param_1);
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("100072dc", "100a41e8", DAT_100a41e8, param_1);
    }
}
