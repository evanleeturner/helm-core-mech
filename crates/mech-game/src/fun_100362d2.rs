// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004cfea(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100362d2() {
    // SAFETY: the callee and the global are the game's own; the global is
    // read by value and passed on as the original does.
    unsafe {
        FUN_1004cfea(DAT_100b0710);
    }
}
