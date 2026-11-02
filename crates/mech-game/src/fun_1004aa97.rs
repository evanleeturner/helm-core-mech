// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004ab46(
        param_1: u32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: u32,
        param_9: i32,
        param_10: i32,
        param_11: i32,
        param_12: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004aa97(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: u32,
    param_8: i32,
) {
    mech_trace::enter!(
        "1004aa97", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8
    );
    // SAFETY: the callee is the game's own function; arguments are forwarded as in the original.
    unsafe {
        mech_trace::cr!(
            "1004aa97",
            "1004ab46",
            FUN_1004ab46(
                param_1, 0, param_2, param_3, param_4, param_5, param_6, param_7, param_8, 0, 0, 0,
            )
        );
    }
}
