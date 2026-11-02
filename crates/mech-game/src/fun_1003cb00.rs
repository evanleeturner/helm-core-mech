// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003cb00() {
    // SAFETY: the global is the game's own memory.
    unsafe {
        DAT_100aefc0 = 1;
    }
}
