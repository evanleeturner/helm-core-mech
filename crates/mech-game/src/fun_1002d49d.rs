// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005aaf7(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002d49d(param_1: *mut i8, param_2: i32) -> i32 {
    mech_trace::enter!("1002d49d", param_1, param_2);
    let mut local_c: i32 = 0;
    let mut local_8: *mut i8 = param_1;
    // SAFETY: param_1 is a NUL-terminated string from the caller; the callee
    // is the game's own function.
    unsafe {
        while mech_trace::dr!("1002d49d", local_8) != 0 {
            let i_var1 = mech_trace::cr!(
                "1002d49d",
                "1005aaf7",
                FUN_1005aaf7(param_2, mech_trace::dr!("1002d49d", local_8) as i32)
            ) as i32;
            local_c = local_c.wrapping_add(i_var1);
            local_8 = local_8.add(1);
        }
    }
    mech_trace::rt!("1002d49d", local_c)
}
