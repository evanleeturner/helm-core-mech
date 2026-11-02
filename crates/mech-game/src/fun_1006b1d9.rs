// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006065d(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006b1d9() {
    // SAFETY: the callee is the game's own function; it takes plain integer arguments.
    unsafe {
        FUN_1006065d(0x11, 0x16a, 1);
    }
}
