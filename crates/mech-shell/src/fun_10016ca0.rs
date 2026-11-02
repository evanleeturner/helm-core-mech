// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edfc: i32;
    static mut DAT_1008f16c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016ca0(param_1: i32) {
    mech_trace::enter!("10016ca0", param_1);
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the string address is passed through unchanged.
    unsafe {
        mech_trace::gw!("10016ca0", "1008edfc", DAT_1008edfc, param_1);
        if param_1 != 0 {
            mech_trace::gw!("10016ca0", "1008edfc", DAT_1008edfc, 0);
            return;
        }
        FUN_100640a0(&raw mut DAT_1008f16c as *mut i8);
    }
}
