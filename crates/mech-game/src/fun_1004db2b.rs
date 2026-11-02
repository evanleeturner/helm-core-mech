// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: i32;
    static mut DAT_101600e0: u8;
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_10215670: u8;
    static mut DAT_10215680: u8;
    static mut DAT_10215688: u8;
    static mut DAT_1021568c: u8;
    static mut DAT_10215690: u8;
    static mut DAT_10215694: u8;
    static mut DAT_10215696: u8;
    static mut DAT_100ef030: u8;
    static mut DAT_100ef034: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003c190(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_1000dde1(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_10003742(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[inline(always)]
unsafe fn rd_i32(addr: usize) -> i32 {
    // SAFETY: caller supplies an address in the game's own memory.
    unsafe { mech_trace::rd!("1004db2b", (addr as *const i32), read_unaligned) }
}

#[inline(always)]
unsafe fn rd_u16(addr: usize) -> u16 {
    // SAFETY: caller supplies an address in the game's own memory.
    unsafe { mech_trace::rd!("1004db2b", (addr as *const u16), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004db2b(param_1: *mut u8, param_2: i32) -> u32 {
    mech_trace::enter!("1004db2b", param_1, param_2);
    // SAFETY: all globals and addresses are the game's own memory; param_1
    // points at the caller's bytes; reads and writes follow the original.
    unsafe {
        let u_var1: u16 = mech_trace::rd!("1004db2b", param_1, read) as u16;
        let local_10: u32 =
            (mech_trace::rd!("1004db2b", param_1.byte_add(1), read) as u32).wrapping_shl(8);
        let tbl = (&raw mut DAT_100eef40) as usize;
        let ent = (&raw mut DAT_101600e0) as usize + (param_2 as u32).wrapping_mul(0x3c) as usize;
        let ent_val = rd_i32(ent);
        let local_8: usize =
            rd_i32(tbl.wrapping_add((ent_val as u32).wrapping_mul(4) as usize)) as u32 as usize;

        if ent_val < 0 && mech_trace::gr!("1004db2b", "100aa1ac", DAT_100aa1ac) == 0 {
            return mech_trace::rt!("1004db2b", 0);
        }
        let u_var3 = local_10 & 0xffff;
        mech_trace::rt!(
            "1004db2b",
            if u_var3 == 0x100 {
                let i_var4 = (u_var1 as u32).wrapping_mul(0x54) as usize;
                let b670 = (&raw mut DAT_10215670) as usize;
                let b680 = (&raw mut DAT_10215680) as usize;
                let b688 = (&raw mut DAT_10215688) as usize;
                let b68c = (&raw mut DAT_1021568c) as usize;
                let b690 = (&raw mut DAT_10215690) as usize;
                let b694 = (&raw mut DAT_10215694) as usize;
                let b696 = (&raw mut DAT_10215696) as usize;
                let mut local_28 = rd_i32(b680.wrapping_add(i_var4));
                if local_28 < 1 {
                    local_28 = 20000;
                }
                let i_var5 = mech_trace::cr!(
                    "1004db2b",
                    "1003c190",
                    FUN_1003c190(
                        rd_i32(local_8.wrapping_add(0x50))
                            .wrapping_sub(rd_i32(b688.wrapping_add(i_var4))),
                        rd_i32(local_8.wrapping_add(0x54))
                            .wrapping_sub(rd_i32(b68c.wrapping_add(i_var4))),
                        rd_i32(local_8.wrapping_add(0x58))
                            .wrapping_sub(rd_i32(b690.wrapping_add(i_var4))),
                        local_28,
                    )
                );
                if i_var5 == 0 {
                    return mech_trace::rt!("1004db2b", 0);
                }
                if rd_i32(ent) == mech_trace::gr!("1004db2b", "100adf38", DAT_100adf38)
                    && (rd_u16(local_8.wrapping_add(0x14)) & 0x2000) != 0
                    && (rd_u16(b694.wrapping_add(i_var4)) & 0x20) == 0
                    && rd_i32(b670.wrapping_add(i_var4)) != 0
                {
                    let p694 = b694.wrapping_add(i_var4) as *mut u16;
                    mech_trace::wr!(
                        "1004db2b",
                        p694,
                        write_unaligned,
                        mech_trace::rd!("1004db2b", p694, read_unaligned) | 0x20
                    );
                    let p696 = b696.wrapping_add(i_var4) as *mut u16;
                    let sh =
                        (mech_trace::rd!("1004db2b", (local_8.wrapping_add(8) as *const u8), read)
                            & 0x1f) as u32;
                    let v = (1u32.wrapping_shl(sh) as u16)
                        | mech_trace::rd!("1004db2b", p696, read_unaligned);
                    mech_trace::wr!("1004db2b", p696, write_unaligned, v);
                    FUN_10003742(0xfc, 100, 0x40, 5, 0x50);
                }
                1
            } else if u_var3 == 0x200 {
                let local_14: usize =
                    rd_i32(tbl.wrapping_add((u_var1 as u32).wrapping_mul(4) as usize)) as u32
                        as usize;
                if (rd_u16(local_14.wrapping_add(0x14)) & 6) == 0 {
                    let i_var5 = mech_trace::cr!(
                        "1004db2b",
                        "1003c190",
                        FUN_1003c190(
                            rd_i32(local_8.wrapping_add(0x50))
                                .wrapping_sub(rd_i32(local_14.wrapping_add(0x50))),
                            rd_i32(local_8.wrapping_add(0x54))
                                .wrapping_sub(rd_i32(local_14.wrapping_add(0x54))),
                            rd_i32(local_8.wrapping_add(0x58))
                                .wrapping_sub(rd_i32(local_14.wrapping_add(0x58))),
                            20000,
                        )
                    );
                    if i_var5 != 0 {
                        return mech_trace::rt!("1004db2b", 1);
                    }
                }
                0
            } else if u_var3 == 0x400 {
                let mut local_1c: i32 = 0;
                let mut local_20: i32 = 0;
                let mut local_24: i32 = 0;
                let off = (u_var1 as u32).wrapping_mul(0x40) as usize;
                let a = rd_i32(((&raw mut DAT_100ef034) as usize).wrapping_add(off));
                FUN_1000dde1(a, &raw mut local_1c, &raw mut local_20, &raw mut local_24);
                mech_trace::cr!(
                    "1004db2b",
                    "1003c190",
                    FUN_1003c190(
                        rd_i32(local_8.wrapping_add(0x50)).wrapping_sub(local_1c),
                        rd_i32(local_8.wrapping_add(0x54)).wrapping_sub(local_20),
                        rd_i32(local_8.wrapping_add(0x58)).wrapping_sub(local_24),
                        20000,
                    )
                ) as u32
            } else {
                0
            }
        )
    }
}
