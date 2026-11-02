// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fc40(param_1: i32) -> u32;
    fn FUN_1002fc56(param_1: i32) -> u32;
    fn FUN_1007c066(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c203(param_1: *mut i32, param_2: *mut u8, param_3: i32) {
    mech_trace::enter!("1007c203", param_1, param_2, param_3);
    if !param_1.is_null() {
        // SAFETY: the callees are the game's own functions; param_1 is a
        // non-null node pointer passed through as the original does.
        unsafe {
            let p_var1 =
                mech_trace::cr!("1007c203", "1002fc40", FUN_1002fc40(param_1 as i32)) as *mut i32;
            if !p_var1.is_null() {
                FUN_1007c203(p_var1, param_2, param_3);
            }
            let p_var1 =
                mech_trace::cr!("1007c203", "1002fc56", FUN_1002fc56(param_1 as i32)) as *mut i32;
            if !p_var1.is_null() {
                FUN_1007c203(p_var1, param_2, param_3);
            }
            mech_trace::cr!(
                "1007c203",
                "1007c066",
                FUN_1007c066(param_1, param_2, param_3)
            );
        }
    }
}
