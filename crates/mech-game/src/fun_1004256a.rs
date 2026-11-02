// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(a: i32, b: i16);
    fn FUN_1002a4e0(a: i32, b: i16, c: u16, d: i32);
    fn FUN_1004521e(a: *mut i32) -> i32;
    fn FUN_10044bba(a: i32, b: i32) -> i32;
    fn FUN_10043e6b(a: i32) -> i32;
    fn FUN_100447ff(a: i32, b: i32) -> u8;
    fn FUN_10051f43(a: i32) -> i32;
    fn FUN_100428b0(a: *mut i16, b: i16) -> u32;
    fn FUN_1002a132(a: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004256a(param_1: *mut i32) -> i32 {
    mech_trace::enter!("1004256a", param_1);
    // SAFETY: param_1 points at the game's object; all offsets read and written
    // are those the original uses, and the callees are the game's own.
    unsafe {
        let mut local_c: i16 = -1;
        let ps_var1: *mut i16 = mech_trace::rd!(
            "1004256a",
            param_1.byte_add(0x202).cast::<*mut i16>(),
            read_unaligned
        );
        let pi_var2: *mut i32 =
            mech_trace::rd!("1004256a", param_1.add(8), read_unaligned) as usize as *mut i32;
        let p = param_1 as i32;
        loop {
            FUN_1002950f(
                p,
                mech_trace::rd!("1004256a", param_1.add(0x5e), read_unaligned) as i16,
            );
            if mech_trace::rd!("1004256a", param_1.add(0x72), read_unaligned) != 0 {
                if mech_trace::rd!("1004256a", param_1.add(0x72), read_unaligned) == -2 {
                    FUN_1002a4e0(
                        p,
                        4,
                        mech_trace::rd!(
                            "1004256a",
                            param_1.byte_add(0x17a).cast::<u16>(),
                            read_unaligned
                        ),
                        0,
                    );
                    mech_trace::wr!(
                        "1004256a",
                        param_1.byte_add(0x17e).cast::<i32>(),
                        write_unaligned,
                        1
                    );
                } else {
                    local_c = mech_trace::rd!("1004256a", param_1.add(0x72), read_unaligned) as i16;
                }
                mech_trace::wr!("1004256a", param_1.add(0x72), write_unaligned, 0);
                return mech_trace::rt!("1004256a", local_c as i32);
            }
            if mech_trace::rd!("1004256a", param_1, read_unaligned) == 1 {
                let i_var4 = mech_trace::cr!("1004256a", "1004521e", FUN_1004521e(pi_var2));
                if i_var4 != 0 {
                    return mech_trace::rt!("1004256a", 10);
                }
                if (mech_trace::rd!(
                    "1004256a",
                    param_1.byte_add(0x17a).cast::<u16>(),
                    read_unaligned
                ) & 0x200)
                    == 0
                {
                    return mech_trace::rt!("1004256a", 0);
                }
                let i_var4 = mech_trace::cr!("1004256a", "10044bba", FUN_10044bba(p, 0x14));
                if i_var4 != 0 {
                    let tmp_u = mech_trace::rd!(
                        "1004256a",
                        param_1.byte_add(0x1f2).cast::<u32>(),
                        read_unaligned
                    );
                    if ((tmp_u >> 1) & 1) != 0 {
                        let i_var4 = mech_trace::cr!("1004256a", "10043e6b", FUN_10043e6b(p));
                        if i_var4 != 0 {
                            if mech_trace::cr!(
                                "1004256a",
                                "100447ff",
                                FUN_100447ff(
                                    p,
                                    mech_trace::rd!("1004256a", param_1.add(0x33), read_unaligned)
                                )
                            ) != 0
                            {
                                if mech_trace::rd!(
                                    "1004256a",
                                    param_1.byte_add(0x1c6).cast::<i16>(),
                                    read_unaligned
                                ) == 0
                                {
                                    local_c = 9;
                                } else {
                                    local_c = 0;
                                }
                                if local_c == 0
                                    && mech_trace::cr!("1004256a", "10051f43", FUN_10051f43(2)) != 0
                                {
                                    local_c = 8;
                                }
                                return mech_trace::rt!("1004256a", local_c as i32);
                            }
                        }
                    }
                }
            }
            if mech_trace::rd!(
                "1004256a",
                param_1.byte_add(0x1c6).cast::<i16>(),
                read_unaligned
            ) == -1
            {
                let a = mech_trace::rd!("1004256a", ps_var1, read_unaligned) as i32;
                let b = mech_trace::rd!("1004256a", ps_var1.add(1), read_unaligned) as i32;
                let i_var4 =
                    mech_trace::cr!("1004256a", "10051f43", FUN_10051f43(a.wrapping_sub(b)));
                let base =
                    mech_trace::rd!("1004256a", ps_var1.add(2).cast::<u32>(), read_unaligned);
                let addr = base.wrapping_add(((i_var4 as i16 as i32).wrapping_mul(0x12)) as u32);
                local_c =
                    mech_trace::rd!("1004256a", (addr as usize as *const i16), read_unaligned);
            } else {
                let u_var5 = mech_trace::cr!(
                    "1004256a",
                    "100428b0",
                    FUN_100428b0(
                        ps_var1,
                        mech_trace::rd!(
                            "1004256a",
                            param_1.byte_add(0x1c6).cast::<i16>(),
                            read_unaligned
                        ),
                    )
                );
                let mut local_8 = u_var5 as i16;
                if local_8 == -1 {
                    local_8 = 0;
                }
                let base =
                    mech_trace::rd!("1004256a", ps_var1.add(2).cast::<u32>(), read_unaligned);
                let off = ((local_8 as i32).wrapping_mul(0x12)) as u32;
                let a1 = base.wrapping_add(2).wrapping_add(off);
                let v = mech_trace::rd!("1004256a", (a1 as usize as *const i16), read_unaligned);
                let i_var4 = mech_trace::cr!("1004256a", "10051f43", FUN_10051f43(v as i32));
                let a2 = (i_var4 as u32)
                    .wrapping_mul(2)
                    .wrapping_add(4)
                    .wrapping_add(base)
                    .wrapping_add(off);
                local_c = mech_trace::rd!("1004256a", (a2 as usize as *const i16), read_unaligned);
            }
            if mech_trace::rd!(
                "1004256a",
                param_1.byte_add(0x1c6).cast::<i16>(),
                read_unaligned
            ) == 4
                && mech_trace::rd!("1004256a", param_1.add(0x76), read_unaligned) == 1
            {
                return mech_trace::rt!("1004256a", 5);
            }
            if local_c == 4 {
                let i_var4 = mech_trace::cr!("1004256a", "10044bba", FUN_10044bba(p, 0x14));
                let tmp_u = mech_trace::rd!(
                    "1004256a",
                    param_1.byte_add(0x1f2).cast::<u32>(),
                    read_unaligned
                );
                if i_var4 == 0 || ((tmp_u >> 1) & 1) == 0 {
                    local_c = -1;
                }
            }
            if local_c == 4 {
                let i_var4 = mech_trace::cr!("1004256a", "1002a132", FUN_1002a132(p));
                if mech_trace::rd!("1004256a", pi_var2.add(0x105), read_unaligned) < i_var4 {
                    local_c = 0;
                }
            }
            if local_c == 3
                && mech_trace::rd!("1004256a", pi_var2.add(0x105), read_unaligned) < 0xa0000
            {
                local_c = 2;
            }
            if local_c == 5 && mech_trace::rd!("1004256a", param_1.add(0x1e), read_unaligned) != 0 {
                local_c = 0;
            }
            if local_c != -1 {
                return mech_trace::rt!("1004256a", local_c as i32);
            }
        }
    }
}
