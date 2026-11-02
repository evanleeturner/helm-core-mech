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
    mech_trace::enter!("10007e8a");
    // SAFETY: the globals and callees are the game's own; the table write
    // goes where the original computes it from DAT_100a40f8.
    unsafe {
        if mech_trace::gr!("10007e8a", "100a5538", DAT_100a5538) != 0 {
            let i_var1 = mech_trace::cr!(
                "10007e8a",
                "10003d60",
                FUN_10003d60(mech_trace::gr!("10007e8a", "100b3c50", DAT_100b3c50), 100)
            );
            let i_var2 = mech_trace::cr!(
                "10007e8a",
                "AIL_sample_user_data",
                AIL_sample_user_data(mech_trace::gr!("10007e8a", "100a5538", DAT_100a5538), 2)
            );
            if -1 < i_var2 {
                let addr = (mech_trace::gr!("10007e8a", "100a40f8", DAT_100a40f8) as u32)
                    .wrapping_add(0x468)
                    .wrapping_add((i_var2 as u32).wrapping_mul(4));
                mech_trace::wr!("10007e8a", (addr as *mut i32), write_unaligned, i_var1);
            }
            AIL_set_sample_volume(
                mech_trace::gr!("10007e8a", "100a5538", DAT_100a5538),
                i_var1,
            );
            AIL_resume_sample(mech_trace::gr!("10007e8a", "100a5538", DAT_100a5538));
        }
    }
}
