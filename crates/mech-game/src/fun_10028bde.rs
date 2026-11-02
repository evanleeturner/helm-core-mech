// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123240: u8;
    static mut DAT_10123130: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002b6fe(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10029e46(param_1: i32, param_2: u16, param_3: u16) -> u32;
    fn FUN_1002950f(param_1: i32, param_2: i16) -> i32;
    fn FUN_1002883b(param_1: i32, param_2: u16, param_3: i16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028bde(
    param_1: i32,
    param_2: u16,
    param_3: i16,
    param_4: *mut u16,
    param_5: *mut u16,
) -> i32 {
    mech_trace::enter!("10028bde", param_1, param_2, param_3, param_4, param_5);
    let mut local_1c: i32 = 0;
    let mut local_20: i32 = 100000000;
    let mut local_8: u32 = 0xFFFF;
    let mut local_44: [i32; 8] = [0; 8];
    // SAFETY: param_1 is a game object pointer, param_4/param_5 are caller
    // out-pointers, and the globals and callees are the game's own memory.
    unsafe {
        mech_trace::wr!("10028bde", param_5, write_unaligned, 0);
        mech_trace::wr!(
            "10028bde",
            param_4,
            write_unaligned,
            mech_trace::rd!("10028bde", param_5, read_unaligned)
        );
        let p1 = param_1 as usize as *const u8;
        let i_var4 = mech_trace::cr!(
            "10028bde",
            "1002b6fe",
            FUN_1002b6fe(
                mech_trace::rd!("10028bde", p1.byte_add(8).cast::<i32>(), read_unaligned),
                local_44.as_mut_ptr() as i32,
                1,
            )
        );
        if i_var4 == 0 {
            local_1c = 0;
        } else {
            let idx = mech_trace::rd!("10028bde", p1.byte_add(8).cast::<i32>(), read_unaligned);
            let local_c = (&raw mut DAT_10123240)
                .cast::<u8>()
                .byte_add(idx.wrapping_mul(0x3C0A) as isize as usize);
            let idx2 = mech_trace::rd!("10028bde", p1.byte_add(8).cast::<i32>(), read_unaligned);
            let local_24 = mech_trace::rd!(
                "10028bde",
                (&raw mut DAT_10123130)
                    .cast::<u8>()
                    .byte_add(idx2.wrapping_mul(4) as isize as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            if local_24 == -1 {
                local_1c = 0;
            } else {
                let local_10: u32 = u32::from(mech_trace::rd!(
                    "10028bde",
                    local_c
                        .byte_add(local_24.wrapping_mul(0x13F).wrapping_add(0x3C) as isize as usize)
                        .cast::<u16>(),
                    read_unaligned
                ));
                loop {
                    let u_var5 = mech_trace::cr!(
                        "10028bde",
                        "10029e46",
                        FUN_10029e46(param_1, param_2, local_8 as u16)
                    );
                    let s_var3 = u_var5 as i16;
                    local_8 = u32::from(s_var3 as u16);
                    if s_var3 == -1 {
                        break;
                    }
                    let mut b_var1: i32 = 0;
                    let b_var2 =
                        mech_trace::cr!("10028bde", "1002950f", FUN_1002950f(param_1, s_var3));
                    if b_var2 != 0 {
                        let c0 = mech_trace::rd!(
                            "10028bde",
                            p1.byte_add(0xC0).cast::<i32>(),
                            read_unaligned
                        );
                        if c0 < local_20 {
                            local_20 = mech_trace::rd!(
                                "10028bde",
                                p1.byte_add(0xC0).cast::<i32>(),
                                read_unaligned
                            );
                            mech_trace::wr!("10028bde", param_5, write_unaligned, local_8 as u16);
                            b_var1 = 1;
                            if (local_8 & 0x200) == 0 {
                                mech_trace::wr!(
                                    "10028bde",
                                    param_4,
                                    write_unaligned,
                                    local_8 as u16
                                );
                            }
                        }
                        if (local_8 & 0x200) != 0 {
                            let mut local_18: i16 = 0;
                            while (local_18 as i32) < i_var4 {
                                let local_14 = mech_trace::rd!(
                                    "10028bde",
                                    local_44.as_ptr().offset(local_18 as isize),
                                    read
                                );
                                let p14 = local_14 as usize as *const u8;
                                if mech_trace::rd!(
                                    "10028bde",
                                    p14.byte_add(4).cast::<i32>(),
                                    read_unaligned
                                ) != mech_trace::rd!(
                                    "10028bde",
                                    p1.byte_add(4).cast::<i32>(),
                                    read_unaligned
                                ) && (mech_trace::rd!(
                                    "10028bde",
                                    p14.byte_add(0x14).cast::<u16>(),
                                    read_unaligned
                                ) & 0x16)
                                    == 0
                                    && (mech_trace::rd!(
                                        "10028bde",
                                        p14.byte_add(0x17A).cast::<u16>(),
                                        read_unaligned
                                    ) & 0xF000)
                                        == 0
                                    && u32::from(mech_trace::rd!(
                                        "10028bde",
                                        p14.byte_add(0x17A).cast::<u16>(),
                                        read_unaligned
                                    )) == ((local_8 as u16 as i16) as i32 as u32)
                                    && (mech_trace::rd!(
                                        "10028bde",
                                        p14.byte_add(0x176).cast::<i16>(),
                                        read_unaligned
                                    ) == 3
                                        || mech_trace::rd!(
                                            "10028bde",
                                            p14.byte_add(0x176).cast::<i16>(),
                                            read_unaligned
                                        ) == 2)
                                    && b_var1 != 0
                                    && (local_10 & 0xFFFF) != 4
                                {
                                    b_var1 = 0;
                                    local_1c = local_14;
                                    break;
                                }
                                local_18 = local_18.wrapping_add(1);
                            }
                            if b_var1 != 0 {
                                mech_trace::wr!(
                                    "10028bde",
                                    param_4,
                                    write_unaligned,
                                    local_8 as u16
                                );
                            }
                        }
                    }
                }
                if mech_trace::rd!("10028bde", param_4, read_unaligned) != 0
                    && (mech_trace::cr!(
                        "10028bde",
                        "1002883b",
                        FUN_1002883b(
                            param_1,
                            mech_trace::rd!("10028bde", param_4, read_unaligned),
                            param_3
                        )
                    ) as i16)
                        == 0
                {
                    mech_trace::wr!("10028bde", param_4, write_unaligned, 0);
                }
                if mech_trace::rd!("10028bde", param_5, read_unaligned) != 0
                    && (mech_trace::cr!(
                        "10028bde",
                        "1002883b",
                        FUN_1002883b(
                            param_1,
                            mech_trace::rd!("10028bde", param_5, read_unaligned),
                            param_3
                        )
                    ) as i16)
                        == 0
                {
                    mech_trace::wr!("10028bde", param_5, write_unaligned, 0);
                }
            }
        }
    }
    mech_trace::rt!("10028bde", local_1c)
}
