// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10027c4c(param_1: i32);
    fn FUN_10027e20(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027c05(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's structure; the callees are the
    // game's own and receive the pointer as an integer, as the original does.
    unsafe {
        if param_1.read_unaligned() == 8 || param_1.read_unaligned() == 2 {
            FUN_10027c4c(param_1 as i32);
            if param_1.read_unaligned() == 8 {
                FUN_10027e20(param_1 as i32);
            }
        }
    }
}
