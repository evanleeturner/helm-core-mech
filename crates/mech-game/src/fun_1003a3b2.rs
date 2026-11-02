// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefa4: i32;
    static mut DAT_100aefa0: i32;
    static mut _DAT_100a11dc: f32;
    static mut DAT_10160fe0: i8;
    static mut DAT_10160ff8: i8;
    static mut DAT_1016108c: i8;
    static mut DAT_10161090: i8;
    static mut DAT_10161094: i8;
    static mut DAT_10161098: i8;
    static mut DAT_1016109c: i8;
    static mut DAT_101610a0: i8;
    static mut DAT_101610a4: i8;
    static mut DAT_101610a8: i8;
    static mut DAT_101610ac: i8;
    static mut DAT_101610b0: i8;
    static mut DAT_101610b4: i8;
    static mut DAT_101610d8: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003a0e9(param_1: i32, param_2: i32);
}

#[inline(always)]
unsafe fn at(base: *mut i8, off: i32) -> *mut i32 {
    // SAFETY: the caller passes a game global plus an in-table byte offset.
    unsafe { base.byte_add(off as usize).cast::<i32>() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a3b2(param_1: *mut i32) {
    mech_trace::enter!("1003a3b2", param_1);
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's request block, read at the same offsets as the original.
    unsafe {
        let rd = |n: usize| -> i32 { mech_trace::rd!("1003a3b2", param_1.add(n), read_unaligned) };
        let u_var1 = rd(1) as u32;
        let local_14: i32;
        if rd(0) == 2 {
            mech_trace::gw!("1003a3b2", "100aefa4", DAT_100aefa4, 2);
            local_14 = 0xffffec78u32 as i32;
        } else {
            mech_trace::gw!("1003a3b2", "100aefa4", DAT_100aefa4, 1);
            local_14 = 0xffffd8f0u32 as i32;
        }
        mech_trace::gw!("1003a3b2", "100aefa0", DAT_100aefa0, 1);
        let mut local_1c: i32 = 0;
        while local_1c < 9 {
            let i_var2 = local_1c.wrapping_mul(0x80c);
            if mech_trace::gr!("1003a3b2", "100aefa4", DAT_100aefa4) == 5 {
                mech_trace::wr!(
                    "1003a3b2",
                    at(&raw mut DAT_10160fe0, i_var2),
                    write_unaligned,
                    0
                );
            } else {
                mech_trace::wr!(
                    "1003a3b2",
                    at(&raw mut DAT_10160fe0, i_var2),
                    write_unaligned,
                    mech_trace::gr!("1003a3b2", "100aefa4", DAT_100aefa4)
                );
                if (u_var1 & 1) != 0 {
                    let mut piVar4 = param_1.add(2);
                    let mut piVar5 = at(&raw mut DAT_10160ff8, i_var2);
                    let mut i_var3: i32 = 0x20;
                    while i_var3 != 0 {
                        mech_trace::wr!(
                            "1003a3b2",
                            piVar5,
                            write_unaligned,
                            mech_trace::rd!("1003a3b2", piVar4, read_unaligned)
                        );
                        piVar4 = piVar4.add(1);
                        piVar5 = piVar5.add(1);
                        i_var3 = i_var3.wrapping_sub(1);
                    }
                }
                if (u_var1 & 2) == 0 {
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_1016109c, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610a0, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610a4, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610a8, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610ac, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610b0, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610b4, i_var2),
                        write_unaligned,
                        0
                    );
                } else {
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_1016109c, i_var2),
                        write_unaligned,
                        rd(0x22)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610a0, i_var2),
                        write_unaligned,
                        rd(0x23)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610a4, i_var2),
                        write_unaligned,
                        rd(0x24)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610a8, i_var2),
                        write_unaligned,
                        rd(0x25)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610ac, i_var2),
                        write_unaligned,
                        rd(0x26)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610b0, i_var2),
                        write_unaligned,
                        rd(0x27)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_101610b4, i_var2),
                        write_unaligned,
                        rd(0x28)
                    );
                }
                if (u_var1 & 4) == 0 {
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_10161098, i_var2),
                        write_unaligned,
                        0xe
                    );
                } else {
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_10161098, i_var2),
                        write_unaligned,
                        rd(0x2c)
                    );
                }
                let f = (mech_trace::rd!(
                    "1003a3b2",
                    at(&raw mut DAT_10161098, i_var2),
                    read_unaligned
                ) as f32)
                    / mech_trace::gr!("1003a3b2", "100a11dc", _DAT_100a11dc);
                mech_trace::wr!(
                    "1003a3b2",
                    at(&raw mut DAT_101610d8, i_var2).cast::<f32>(),
                    write_unaligned,
                    f
                );
                if (u_var1 & 8) == 0 {
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_1016108c, i_var2),
                        write_unaligned,
                        0
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_10161090, i_var2),
                        write_unaligned,
                        local_14
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_10161094, i_var2),
                        write_unaligned,
                        0
                    );
                } else {
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_1016108c, i_var2),
                        write_unaligned,
                        rd(0x29)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_10161090, i_var2),
                        write_unaligned,
                        rd(0x2a)
                    );
                    mech_trace::wr!(
                        "1003a3b2",
                        at(&raw mut DAT_10161094, i_var2),
                        write_unaligned,
                        rd(0x2b)
                    );
                }
            }
            local_1c = local_1c.wrapping_add(1);
        }
        if (u_var1 & 0x10000) != 0 {
            FUN_1003a0e9(param_1.add(0x2d) as usize as i32, 0);
        }
        if (u_var1 & 0x20000) != 0 {
            FUN_1003a0e9(param_1.add(0x6a) as usize as i32, 1);
        }
        if (u_var1 & 0x40000) != 0 {
            FUN_1003a0e9(param_1.add(0xa7) as usize as i32, 2);
        }
        if (u_var1 & 0x80000) != 0 {
            FUN_1003a0e9(param_1.add(0xe4) as usize as i32, 3);
        }
        if (u_var1 & 0x100000) != 0 {
            FUN_1003a0e9(param_1.add(0x121) as usize as i32, 4);
        }
        if (u_var1 & 0x200000) != 0 {
            FUN_1003a0e9(param_1.add(0x15e) as usize as i32, 5);
        }
        if (u_var1 & 0x400000) != 0 {
            FUN_1003a0e9(param_1.add(0x19b) as usize as i32, 6);
        }
        if (u_var1 & 0x800000) != 0 {
            FUN_1003a0e9(param_1.add(0x1d8) as usize as i32, 7);
        }
        if (u_var1 & 0x1000000) != 0 {
            FUN_1003a0e9(param_1.add(0x215) as usize as i32, 8);
        }
    }
}
