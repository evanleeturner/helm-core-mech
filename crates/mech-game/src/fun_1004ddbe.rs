// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100ef030: i8;
    static mut DAT_100ef034: i8;
    static mut DAT_101600e0: i8;
    static mut DAT_10215670: i8;
    static mut DAT_10215680: i8;
    static mut DAT_10215688: i8;
    static mut DAT_1021568c: i8;
    static mut DAT_10215690: i8;
    static mut DAT_10215694: i8;
    static mut DAT_10215696: i8;
    static mut DAT_100eef40: [*mut u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003c190(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_1000dde1(a: i32, b: *mut i32, c: *mut i32, d: *mut i32);
    fn FUN_10003742(a: i32, b: i32, c: i32, d: i32, e: i32);
}

unsafe fn rd_i32(p: *const u8, off: usize) -> i32 {
    // SAFETY: caller supplies a valid game pointer and offset.
    unsafe { p.byte_add(off).cast::<i32>().read_unaligned() }
}

unsafe fn rd_u16(p: *const u8, off: usize) -> u16 {
    // SAFETY: caller supplies a valid game pointer and offset.
    unsafe { p.byte_add(off).cast::<u16>().read_unaligned() }
}

unsafe fn ent(i: i32) -> *mut u8 {
    // SAFETY: the game's entity pointer table holds valid entries.
    unsafe {
        (&raw mut DAT_100eef40)
            .cast::<*mut u8>()
            .add(i as usize)
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004ddbe(param_1: *mut u8, param_2: i32) -> u32 {
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's two bytes; table entries and offsets are read as the original does.
    unsafe {
        let u_var1: u16 = param_1.read() as u16;
        let u_var2: u32 = ((param_1.add(1).read() as u32) << 8) & 0xffff;
        let flag_base = (&raw mut DAT_101600e0).cast::<u8>();
        let flag_off = (param_2 as u32).wrapping_mul(0x3c) as usize;
        if !(rd_i32(flag_base, flag_off) < 0) || DAT_100aa1ac != 0 {
            if u_var2 == 0x100 {
                let i_var3 = (u_var1 as u32).wrapping_mul(0x54) as usize;
                let mut local_28 = rd_i32((&raw mut DAT_10215680).cast::<u8>(), i_var3);
                if local_28 < 1 {
                    local_28 = 20000;
                }
                let mut local_10: i32 = 0;
                while local_10 < DAT_100b38d8 {
                    let e = ent(local_10);
                    if rd_i32(e, 8) == param_2 {
                        let r = FUN_1003c190(
                            rd_i32(e, 0x50)
                                .wrapping_sub(rd_i32((&raw mut DAT_10215688).cast::<u8>(), i_var3)),
                            rd_i32(e, 0x54)
                                .wrapping_sub(rd_i32((&raw mut DAT_1021568c).cast::<u8>(), i_var3)),
                            rd_i32(e, 0x58)
                                .wrapping_sub(rd_i32((&raw mut DAT_10215690).cast::<u8>(), i_var3)),
                            local_28,
                        );
                        if r != 0 {
                            let e = ent(local_10);
                            if DAT_100adf38 == local_10
                                && (rd_u16(e, 0x14) & 0x2000) != 0
                                && (rd_u16((&raw mut DAT_10215694).cast::<u8>(), i_var3) & 0x20)
                                    == 0
                                && rd_i32((&raw mut DAT_10215670).cast::<u8>(), i_var3) != 0
                            {
                                let p94 = (&raw mut DAT_10215694)
                                    .cast::<u8>()
                                    .add(i_var3)
                                    .cast::<u16>();
                                p94.write_unaligned(p94.read_unaligned() | 0x20);
                                let p96 = (&raw mut DAT_10215696)
                                    .cast::<u8>()
                                    .add(i_var3)
                                    .cast::<u16>();
                                let bit = 1u32.wrapping_shl((param_2 as u32) & 0x1f) as u16;
                                p96.write_unaligned(p96.read_unaligned() | bit);
                                FUN_10003742(0xfc, 100, 0x40, 5, 0x50);
                            }
                            return 1;
                        }
                    }
                    local_10 = local_10.wrapping_add(1);
                }
            } else if u_var2 == 0x200 {
                let local_14 = ent(u_var1 as i32);
                let mut local_10: i32 = 0;
                while local_10 < DAT_100b38d8 {
                    let e = ent(local_10);
                    if rd_i32(e, 8) == param_2 && (rd_u16(e, 0x14) & 6) == 0 {
                        let r = FUN_1003c190(
                            rd_i32(e, 0x50).wrapping_sub(rd_i32(local_14, 0x50)),
                            rd_i32(e, 0x54).wrapping_sub(rd_i32(local_14, 0x54)),
                            rd_i32(e, 0x58).wrapping_sub(rd_i32(local_14, 0x58)),
                            20000,
                        );
                        if r != 0 {
                            return 1;
                        }
                    }
                    local_10 = local_10.wrapping_add(1);
                }
            } else if u_var2 == 0x400 {
                let off = (u_var1 as u32).wrapping_mul(0x40) as usize;
                let mut local_1c: i32 = 0;
                let mut local_20: i32 = 0;
                let mut local_24: i32 = 0;
                FUN_1000dde1(
                    rd_i32((&raw mut DAT_100ef034).cast::<u8>(), off),
                    &raw mut local_1c,
                    &raw mut local_20,
                    &raw mut local_24,
                );
                let _ = (&raw mut DAT_100ef030).cast::<u8>().add(off);
                let mut local_10: i32 = 0;
                while local_10 < DAT_100b38d8 {
                    let e = ent(local_10);
                    if rd_i32(e, 8) == param_2 {
                        let r = FUN_1003c190(
                            rd_i32(e, 0x50).wrapping_sub(local_1c),
                            rd_i32(e, 0x54).wrapping_sub(local_20),
                            rd_i32(e, 0x58).wrapping_sub(local_24),
                            20000,
                        );
                        if r != 0 {
                            return 1;
                        }
                    }
                    local_10 = local_10.wrapping_add(1);
                }
            }
        }
        0
    }
}
