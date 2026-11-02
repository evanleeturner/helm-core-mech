// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b50f(param_1: i32) -> u32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> i32;
    fn FUN_100663e5(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006657e(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    mech_trace::enter!("1006657e", param_1, param_2, param_3);
    // SAFETY: the callees are the game's own; param_1 points at a rect of
    // i32 fields read at the same offsets as the original.
    unsafe {
        let u_var1 = mech_trace::cr!("1006657e", "1005b50f", FUN_1005b50f(param_3));
        let r0 = mech_trace::rd!(
            "1006657e",
            ((param_1.wrapping_add(0xc)) as *const i32),
            read_unaligned
        );
        let r1 = mech_trace::rd!(
            "1006657e",
            ((param_1.wrapping_add(4)) as *const i32),
            read_unaligned
        );
        let i_var2 = mech_trace::cr!(
            "1006657e",
            "10002570",
            FUN_10002570(
                ((u_var1 as i32) >> 0x10) as u32,
                r0.wrapping_sub(r1).wrapping_add(1),
            )
        );
        let r2 = mech_trace::rd!(
            "1006657e",
            ((param_1.wrapping_add(0x10)) as *const i32),
            read_unaligned
        );
        let r3 = mech_trace::rd!(
            "1006657e",
            ((param_1.wrapping_add(8)) as *const i32),
            read_unaligned
        );
        let i_var3 = mech_trace::cr!(
            "1006657e",
            "10002570",
            FUN_10002570(u_var1 & 0xffff, r2.wrapping_sub(r3).wrapping_add(1))
        );
        mech_trace::cr!(
            "1006657e",
            "100663e5",
            FUN_100663e5(param_1, param_2, i_var2, i_var3)
        );
        mech_trace::rt!("1006657e", param_2)
    }
}
