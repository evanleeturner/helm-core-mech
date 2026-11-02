// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9ec4: i32;
    static mut DAT_100c9ec8: i32;
    static mut DAT_100c9ec0: i32;
    static mut DAT_100c96b0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d21a(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: the globals are the game's own memory; each is written once
    // in the original's order.
    unsafe {
        DAT_100c9ec4 = param_1;
        DAT_100c9ec8 = param_2;
        DAT_100c9ec0 = param_3;
        DAT_100c96b0 = param_4;
    }
}
