// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1006abdb();
    fn FUN_10007e8a();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006aeb1() {
    // SAFETY: both callees are the game's own functions and take no arguments.
    unsafe {
        FUN_1006abdb();
        FUN_10007e8a();
    }
}
