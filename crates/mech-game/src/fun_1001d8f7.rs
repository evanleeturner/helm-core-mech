// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100823ef(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001d8f7() {
    mech_trace::enter!("1001d8f7");
    // SAFETY: the callee is the game's own function; arguments are passed as the original does.
    unsafe {
        FUN_100823ef(0, 0x100, 0x101e86b0);
    }
}
