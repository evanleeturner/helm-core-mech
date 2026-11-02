// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10032959(
        param_1: f32,
        param_2: f32,
        param_3: f32,
        param_4: *mut u32,
        param_5: *mut u32,
        param_6: *mut u32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032a60(
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
) {
    let a = (param_9 - param_2) * (param_5 - param_2) - (param_8 - param_2) * (param_6 - param_3);
    let b = (param_9 - param_2) * (param_4 - param_1) - (param_7 - param_1) * (param_6 - param_3);
    let c = (param_8 - param_2) * (param_4 - param_1) - (param_7 - param_1) * (param_5 - param_2);
    // SAFETY: the callee is the game's own; the output pointers are passed
    // through unchanged from the caller.
    unsafe {
        FUN_10032959(a, b, c, param_10, param_11, param_12);
    }
}
