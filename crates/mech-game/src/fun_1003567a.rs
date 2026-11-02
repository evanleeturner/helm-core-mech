// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101150f8: u8;
    static mut DAT_10115100: [u32; 20];
    static mut DAT_1011515c: u32;
    static mut DAT_10115160: u32;
    static mut DAT_10115168: u8;
    static mut DAT_10160eb4: u32;
    static mut DAT_10160eb8: u32;
    static mut DAT_10160ec0: u32;
    static mut DAT_10160ec4: u32;
    static mut DAT_10160ed0: u32;
    static mut DAT_10160edc: u32;
    static mut DAT_10160ee0: u32;
    static mut DAT_10160ee8: u32;
    static mut DAT_10168398: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100359c0(param_1: u32, param_2: i32, param_3: u8, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003567a(param_1: *mut u32) -> *mut u32 {
    mech_trace::enter!("1003567a", param_1);
    let mut local_c: u8 = 0;
    // SAFETY: param_1 points at the caller's record; the globals and the
    // callee are the game's own, accessed in the original's order.
    unsafe {
        if mech_trace::rd!("1003567a", param_1.byte_add(0x1d), read) == 0 {
            let u_var4 = mech_trace::rd!("1003567a", param_1.add(1), read_unaligned);
            let u_var1 = mech_trace::cr!(
                "1003567a",
                "100359c0",
                FUN_100359c0(
                    mech_trace::rd!("1003567a", param_1, read_unaligned),
                    mech_trace::gr!("1003567a", "10168398", DAT_10168398) as i32,
                    mech_trace::gr!("1003567a", "10160eb4", DAT_10160eb4) as u8,
                    mech_trace::gr!("1003567a", "10160ec4", DAT_10160ec4) as i32,
                )
            ) as u32;
            let i_var3 = mech_trace::gr!("1003567a", "10160ed0", DAT_10160ed0)
                .wrapping_sub(mech_trace::gr!("1003567a", "10160ee0", DAT_10160ee0))
                as i32;
            let i_var2 = mech_trace::cr!(
                "1003567a",
                "100359c0",
                FUN_100359c0(
                    u_var4,
                    mech_trace::gr!("1003567a", "10168398", DAT_10168398) as i32,
                    mech_trace::gr!("1003567a", "10160eb8", DAT_10160eb8) as u8,
                    mech_trace::gr!("1003567a", "10160ee8", DAT_10160ee8) as i32,
                )
            );
            let u_var4 = i_var3.wrapping_sub(i_var2) as u32;
            local_c = ((u_var1.wrapping_sub(mech_trace::gr!("1003567a", "10160ec0", DAT_10160ec0))
                as i32)
                < 0) as u8;
            if 0 < (u_var1.wrapping_sub(mech_trace::gr!("1003567a", "10160edc", DAT_10160edc))
                as i32)
            {
                local_c |= 2;
            }
            if (u_var4.wrapping_sub(mech_trace::gr!("1003567a", "10160ee0", DAT_10160ee0)) as i32)
                < 0
            {
                local_c |= 4;
            }
            if 0 < (u_var4.wrapping_sub(mech_trace::gr!("1003567a", "10160ed0", DAT_10160ed0))
                as i32)
            {
                local_c |= 8;
            }
            mech_trace::wr!("1003567a", param_1.add(3), write_unaligned, u_var1);
            mech_trace::wr!("1003567a", param_1.add(4), write_unaligned, u_var4);
            mech_trace::wr!("1003567a", param_1.add(7).cast::<u8>(), write, local_c);
            mech_trace::wr!("1003567a", param_1.byte_add(0x1d), write, 1);
        }
        mech_trace::gu!("1003567a", "101150f8", DAT_101150f8, |=, local_c);
        mech_trace::gu!("1003567a", "10115168", DAT_10115168, &=, local_c);
        if mech_trace::gr!("1003567a", "10115160", DAT_10115160) < 0x14 {
            let idx = mech_trace::gr!("1003567a", "10115160", DAT_10115160) as usize;
            mech_trace::wr!(
                "1003567a",
                (&raw mut DAT_10115100).cast::<u32>().add(idx),
                write_unaligned,
                param_1 as u32
            );
            mech_trace::gw!(
                "1003567a",
                "10115160",
                DAT_10115160,
                mech_trace::gr!("1003567a", "10115160", DAT_10115160).wrapping_add(1)
            );
        } else {
            mech_trace::gw!("1003567a", "1011515c", DAT_1011515c, 0);
        }
    }
    mech_trace::rt!("1003567a", param_1)
}
