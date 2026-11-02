// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003686(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: i32,
        param_6: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003658(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: i32,
) {
    mech_trace::enter!("10003658", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_10003686(param_1, param_2, param_3, param_4, param_5, 0x32);
    }
}
