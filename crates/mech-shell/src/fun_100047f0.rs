// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10004550();
    fn FUN_10004620();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100047f0() {
    mech_trace::enter!("100047f0");
    // SAFETY: the callees are the game's own functions and take no arguments.
    unsafe {
        FUN_10004550();
        FUN_10004620();
    }
}
