// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7fb0: u32;
    static mut DAT_100b39a8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068d10(param_1: u32, param_2: u32) {
    mech_trace::enter!("10068d10", param_1, param_2);
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        mech_trace::gw!("10068d10", "100c7fb0", DAT_100c7fb0, param_1);
        mech_trace::gw!("10068d10", "100b39a8", DAT_100b39a8, param_2);
    }
}
