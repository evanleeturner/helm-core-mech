// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3924: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a37(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065e16() {
    // SAFETY: the callee and the global are the game's own.
    unsafe {
        FUN_10036a37(DAT_100b3924);
    }
}
