// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b68: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ec35(param_1: i32) {
    mech_trace::enter!("1000ec35", param_1);
    // SAFETY: the global is the game's own memory; it is written once.
    unsafe {
        if !(0..=10).contains(&param_1) {
            mech_trace::gw!("1000ec35", "100a7b68", DAT_100a7b68, 0);
        } else {
            mech_trace::gw!("1000ec35", "100a7b68", DAT_100a7b68, param_1);
        }
    }
}
