// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100603ff(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000ec6b() {
    // SAFETY: the callee is the game's own function; it takes a plain integer argument.
    unsafe {
        FUN_100603ff(0);
    }
}
