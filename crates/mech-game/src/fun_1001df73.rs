// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d8: u32;
    static mut DAT_100ab2dc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001df73(param_1: u32, param_2: u32) {
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        DAT_100ab2d8 = param_1;
        DAT_100ab2dc = param_2;
    }
}
