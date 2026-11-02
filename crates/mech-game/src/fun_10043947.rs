// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043947(param_1: i32, param_2: i16) -> i32 {
    mech_trace::enter!("10043947", param_1, param_2);
    // SAFETY: the callees are the game's own functions, called in the original order.
    unsafe {
        FUN_1002950f(param_1, param_2);
        let i_var1 = mech_trace::cr!("10043947", "1002a132", FUN_1002a132(param_1));
        FUN_10022b70(param_1, i_var1);
    }
    mech_trace::rt!("10043947", 0)
}
