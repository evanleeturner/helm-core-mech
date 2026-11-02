// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d8: unsafe extern "C" fn(u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001df8e(param_1: u32) {
    // SAFETY: the global holds the game's own function pointer; it is read
    // and called with the caller's argument, as the original does.
    unsafe {
        (DAT_100ab2d8)(param_1);
    }
}
