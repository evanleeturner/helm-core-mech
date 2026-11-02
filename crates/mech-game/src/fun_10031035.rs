// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada6c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031035(param_1: i32) {
    let mut param_1 = param_1;
    if param_1 < 1 {
        param_1 = 0;
    } else if 0xf < param_1 {
        param_1 = 0xf;
    }
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_100ada6c = param_1;
    }
}
