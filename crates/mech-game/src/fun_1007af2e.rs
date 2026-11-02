// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4ed0: u32;
    static mut DAT_100b4ed4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007af2e(param_1: u32, param_2: u32) {
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        DAT_100b4ed0 = param_1;
        DAT_100b4ed4 = param_2;
    }
}
