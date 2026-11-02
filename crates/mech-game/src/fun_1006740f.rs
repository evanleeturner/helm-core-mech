// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
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
pub unsafe extern "C" fn FUN_1006740f(param_1: *mut i32, param_2: i32, param_3: u32) {
    mech_trace::enter!("1006740f", param_1, param_2, param_3);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's int array and the global pointer refers to the game's data.
    unsafe {
        let i_var8 = mech_trace::rd!(
            "1006740f",
            ((mech_trace::gr!("1006740f", "100aaf68", DAT_100aaf68) as usize as *const u8)
                .byte_add(0x44)
                .cast::<i32>()),
            read_unaligned
        );
        let i_var1 = mech_trace::rd!("1006740f", param_1.add(3), read_unaligned);
        let i_var2 = mech_trace::rd!("1006740f", param_1.add(1), read_unaligned);
        let i_var3 = mech_trace::rd!("1006740f", param_1.add(4), read_unaligned);
        let i_var4 = mech_trace::rd!("1006740f", param_1.add(2), read_unaligned);
        let i_var7 = mech_trace::rd!("1006740f", param_1.add(3), read_unaligned);
        let i_var5 = mech_trace::rd!("1006740f", param_1.add(1), read_unaligned);
        let i_var6 =
            mech_trace::cr!("1006740f", "10002570", FUN_10002570(param_3, 0x640000)) as i32;
        let i_var7 = mech_trace::cr!(
            "1006740f",
            "10003d60",
            FUN_10003d60(
                (i_var7.wrapping_sub(i_var5).wrapping_add(1) / 2).wrapping_sub(1),
                i_var6,
            )
        );
        let i_var8 = mech_trace::cr!("1006740f", "10003d60", FUN_10003d60(i_var7, i_var8));
        mech_trace::cr!(
            "1006740f",
            "100594ec",
            FUN_100594ec(
                param_1,
                i_var1.wrapping_sub(i_var2).wrapping_add(1) >> 1,
                i_var3.wrapping_sub(i_var4).wrapping_add(1) >> 1,
                i_var7,
                i_var8,
                param_2,
            )
        );
    }
}
