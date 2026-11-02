// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000389e(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10007e2a();
    fn FUN_1006ab7b();
    fn FUN_1003ca49();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ae8d() {
    mech_trace::enter!("1006ae8d");
    // SAFETY: the callees are the game's own functions, called in the original order.
    unsafe {
        FUN_1000389e(0);
        FUN_10007e2a();
        FUN_1006ab7b();
        FUN_1003ca49();
    }
}
