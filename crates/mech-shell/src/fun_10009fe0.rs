// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10009f90(param_1: u32, param_2: i32, param_3: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009fe0(param_1: u32, param_2: *mut i32, param_3: *mut i32) -> u32 {
    mech_trace::enter!("10009fe0", param_1, param_2, param_3);
    let mut local_4: i32 = 0;
    let mut i_var2: i32 = 0;
    loop {
        // SAFETY: the callee is the game's own; local_4 is a valid local out-pointer.
        let i_var1 = unsafe {
            mech_trace::cr!(
                "10009fe0",
                "10009f90",
                FUN_10009f90(param_1, i_var2, &raw mut local_4)
            )
        } as i32;
        if i_var1 != 0 {
            // SAFETY: callers pass either null or valid pointers for the outputs.
            unsafe {
                if !param_2.is_null() {
                    mech_trace::wr!("10009fe0", param_2, write_unaligned, i_var2);
                }
                if !param_3.is_null() {
                    mech_trace::wr!("10009fe0", param_3, write_unaligned, local_4);
                }
            }
            return mech_trace::rt!("10009fe0", 1);
        }
        i_var2 = i_var2.wrapping_add(1);
        if i_var2 >= 8 {
            break;
        }
    }
    mech_trace::rt!("10009fe0", 0)
}
