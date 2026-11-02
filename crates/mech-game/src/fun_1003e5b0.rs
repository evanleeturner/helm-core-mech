// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003e938();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003e5b0() {
    // SAFETY: the callee is the game's own function and takes no arguments.
    unsafe {
        FUN_1003e938();
    }
}
