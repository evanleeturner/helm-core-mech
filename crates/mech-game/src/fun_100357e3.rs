// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f4c: i32;
    static mut DAT_10160ef4: i32;
    static mut DAT_10160ef8: i32;
    static mut DAT_10160efc: i32;
    static mut DAT_10160f00: i32;
    static mut DAT_10160f04: i32;
    static mut DAT_10160f08: i32;
    static mut DAT_10160f0c: i32;
    static mut DAT_10160f10: i32;
    static mut DAT_10160f14: i32;
    static mut DAT_10168398: i32;
    static mut DAT_10160eb4: i32;
    static mut DAT_10160eb8: i32;
    static mut DAT_10160ec0: i32;
    static mut DAT_10160ec4: i32;
    static mut DAT_10160ed0: i32;
    static mut DAT_10160edc: i32;
    static mut DAT_10160ee0: i32;
    static mut DAT_10160ee8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a080(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_100359c0(param_1: u32, param_2: i32, param_3: u8, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100357e3(param_1: *mut i32) -> u32 {
    mech_trace::enter!("100357e3", param_1);
    // SAFETY: globals and callees are the game's own; param_1 points at three
    // readable and writable i32 values supplied by the caller.
    unsafe {
        let mut local_8: u32 = 0;
        let mut i_var1 = mech_trace::rd!("100357e3", param_1, read_unaligned)
            .wrapping_sub(mech_trace::gr!("100357e3", "10160f48", _DAT_10160f48));
        let mut i_var2 = mech_trace::rd!("100357e3", param_1.add(1), read_unaligned)
            .wrapping_sub(mech_trace::gr!("100357e3", "10160f44", _DAT_10160f44));
        let mut i_var3 = mech_trace::rd!("100357e3", param_1.add(2), read_unaligned)
            .wrapping_sub(mech_trace::gr!("100357e3", "10160f4c", _DAT_10160f4c));
        let u_var4 = mech_trace::cr!(
            "100357e3",
            "1001a080",
            FUN_1001a080(
                i_var1,
                mech_trace::gr!("100357e3", "10160ef4", DAT_10160ef4),
                i_var2,
                mech_trace::gr!("100357e3", "10160ef8", DAT_10160ef8),
                i_var3,
                mech_trace::gr!("100357e3", "10160efc", DAT_10160efc),
            )
        ) as u32;
        let u_var5 = mech_trace::cr!(
            "100357e3",
            "1001a080",
            FUN_1001a080(
                i_var1,
                mech_trace::gr!("100357e3", "10160f00", DAT_10160f00),
                i_var2,
                mech_trace::gr!("100357e3", "10160f04", DAT_10160f04),
                i_var3,
                mech_trace::gr!("100357e3", "10160f08", DAT_10160f08),
            )
        ) as u32;
        i_var1 = mech_trace::cr!(
            "100357e3",
            "1001a080",
            FUN_1001a080(
                i_var1,
                mech_trace::gr!("100357e3", "10160f0c", DAT_10160f0c),
                i_var2,
                mech_trace::gr!("100357e3", "10160f10", DAT_10160f10),
                i_var3,
                mech_trace::gr!("100357e3", "10160f14", DAT_10160f14),
            )
        );
        i_var2 = mech_trace::cr!(
            "100357e3",
            "100359c0",
            FUN_100359c0(
                u_var4,
                mech_trace::gr!("100357e3", "10168398", DAT_10168398),
                mech_trace::gr!("100357e3", "10160eb4", DAT_10160eb4) as u8,
                mech_trace::gr!("100357e3", "10160ec4", DAT_10160ec4)
            )
        );
        mech_trace::wr!("100357e3", param_1, write_unaligned, i_var2);
        i_var3 = mech_trace::gr!("100357e3", "10160ed0", DAT_10160ed0)
            .wrapping_sub(mech_trace::gr!("100357e3", "10160ee0", DAT_10160ee0));
        i_var2 = mech_trace::cr!(
            "100357e3",
            "100359c0",
            FUN_100359c0(
                u_var5,
                mech_trace::gr!("100357e3", "10168398", DAT_10168398),
                mech_trace::gr!("100357e3", "10160eb8", DAT_10160eb8) as u8,
                mech_trace::gr!("100357e3", "10160ee8", DAT_10160ee8)
            )
        );
        mech_trace::wr!(
            "100357e3",
            param_1.add(1),
            write_unaligned,
            i_var3.wrapping_sub(i_var2)
        );
        mech_trace::wr!("100357e3", param_1.add(2), write_unaligned, i_var1);
        if 0 < i_var1 {
            if mech_trace::rd!("100357e3", param_1, read_unaligned)
                < mech_trace::gr!("100357e3", "10160ec0", DAT_10160ec0)
                || mech_trace::gr!("100357e3", "10160edc", DAT_10160edc)
                    < mech_trace::rd!("100357e3", param_1, read_unaligned)
                || mech_trace::rd!("100357e3", param_1.add(1), read_unaligned)
                    < mech_trace::gr!("100357e3", "10160ee0", DAT_10160ee0)
                || mech_trace::gr!("100357e3", "10160ed0", DAT_10160ed0)
                    < mech_trace::rd!("100357e3", param_1.add(1), read_unaligned)
            {
                local_8 = 0;
            } else {
                local_8 = 1;
            }
        }
        mech_trace::rt!("100357e3", local_8)
    }
}
