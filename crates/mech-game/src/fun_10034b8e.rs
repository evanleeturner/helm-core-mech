// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063fbd(param_1: i32, param_2: *mut i32);
    fn FUN_10063f34(param_1: i32) -> i32;
    fn FUN_1002650e(param_1: *mut i32, param_2: i32);
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10034b8e(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
) -> i32 {
    let mut local_48: [i32; 14] = [0; 14];
    let mut local_10: i32;
    let mut local_c: i32;
    let mut local_8: i32;
    // SAFETY: the callees are the game's own; param_1, param_2 and param_3
    // point at the caller's integers, read and written where the original does.
    unsafe {
        FUN_10063fbd(param_1.read(), local_48.as_mut_ptr());
        let i_var1 = FUN_10063f34(param_1.read());
        FUN_1002650e(local_48.as_mut_ptr(), i_var1);
        local_8 = local_48[3];
        local_c = local_48[4];
        local_10 = local_48[5];
        let u_var2 = FUN_1003ba89(&raw mut local_8, &raw mut local_c, &raw mut local_10);
        param_2.write(local_8);
        param_3.write(local_c);
        u_var2
    }
}
