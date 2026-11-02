// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10063fbd(param_1: i32, param_2: *mut i32);
    fn FUN_10063f34(param_1: i32) -> i32;
    fn FUN_1002650e(param_1: *mut i32, param_2: i32);
    fn FUN_100640ee(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f4d(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
) {
    let mut local_48: [i32; 6] = [0; 6];
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the callees are the game's own; the locals are valid for the
    // writes they make, and the output pointers are the caller's.
    unsafe {
        FUN_10063fbd(param_1, local_48.as_mut_ptr());
        let i_var1 = FUN_10063f34(param_1);
        FUN_1002650e(local_48.as_mut_ptr(), i_var1);
        FUN_100640ee(
            param_1,
            &raw mut local_8,
            &raw mut local_c,
            &raw mut local_10,
        );
        param_2.write_unaligned(local_48[3].wrapping_sub(local_8));
        param_3.write_unaligned(local_48[4].wrapping_sub(local_c));
        param_4.write_unaligned(local_48[5].wrapping_sub(local_10));
    }
}
