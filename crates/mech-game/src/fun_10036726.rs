// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036142(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036726(param_1: i32) -> i32 {
    mech_trace::enter!("10036726", param_1);
    let mut param_1 = param_1;
    // SAFETY: the callee is the game's own function.
    unsafe {
        let i_var1 = mech_trace::cr!("10036726", "10036142", FUN_10036142(1));
        let i_var2 = mech_trace::cr!("10036726", "10036142", FUN_10036142(2));
        if param_1 == 0 {
            if i_var1 == 0 || i_var2 != 0 {
                if i_var1 == 0 && i_var2 != 0 {
                    param_1 = 2;
                } else if i_var1 == 0 && i_var2 == 0 {
                    param_1 = -1;
                }
            } else {
                param_1 = 1;
            }
        } else if param_1 == 1 && i_var1 == 0 {
            param_1 = -1;
        } else if param_1 == 2 && i_var2 == 0 {
            param_1 = -1;
        }
    }
    mech_trace::rt!("10036726", param_1)
}
