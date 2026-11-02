// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7ce8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046915(param_1: u32) {
    mech_trace::enter!("10046915", param_1);
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("10046915", "100c7ce8", DAT_100c7ce8, param_1);
    }
}
