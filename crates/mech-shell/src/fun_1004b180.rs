// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100a6798: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004b0f0() -> i32;
    fn FUN_1004b140() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004b180(param_1: i8) -> i32 {
    mech_trace::enter!("1004b180", param_1);
    // SAFETY: the globals hold pointers to the game's own structures; the
    // offsets are read and written in the original's order.
    unsafe {
        let i_var8 = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x23a3))
                as *const i32),
            read_unaligned
        );
        let i_var1 = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x23a7))
                as *const i32),
            read_unaligned
        );
        let i_var2 = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "100a6798", DAT_100a6798).wrapping_add(4)) as *const i32),
            read_unaligned
        );
        let i_var3 = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x23af))
                as *const i32),
            read_unaligned
        );
        let i_var4 = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x23b3))
                as *const i32),
            read_unaligned
        );
        let i_var5 = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x23ab))
                as *const i32),
            read_unaligned
        );
        let i_var6 = mech_trace::cr!("1004b180", "1004b0f0", FUN_1004b0f0());
        let i_var7 = mech_trace::cr!("1004b180", "1004b140", FUN_1004b140());
        let sum = i_var7
            .wrapping_add(i_var8.wrapping_add(i_var1).wrapping_mul(i_var2))
            .wrapping_add(i_var3)
            .wrapping_add(i_var4)
            .wrapping_add(i_var5)
            .wrapping_add(1)
            .wrapping_add(i_var6);
        let factor = mech_trace::rd!(
            "1004b180",
            ((mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x1fa))
                as *const i32),
            read_unaligned
        );
        let result = factor.wrapping_mul(sum);
        if param_1 == 1 {
            let p = (mech_trace::gr!("1004b180", "10083508", DAT_10083508).wrapping_add(0x12a))
                as *mut i32;
            mech_trace::wr!(
                "1004b180",
                p,
                write_unaligned,
                mech_trace::rd!("1004b180", p, read_unaligned).wrapping_sub(result)
            );
        }
        mech_trace::rt!("1004b180", result)
    }
}
