// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4bbc: unsafe extern "C" fn(u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10059ea0(param_1: u32) {
    // SAFETY: the global is the game's own function pointer, read by value
    // and called with the caller's argument.
    unsafe {
        (DAT_100a4bbc)(param_1);
    }
}
