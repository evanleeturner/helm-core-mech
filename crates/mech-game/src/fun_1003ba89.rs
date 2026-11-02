// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160f48: i32;
    static mut DAT_10160f44: i32;
    static mut DAT_10160f4c: i32;
    static mut DAT_10160ef4: i32;
    static mut DAT_10160ef8: i32;
    static mut DAT_10160efc: i32;
    static mut DAT_10160f00: i32;
    static mut DAT_10160f04: i32;
    static mut DAT_10160f08: i32;
    static mut DAT_10160f0c: i32;
    static mut DAT_10160f10: i32;
    static mut DAT_10160f14: i32;
    static mut DAT_10160eb0: i32;
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
    ) -> u32;
    fn FUN_100359c0(param_1: u32, param_2: i32, param_3: u8, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ba89(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
) -> u32 {
    mech_trace::enter!("1003ba89", param_1, param_2, param_3);
    // SAFETY: globals and callees are the game's own; the params point at
    // the caller's i32 values, read and written where the original does.
    unsafe {
        let i_var2 = (mech_trace::rd!("1003ba89", param_1, read_unaligned) as u32)
            .wrapping_sub(mech_trace::gr!("1003ba89", "10160f48", DAT_10160f48) as u32)
            as i32;
        let i_var3 = (mech_trace::rd!("1003ba89", param_2, read_unaligned) as u32)
            .wrapping_sub(mech_trace::gr!("1003ba89", "10160f44", DAT_10160f44) as u32)
            as i32;
        let i_var4 = (mech_trace::rd!("1003ba89", param_3, read_unaligned) as u32)
            .wrapping_sub(mech_trace::gr!("1003ba89", "10160f4c", DAT_10160f4c) as u32)
            as i32;
        let u_var5 = mech_trace::cr!(
            "1003ba89",
            "1001a080",
            FUN_1001a080(
                i_var2,
                mech_trace::gr!("1003ba89", "10160ef4", DAT_10160ef4),
                i_var3,
                mech_trace::gr!("1003ba89", "10160ef8", DAT_10160ef8),
                i_var4,
                mech_trace::gr!("1003ba89", "10160efc", DAT_10160efc),
            )
        );
        let u_var6 = mech_trace::cr!(
            "1003ba89",
            "1001a080",
            FUN_1001a080(
                i_var2,
                mech_trace::gr!("1003ba89", "10160f00", DAT_10160f00),
                i_var3,
                mech_trace::gr!("1003ba89", "10160f04", DAT_10160f04),
                i_var4,
                mech_trace::gr!("1003ba89", "10160f08", DAT_10160f08),
            )
        );
        let mut depth = mech_trace::cr!(
            "1003ba89",
            "1001a080",
            FUN_1001a080(
                i_var2,
                mech_trace::gr!("1003ba89", "10160f0c", DAT_10160f0c),
                i_var3,
                mech_trace::gr!("1003ba89", "10160f10", DAT_10160f10),
                i_var4,
                mech_trace::gr!("1003ba89", "10160f14", DAT_10160f14),
            )
        ) as i32;
        mech_trace::wr!("1003ba89", param_3, write_unaligned, depth);
        let too_near: bool;
        if mech_trace::gr!("1003ba89", "10160eb0", DAT_10160eb0) < depth {
            too_near = false;
        } else {
            too_near = true;
            if depth < 0 {
                depth = 0u32.wrapping_sub(depth as u32) as i32;
            } else if depth == 0 {
                depth = 1;
            }
        }
        let r = mech_trace::cr!(
            "1003ba89",
            "100359c0",
            FUN_100359c0(
                u_var5,
                depth,
                mech_trace::gr!("1003ba89", "10160eb4", DAT_10160eb4) as u8,
                mech_trace::gr!("1003ba89", "10160ec4", DAT_10160ec4)
            )
        );
        mech_trace::wr!("1003ba89", param_1, write_unaligned, r);
        let i_var3 = (mech_trace::gr!("1003ba89", "10160ed0", DAT_10160ed0) as u32)
            .wrapping_sub(mech_trace::gr!("1003ba89", "10160ee0", DAT_10160ee0) as u32)
            as i32;
        let r2 = mech_trace::cr!(
            "1003ba89",
            "100359c0",
            FUN_100359c0(
                u_var6,
                depth,
                mech_trace::gr!("1003ba89", "10160eb8", DAT_10160eb8) as u8,
                mech_trace::gr!("1003ba89", "10160ee8", DAT_10160ee8)
            )
        );
        mech_trace::wr!(
            "1003ba89",
            param_2,
            write_unaligned,
            (i_var3 as u32).wrapping_sub(r2 as u32) as i32
        );
        mech_trace::rt!(
            "1003ba89",
            if too_near {
                0
            } else if mech_trace::rd!("1003ba89", param_1, read_unaligned)
                < mech_trace::gr!("1003ba89", "10160ec0", DAT_10160ec0)
                || mech_trace::gr!("1003ba89", "10160edc", DAT_10160edc)
                    < mech_trace::rd!("1003ba89", param_1, read_unaligned)
                || mech_trace::rd!("1003ba89", param_2, read_unaligned)
                    < mech_trace::gr!("1003ba89", "10160ee0", DAT_10160ee0)
                || mech_trace::gr!("1003ba89", "10160ed0", DAT_10160ed0)
                    < mech_trace::rd!("1003ba89", param_2, read_unaligned)
            {
                0
            } else {
                1
            }
        )
    }
}
