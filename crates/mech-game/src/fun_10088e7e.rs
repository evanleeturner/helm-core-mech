// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100baa04: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10088e7e() {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_100baa04 = 0;
    }
}
