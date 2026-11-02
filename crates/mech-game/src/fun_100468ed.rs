// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7cec: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100468ed(param_1: u32) {
    mech_trace::enter!("100468ed", param_1);
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("100468ed", "100c7cec", DAT_100c7cec, param_1);
    }
}
