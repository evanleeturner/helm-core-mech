// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: u32;
    static mut DAT_101600e4: u8;
    static mut DAT_101600e8: u8;
    static mut DAT_101600fc: u8;
    static mut DAT_1016010c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> u32;
    fn FUN_10027fed(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d4b9(param_1: i32) -> u32 {
    mech_trace::enter!("1004d4b9", param_1);
    // SAFETY: the globals and callees are the game's own memory; the pointers
    // read from the game's tables are dereferenced where the original does.
    unsafe {
        let i_var2 = param_1.wrapping_mul(0x3c);
        let u_var3 = mech_trace::cr!("1004d4b9", "1004d47f", FUN_1004d47f(param_1));
        let mut local_18 = u_var3 as i16;
        if local_18 == -1 {
            let i_var4 = mech_trace::cr!("1004d4b9", "10027fed", FUN_10027fed(param_1));
            local_18 = i_var4 as i16;
            if local_18 == -1 {
                return mech_trace::rt!("1004d4b9", 0);
            }
        }
        let mut local_14: i16 = 1;
        let mut local_10: i16 = 0;
        loop {
            let count = mech_trace::rd!(
                "1004d4b9",
                (&raw mut DAT_101600e4)
                    .byte_offset(i_var2 as isize)
                    .cast::<i32>(),
                read_unaligned
            );
            if !((local_10 as i32) < count) {
                break;
            }
            let idx =
                mech_trace::rd!("1004d4b9", (&raw mut DAT_101600fc)
                .byte_offset((local_10 as i32).wrapping_mul(4).wrapping_add(i_var2) as isize)
                .cast::<i32>(), read_unaligned);
            let p = mech_trace::rd!(
                "1004d4b9",
                (&raw mut DAT_100eef40).offset(idx as isize),
                read_unaligned
            ) as usize as *mut i32;
            if mech_trace::rd!("1004d4b9", p, read_unaligned) == 1
                && (mech_trace::rd!("1004d4b9", p.add(5).cast::<u16>(), read_unaligned) & 6) == 0
            {
                if mech_trace::rd!("1004d4b9", p.add(1), read_unaligned) == local_18 as i32 {
                    mech_trace::wr!("1004d4b9", p.add(3), write_unaligned, 0);
                } else {
                    mech_trace::wr!("1004d4b9", p.add(3), write_unaligned, local_14 as i32);
                    local_14 = local_14.wrapping_add(1);
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
        let flag = mech_trace::rd!(
            "1004d4b9",
            (&raw mut DAT_101600e8)
                .byte_offset(i_var2 as isize)
                .cast::<i32>(),
            read_unaligned
        );
        if flag != 0 {
            let idx = mech_trace::rd!(
                "1004d4b9",
                (&raw mut DAT_1016010c)
                    .byte_offset(i_var2 as isize)
                    .cast::<i32>(),
                read_unaligned
            );
            let e = mech_trace::rd!(
                "1004d4b9",
                (&raw mut DAT_100eef40).offset(idx as isize),
                read_unaligned
            ) as usize;
            if (mech_trace::rd!(
                "1004d4b9",
                (e.wrapping_add(0x14) as *const u16),
                read_unaligned
            ) & 6)
                == 0
            {
                let idx2 = mech_trace::rd!(
                    "1004d4b9",
                    (&raw mut DAT_1016010c)
                        .byte_offset(i_var2 as isize)
                        .cast::<i32>(),
                    read_unaligned
                );
                let e2 = mech_trace::rd!(
                    "1004d4b9",
                    (&raw mut DAT_100eef40).offset(idx2 as isize),
                    read_unaligned
                ) as usize;
                mech_trace::wr!(
                    "1004d4b9",
                    (e2.wrapping_add(0xc) as *mut u32),
                    write_unaligned,
                    4
                );
            }
        }
        mech_trace::rt!("1004d4b9", 1)
    }
}
