// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008984b(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: u32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10089cc9(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    mech_trace::enter!(
        "10089cc9", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        mech_trace::cr!(
            "10089cc9",
            "1008984b",
            FUN_1008984b(
                param_1, param_2, param_3, param_4, param_5, param_6, param_7, 0,
            )
        );
    }
}
