// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10080f37();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10080f20() {
    mech_trace::enter!("10080f20");
    // SAFETY: the callee is the game's own function and takes no arguments.
    unsafe {
        FUN_10080f37();
    }
}
