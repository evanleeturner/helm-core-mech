// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafc0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002cd49(param_1: u32) {
    // SAFETY: the global is the game's own memory, read and written in place.
    unsafe {
        DAT_100aafc0 = DAT_100aafc0 ^ param_1;
    }
}
