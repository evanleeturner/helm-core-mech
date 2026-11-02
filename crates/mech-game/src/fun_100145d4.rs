// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10014642(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_10014718(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100145d4(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut i32,
) -> u32 {
    mech_trace::enter!("100145d4", param_1, param_2, param_3, param_4);
    // SAFETY: the callees and the global are the game's own; param_4 points
    // at a caller-provided i32 that is written and read back as the original does.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "100145d4",
            "10014642",
            FUN_10014642(
                mech_trace::gr!("100145d4", "100af098", DAT_100af098) as i32,
                param_1,
                param_2,
                param_3
            )
        );
        mech_trace::wr!("100145d4", param_4, write_unaligned, i_var1);
        if mech_trace::rd!("100145d4", param_4, read_unaligned) != 0 {
            let r = mech_trace::cr!(
                "100145d4",
                "10014718",
                FUN_10014718(
                    mech_trace::rd!("100145d4", param_4, read_unaligned),
                    param_1 as u32,
                    param_2 as u32,
                    param_3 as u32,
                )
            );
            if r != 0 {
                return mech_trace::rt!("100145d4", 1);
            }
        }
        mech_trace::rt!("100145d4", 0)
    }
}
