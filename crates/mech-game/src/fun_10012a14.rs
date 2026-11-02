// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1d8: u32;
    static mut DAT_100aa1ec: *mut u16;
    static mut DAT_100aa258: u16;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [u32; 8];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: u32, param_2: u32) -> u8;
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_10006f9a(param_1: u32, param_2: u32, param_3: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10012a14(param_1: i32, param_2: *mut i8) -> u32 {
    mech_trace::enter!("10012a14", param_1, param_2);
    // SAFETY: globals and callees are the game's own; param_2 is a
    // NUL-terminated string and the destination buffer is the game's.
    unsafe {
        let mut u_var5: u32 = 0xffff_ffff;
        let mut pc_var7: *mut i8 = param_2;
        loop {
            if u_var5 == 0 {
                break;
            }
            u_var5 = u_var5.wrapping_sub(1);
            let c = mech_trace::rd!("10012a14", pc_var7, read);
            pc_var7 = pc_var7.add(1);
            if c == 0 {
                break;
            }
        }
        mech_trace::rt!(
            "10012a14",
            if (!u_var5).wrapping_sub(1) < 0x50 {
                let i_var1: i32 = (!u_var5).wrapping_add(2) as i32;
                mech_trace::wr!(
                    "10012a14",
                    mech_trace::gr!("10012a14", "100aa1ec", DAT_100aa1ec),
                    write_unaligned,
                    mech_trace::gr!("10012a14", "100aa258", DAT_100aa258)
                );
                u_var5 = 0xffff_ffff;
                let mut p2 = param_2;
                loop {
                    pc_var7 = p2;
                    if u_var5 == 0 {
                        break;
                    }
                    u_var5 = u_var5.wrapping_sub(1);
                    pc_var7 = p2.add(1);
                    let c = mech_trace::rd!("10012a14", p2, read);
                    p2 = pc_var7;
                    if c == 0 {
                        break;
                    }
                }
                u_var5 = !u_var5;
                pc_var7 = pc_var7.wrapping_sub(u_var5 as usize);
                let mut pc_var8: *mut i8 =
                    mech_trace::gr!("10012a14", "100aa1ec", DAT_100aa1ec).add(1) as *mut i8;
                let mut u_var6 = u_var5 >> 2;
                while u_var6 != 0 {
                    mech_trace::wr!(
                        "10012a14",
                        pc_var8.cast::<u32>(),
                        write_unaligned,
                        mech_trace::rd!("10012a14", pc_var7.cast::<u32>(), read_unaligned)
                    );
                    pc_var7 = pc_var7.add(4);
                    pc_var8 = pc_var8.add(4);
                    u_var6 = u_var6.wrapping_sub(1);
                }
                u_var5 &= 3;
                while u_var5 != 0 {
                    mech_trace::wr!(
                        "10012a14",
                        pc_var8,
                        write,
                        mech_trace::rd!("10012a14", pc_var7, read)
                    );
                    pc_var7 = pc_var7.add(1);
                    pc_var8 = pc_var8.add(1);
                    u_var5 = u_var5.wrapping_sub(1);
                }
                if param_1 == -3 {
                    let mut local_8: i32 = 0;
                    while local_8 < 8 {
                        if mech_trace::gr!("10012a14", "100eef40", DAT_100eef40[local_8 as usize])
                            != 0
                            && mech_trace::cr!("10012a14", "1004d5e3", FUN_1004d5e3(local_8)) == 1
                        {
                            let base = mech_trace::rd!(
                                "10012a14",
                                ((mech_trace::gr!("10012a14", "100aa1d8", DAT_100aa1d8)
                                    .wrapping_add(0x10))
                                    as *const i32),
                                read_unaligned
                            );
                            let v = mech_trace::rd!(
                                "10012a14",
                                (base.wrapping_add(local_8.wrapping_mul(4)) as *const u32),
                                read_unaligned
                            );
                            mech_trace::cr!(
                                "10012a14",
                                "10006f9a",
                                FUN_10006f9a(
                                    v,
                                    mech_trace::gr!("10012a14", "100aa1ec", DAT_100aa1ec) as u32,
                                    i_var1 as u32
                                )
                            );
                        }
                        local_8 += 1;
                    }
                } else if param_1 == -2 {
                    let mut local_8: i32 = 0;
                    while local_8 < 8 {
                        if mech_trace::gr!("10012a14", "100eef40", DAT_100eef40[local_8 as usize])
                            != 0
                            && mech_trace::cr!("10012a14", "1004d5e3", FUN_1004d5e3(local_8)) == 0
                            && local_8 != mech_trace::gr!("10012a14", "100adf38", DAT_100adf38)
                        {
                            let base = mech_trace::rd!(
                                "10012a14",
                                ((mech_trace::gr!("10012a14", "100aa1d8", DAT_100aa1d8)
                                    .wrapping_add(0x10))
                                    as *const i32),
                                read_unaligned
                            );
                            let v = mech_trace::rd!(
                                "10012a14",
                                (base.wrapping_add(local_8.wrapping_mul(4)) as *const u32),
                                read_unaligned
                            );
                            mech_trace::cr!(
                                "10012a14",
                                "10006f9a",
                                FUN_10006f9a(
                                    v,
                                    mech_trace::gr!("10012a14", "100aa1ec", DAT_100aa1ec) as u32,
                                    i_var1 as u32
                                )
                            );
                        }
                        local_8 += 1;
                    }
                } else if param_1 == -1 {
                    mech_trace::cr!(
                        "10012a14",
                        "10006e00",
                        FUN_10006e00(
                            mech_trace::gr!("10012a14", "100aa1ec", DAT_100aa1ec) as u32,
                            i_var1 as u32
                        )
                    );
                } else {
                    let base = mech_trace::rd!(
                        "10012a14",
                        ((mech_trace::gr!("10012a14", "100aa1d8", DAT_100aa1d8).wrapping_add(0x10))
                            as *const i32),
                        read_unaligned
                    );
                    let v = mech_trace::rd!(
                        "10012a14",
                        (base.wrapping_add(param_1.wrapping_mul(4)) as *const u32),
                        read_unaligned
                    );
                    mech_trace::cr!(
                        "10012a14",
                        "10006f9a",
                        FUN_10006f9a(
                            v,
                            mech_trace::gr!("10012a14", "100aa1ec", DAT_100aa1ec) as u32,
                            i_var1 as u32
                        )
                    );
                }
                1
            } else {
                0
            }
        )
    }
}
