// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007d0a4(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081046() {
    // SAFETY: the callee is the game's own function; it takes a plain integer.
    unsafe {
        FUN_1007d0a4(-1);
    }
}
