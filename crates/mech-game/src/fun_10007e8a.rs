// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5538: i32;
    static mut DAT_100b3c50: i32;
    static mut DAT_100a40f8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn AIL_sample_user_data(param_1: i32, param_2: i32) -> i32;
    fn AIL_set_sample_volume(param_1: i32, param_2: i32);
    fn AIL_resume_sample(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10007e8a() {
    // SAFETY: the globals and callees are the game's own; the table write
    // goes where the original computes it from DAT_100a40f8.
    unsafe {
        if DAT_100a5538 != 0 {
            let i_var1 = FUN_10003d60(DAT_100b3c50, 100);
            let i_var2 = AIL_sample_user_data(DAT_100a5538, 2);
            if -1 < i_var2 {
                let addr = (DAT_100a40f8 as u32)
                    .wrapping_add(0x468)
                    .wrapping_add((i_var2 as u32).wrapping_mul(4));
                (addr as *mut i32).write_unaligned(i_var1);
            }
            AIL_set_sample_volume(DAT_100a5538, i_var1);
            AIL_resume_sample(DAT_100a5538);
        }
    }
}
