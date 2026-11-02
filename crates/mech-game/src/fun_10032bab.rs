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
pub unsafe extern "C" fn FUN_10032bab(
    param_1: f32,
    param_2: f32,
    param_3: f32,
    param_4: f32,
    param_5: f32,
    param_6: f32,
    param_7: *mut u32,
    param_8: *mut u32,
    param_9: *mut u32,
    param_10: *mut u32,
) {
    // SAFETY: the callee is the game's own function; the pointers are passed
    // through unchanged.
    unsafe {
        FUN_100329cf(
            param_4,
            -param_5,
            param_6,
            -(param_1 * param_4 + param_2 * -param_5 + param_3 * param_6),
            param_7,
            param_8,
            param_9,
            param_10,
        );
    }
}
