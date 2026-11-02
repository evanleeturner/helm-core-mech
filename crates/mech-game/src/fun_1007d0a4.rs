// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6824: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d0a4(param_1: i32) -> i32 {
    let mut param_1 = param_1;
    // SAFETY: the global is the game's own memory, read and written where the original does.
    unsafe {
        if param_1 == -1 {
            param_1 = DAT_100b6824;
        }
        DAT_100b6824 = param_1;
    }
    param_1
}
