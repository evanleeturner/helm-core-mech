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
    );
    fn FUN_1002f462(param_1: i32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f668(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) {
    let mut local_34: [i32; 12] = [0; 12];
    // SAFETY: the callees are the game's own; local_34 is a local buffer
    // passed by pointer as the original does.
    unsafe {
        FUN_1008984b(
            local_34.as_mut_ptr(),
            param_2,
            param_3,
            param_4,
            0,
            0,
            0,
            param_5,
        );
        FUN_1002f462(param_1, local_34.as_mut_ptr().cast::<u32>());
    }
}
