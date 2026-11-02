// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b398c: u32;
    static mut DAT_100b3990: u32;
    static mut DAT_100b3994: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068d2b(param_1: u32, param_2: u32, param_3: u32) {
    // SAFETY: the globals are the game's own memory; each is written once
    // in the original's order.
    unsafe {
        DAT_100b398c = param_1;
        DAT_100b3990 = param_2;
        DAT_100b3994 = param_3;
    }
}
