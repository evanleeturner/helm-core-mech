// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_100594ec(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067389(param_1: *mut i32, param_2: i32) {
    mech_trace::enter!("10067389", param_1, param_2);
    // SAFETY: param_1 points at the caller's int array, the global and the
    // callees are the game's own; reads and calls keep the original order.
    unsafe {
        let i_var1 = mech_trace::rd!("10067389", param_1.add(3), read_unaligned);
        let i_var2 = mech_trace::rd!("10067389", param_1.add(1), read_unaligned);
        let i_var3 = mech_trace::rd!("10067389", param_1.add(4), read_unaligned);
        let i_var4 = mech_trace::rd!("10067389", param_1.add(2), read_unaligned);
        let i_var5 = (mech_trace::rd!("10067389", param_1.add(3), read_unaligned)
            .wrapping_sub(mech_trace::rd!("10067389", param_1.add(1), read_unaligned))
            .wrapping_add(1)
            / 2)
        .wrapping_sub(1);
        let base = mech_trace::gr!("10067389", "100aaf68", DAT_100aaf68) as usize as *const u8;
        let i_var6 = mech_trace::cr!(
            "10067389",
            "10003d60",
            FUN_10003d60(
                i_var5,
                mech_trace::rd!(
                    "10067389",
                    base.byte_add(0x44).cast::<i32>(),
                    read_unaligned
                )
            )
        );
        mech_trace::cr!(
            "10067389",
            "100594ec",
            FUN_100594ec(
                param_1,
                i_var1.wrapping_sub(i_var2).wrapping_add(1) >> 1,
                i_var3.wrapping_sub(i_var4).wrapping_add(1) >> 1,
                i_var5,
                i_var6,
                param_2,
            )
        );
    }
}
