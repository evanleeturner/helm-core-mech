// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005aaf7(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002d49d(param_1: *mut i8, param_2: i32) -> i32 {
    let mut local_c: i32 = 0;
    let mut local_8: *mut i8 = param_1;
    // SAFETY: param_1 is a NUL-terminated string from the caller; the callee
    // is the game's own function.
    unsafe {
        while *local_8 != 0 {
            let i_var1 = FUN_1005aaf7(param_2, *local_8 as i32) as i32;
            local_c = local_c.wrapping_add(i_var1);
            local_8 = local_8.add(1);
        }
    }
    local_c
}
