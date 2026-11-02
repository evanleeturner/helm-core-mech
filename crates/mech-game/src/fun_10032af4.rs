// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100329cf(
        param_1: f32,
        param_2: f32,
        param_3: f32,
        param_4: f32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut u32,
        param_8: *mut u32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032af4(
    param_1: f32,
    param_2: f32,
    param_3: f32,
    param_4: f32,
    param_5: f32,
    param_6: f32,
    param_7: f32,
    param_8: f32,
    param_9: f32,
    param_10: *mut u32,
    param_11: *mut u32,
    param_12: *mut u32,
    param_13: *mut u32,
) {
    let f_var1 =
        (param_9 - param_2) * (param_5 - param_2) - (param_6 - param_3) * (param_8 - param_2);
    let f_var2 =
        (param_4 - param_1) * (param_9 - param_2) - (param_6 - param_3) * (param_7 - param_1);
    let f_var3 =
        (param_4 - param_1) * (param_8 - param_2) - (param_5 - param_2) * (param_7 - param_1);
    // SAFETY: the callee is the game's own function; the output pointers are
    // passed through unchanged from the caller.
    unsafe {
        FUN_100329cf(
            f_var1,
            f_var2,
            f_var3,
            -(f_var2 * param_2 + f_var3 * param_3 + f_var1 * param_1),
            param_10,
            param_11,
            param_12,
            param_13,
        );
    }
}
