// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160eb4: u32;
    static mut DAT_10160eb8: u32;
    static mut DAT_10160ec4: u32;
    static mut DAT_10160ee8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005558c(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_10055bc0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u64;
    fn FUN_10055b60(param_1: i32, param_2: i32, param_3: i32, param_4: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100558df(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: u32,
    param_4: *mut u32,
) {
    mech_trace::enter!("100558df", param_1, param_2, param_3, param_4);
    // SAFETY: the callees and globals are the game's own; the three pointers
    // address the caller's arrays, read and written where the original does.
    unsafe {
        let a = |i: usize| mech_trace::rd!("100558df", param_1.add(i), read_unaligned);
        let b = |i: usize| mech_trace::rd!("100558df", param_2.add(i), read_unaligned);
        let rd = |i: usize| mech_trace::rd!("100558df", param_4.add(i), read_unaligned);
        let wr = |i: usize, v: u32| mech_trace::wr!("100558df", param_4.add(i), write_unaligned, v);

        let mut u_var1 = mech_trace::cr!(
            "100558df",
            "1005558c",
            FUN_1005558c(a(1), b(1), param_3, 0, a(2), b(2))
        ) as u32;
        wr(2, u_var1);
        u_var1 = rd(2);
        let u_var3 = mech_trace::cr!(
            "100558df",
            "10055bc0",
            FUN_10055bc0(a(4), b(4), param_3 as i32, a(3), b(3))
        );
        wr(3, u_var3 as u32);
        wr(4, param_3);
        let u_var2 = mech_trace::cr!(
            "100558df",
            "10055b60",
            FUN_10055b60(
                rd(3) as i32,
                mech_trace::gr!("100558df", "10160ec4", DAT_10160ec4) as i32,
                u_var1 as i32,
                mech_trace::gr!("100558df", "10160eb4", DAT_10160eb4) as u8,
            )
        );
        wr(0, u_var2);
        let u_var1 = mech_trace::cr!(
            "100558df",
            "10055b60",
            FUN_10055b60(
                mech_trace::gr!("100558df", "10160ee8", DAT_10160ee8) as i32,
                param_3 as i32,
                u_var1 as i32,
                mech_trace::gr!("100558df", "10160eb8", DAT_10160eb8) as u8,
            )
        );
        wr(1, u_var1);
        let mut local_c = a(0).wrapping_sub(b(0));
        if local_c < 0 {
            local_c = local_c.wrapping_neg();
        }
        let mut local_10 = a(1).wrapping_sub(b(1));
        if local_10 < 0 {
            local_10 = local_10.wrapping_neg();
        }
        if local_c < local_10 {
            let r = mech_trace::cr!(
                "100558df",
                "10055bc0",
                FUN_10055bc0(a(1), b(1), rd(1) as i32, a(5), b(5))
            );
            wr(5, r as u32);
            let r = mech_trace::cr!(
                "100558df",
                "10055bc0",
                FUN_10055bc0(a(1), b(1), rd(1) as i32, a(6), b(6))
            );
            wr(6, r as u32);
            let r = mech_trace::cr!(
                "100558df",
                "10055bc0",
                FUN_10055bc0(a(1), b(1), rd(1) as i32, a(0xb), b(0xb))
            );
            wr(0xb, r as u32);
        } else if local_c == 0 {
            wr(
                5,
                ((b(5) as u32).wrapping_add(a(5) as u32) as i32 >> 1) as u32,
            );
            wr(
                6,
                ((b(6) as u32).wrapping_add(a(6) as u32) as i32 >> 1) as u32,
            );
            wr(
                0xb,
                ((b(0xb) as u32).wrapping_add(a(0xb) as u32) as i32 >> 1) as u32,
            );
        } else {
            let r = mech_trace::cr!(
                "100558df",
                "10055bc0",
                FUN_10055bc0(a(0), b(0), rd(0) as i32, a(5), b(5))
            );
            wr(5, r as u32);
            let r = mech_trace::cr!(
                "100558df",
                "10055bc0",
                FUN_10055bc0(a(0), b(0), rd(0) as i32, a(6), b(6))
            );
            wr(6, r as u32);
            let r = mech_trace::cr!(
                "100558df",
                "10055bc0",
                FUN_10055bc0(a(0), b(0), rd(0) as i32, a(0xb), b(0xb))
            );
            wr(0xb, r as u32);
        }
    }
}
