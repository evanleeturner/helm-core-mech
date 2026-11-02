// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafc0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002cd87(param_1: u32, enable: i32) {
    // SAFETY: the global is the game's own memory, read and written where the original does.
    unsafe {
        if enable == 0 {
            DAT_100aafc0 = DAT_100aafc0 | param_1;
        } else {
            DAT_100aafc0 = DAT_100aafc0 & !param_1;
        }
    }
}
