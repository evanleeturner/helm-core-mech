// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d69b(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1004d69b", param_1, param_2);
    // SAFETY: the callee is the game's own function; calls keep the original order.
    unsafe {
        let i_var1 = mech_trace::cr!("1004d69b", "1004d5e3", FUN_1004d5e3(param_2));
        let i_var2 = mech_trace::cr!("1004d69b", "1004d5e3", FUN_1004d5e3(param_1));
        mech_trace::rt!("1004d69b", (i_var1 == i_var2) as i32)
    }
}
