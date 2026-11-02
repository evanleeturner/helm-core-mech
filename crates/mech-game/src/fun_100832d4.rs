// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b9f64: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef030: [u8; 0];
    static mut DAT_100ef032: [u8; 0];
    static mut DAT_100ef040: [u8; 0];
    static mut DAT_10215680: [u8; 0];
    static mut DAT_10215694: [u8; 0];
    static mut DAT_10215696: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10082d2f(param_1: i32, param_2: i32, param_3: u32) -> u32;
    fn FUN_1008304a(param_1: i32, param_2: i32, param_3: u32) -> u32;
    fn FUN_10082adc(param_1: u32, param_2: i32, param_3: u32) -> u32;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_10083b74(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

unsafe fn rd_u32(addr: u32) -> u32 {
    // SAFETY: the caller supplies an address in the game's memory.
    unsafe { (addr as usize as *const u32).read_unaligned() }
}

unsafe fn rd_i32(addr: u32) -> i32 {
    // SAFETY: the caller supplies an address in the game's memory.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

unsafe fn rd_u16(addr: u32) -> u16 {
    // SAFETY: the caller supplies an address in the game's memory.
    unsafe { (addr as usize as *const u16).read_unaligned() }
}

unsafe fn wr_u16(addr: u32, v: u16) {
    // SAFETY: the caller supplies an address in the game's memory.
    unsafe { (addr as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100832d4(param_1: i32) -> u32 {
    // SAFETY: param_1 points at the game's object; all globals and callees
    // are the game's own, and every access is made where the original makes it.
    unsafe {
        let p = param_1 as u32;
        let mut b_var6 = true;
        let b_var5 = rd_u32(p.wrapping_add(4)) == DAT_100adf38;
        let u_var1 = rd_u32(p.wrapping_add(0xdc)) & 0xff;
        let u_var2 = rd_u32(p.wrapping_add(0xdc)) & 0xf00;
        let bit: u16 = 1u32.wrapping_shl((rd_u32(p.wrapping_add(8)) & 0x1f) as u32) as u16;
        let bit_i: i32 = 1u32.wrapping_shl((rd_u32(p.wrapping_add(8)) & 0x1f) as u32) as i32;

        let a680 = (&raw mut DAT_10215680) as usize as u32;
        let a694 = (&raw mut DAT_10215694) as usize as u32;
        let a696 = (&raw mut DAT_10215696) as usize as u32;
        let eef = (&raw mut DAT_100eef40) as *mut u32;
        let a030 = (&raw mut DAT_100ef030) as usize as u32;
        let a032 = (&raw mut DAT_100ef032) as usize as u32;
        let a040 = (&raw mut DAT_100ef040) as usize as u32;

        if u_var2 == 0x100 {
            let i_var3 = FUN_10082adc(rd_u32(p.wrapping_add(4)), u_var1 as i32, 0) as i32;
            if -1 < i_var3 {
                b_var6 = false;
                if rd_i32(p.wrapping_add(0xc0))
                    < rd_i32(a680.wrapping_add(u_var1.wrapping_mul(0x54)))
                    && rd_i32((rd_i32(p.wrapping_add(0x20)) as u32).wrapping_add(0x3f0)) != 1
                    && rd_u32(p.wrapping_add(4)) == DAT_100adf38
                {
                    let o = a694.wrapping_add(u_var1.wrapping_mul(0x54));
                    if (rd_u16(o) & 0x20) == 0 {
                        wr_u16(o, rd_u16(o) | 0x20);
                        let o2 = a696.wrapping_add(u_var1.wrapping_mul(0x54));
                        wr_u16(o2, rd_u16(o2) | bit);
                        FUN_10003742(0xfc, 100, 0x40, 5, 0x50);
                    }
                    FUN_10083b74(param_1, 1, 0);
                }
            }
        } else if u_var2 == 0x200 {
            let i_var3 = FUN_10082d2f(rd_i32(p.wrapping_add(4)), u_var1 as i32, 0) as i32;
            b_var6 = i_var3 < 0;
        } else if u_var2 == 0x400 {
            let i_var3 = FUN_1008304a(rd_i32(p.wrapping_add(4)), u_var1 as i32, 0) as i32;
            b_var6 = i_var3 < 0;
        }

        if b_var6 {
            let o = p.wrapping_add(0xdc);
            (o as usize as *mut u32).write_unaligned(rd_u32(o) | 0x1000);
            return 0;
        }

        if b_var5 {
            DAT_100b9f64 = 0;
        }
        let q = rd_u32(p.wrapping_add(0x4c)).wrapping_add(0x3b);
        if (q as usize as *const u8).read() != 0 {
            (q as usize as *mut u8).write(0);
            if u_var2 == 0x200 {
                let e = eef.add(u_var1 as usize).read();
                if (bit_i & (rd_u16(e.wrapping_add(0x16)) as i16 as i32)) == 0 {
                    let e2 = eef.add(u_var1 as usize).read();
                    if rd_i32(p.wrapping_add(0xc0))
                        < rd_i32(rd_u32(e2.wrapping_add(0x20)).wrapping_add(0x41c))
                            .wrapping_add(20000)
                    {
                        if b_var5 {
                            DAT_100b9f64 = 1;
                        }
                        let e3 = eef.add(u_var1 as usize).read();
                        wr_u16(e3.wrapping_add(0x14), rd_u16(e3.wrapping_add(0x14)) | 0x20);
                        let e4 = eef.add(u_var1 as usize).read();
                        wr_u16(e4.wrapping_add(0x16), rd_u16(e4.wrapping_add(0x16)) | bit);
                    } else if b_var5 {
                        DAT_100b9f64 = 2;
                    }
                } else if b_var5 {
                    DAT_100b9f64 = 3;
                }
            } else if u_var2 == 0x400 {
                let o32 = a032.wrapping_add(u_var1.wrapping_mul(0x40));
                if (bit_i & (rd_u16(o32) as i16 as i32)) == 0 {
                    if rd_i32(p.wrapping_add(0xc0))
                        < rd_i32(a040.wrapping_add(u_var1.wrapping_mul(0x40))).wrapping_add(20000)
                    {
                        if b_var5 {
                            DAT_100b9f64 = 1;
                        }
                        let o30 = a030.wrapping_add(u_var1.wrapping_mul(0x40));
                        wr_u16(o30, rd_u16(o30) | 0x20);
                        let o32b = a032.wrapping_add(u_var1.wrapping_mul(0x40));
                        wr_u16(o32b, rd_u16(o32b) | bit);
                    } else if b_var5 {
                        DAT_100b9f64 = 2;
                    }
                } else if b_var5 {
                    DAT_100b9f64 = 3;
                }
            }
        }
        1
    }
}
