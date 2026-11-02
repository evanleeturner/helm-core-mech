// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6814: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d001(param_1: i32) {
    mech_trace::enter!("1007d001", param_1);
    let mut param_1 = param_1;
    if param_1 < 1 {
        param_1 = 0;
    } else if 0x80 < param_1 {
        param_1 = 0x80;
    }
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("1007d001", "100b6814", DAT_100b6814, param_1);
    }
}
