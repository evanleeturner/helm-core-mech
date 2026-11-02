// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100240be(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081009() {
    // SAFETY: the callee is the game's own function; it takes one int by value.
    unsafe {
        FUN_100240be(-1);
    }
}
