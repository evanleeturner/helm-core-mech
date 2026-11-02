// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b8: i32;
    static mut DAT_100b04bc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046fc9(param_1: i32) -> u32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if param_1 < DAT_100b04b8 || DAT_100b04bc < param_1 {
            return 0;
        }
    }
    1
}
