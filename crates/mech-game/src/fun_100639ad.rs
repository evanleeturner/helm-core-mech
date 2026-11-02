// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bec: u8;
    static mut DAT_100a7bf0: u8;
    static mut DAT_100b3904: u32;
    static mut DAT_100eef40: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003c190(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

#[inline(always)]
unsafe fn rd(base: usize, off: usize) -> i32 {
    // SAFETY: caller supplies a valid game-memory address.
    unsafe { ((base.wrapping_add(off)) as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_u16(base: usize, off: usize) -> u16 {
    // SAFETY: caller supplies a valid game-memory address.
    unsafe { ((base.wrapping_add(off)) as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn wr_u16(base: usize, off: usize, v: u16) {
    // SAFETY: caller supplies a valid game-memory address.
    unsafe { ((base.wrapping_add(off)) as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wr(base: usize, off: usize, v: i32) {
    // SAFETY: caller supplies a valid game-memory address.
    unsafe { ((base.wrapping_add(off)) as *mut i32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100639ad(param_1: *mut i32) {
    let p = param_1 as usize;
    // SAFETY: param_1 and everything reachable from it are the game's own
    // structures; globals are the game's memory, accessed as the original does.
    unsafe {
        let s = rd(p, 0) as usize;
        if rd(s, 0x10) != 2 || rd(s, 0x1b4) == 0 || (rd_u16(p, 0x440) & 0x80) == 0 {
            wr_u16(p, 0x440, rd_u16(p, 0x440) & 0xff7f);
            let idx = rd(p, 0x3e0);
            let tbl = rd(p, 0x54);
            let iv6 = rd(
                idx.wrapping_mul(0x40).wrapping_add(4).wrapping_add(tbl) as usize,
                0,
            )
            .wrapping_mul(0x68);
            let a_bc8 = (&raw mut DAT_100a7bc8) as usize;
            let a_bec = (&raw mut DAT_100a7bec) as usize;
            let a_bf0 = (&raw mut DAT_100a7bf0) as usize;
            let b_dd = (s.wrapping_add(0xdd) as *const u8).read();
            if idx == -1
                || rd(s, 0xdc) == 0
                || (b_dd & 0x10) != 0
                || (b_dd & 1) != 0
                || rd(a_bc8.wrapping_add(iv6 as usize), 0) == 0
            {
                wr_u16(p, 0x440, rd_u16(p, 0x440) & 0x7fff);
                wr_u16(p, 0x440, rd_u16(p, 0x440) & 0xffbf);
            } else if (b_dd & 2) == 0
                || (rd_u16(
                    rd(
                        (&raw mut DAT_100eef40)
                            .add((rd(s, 0xdc) as u32 & 0xff) as usize)
                            .read() as usize,
                        0x20,
                    ) as usize,
                    0x442,
                ) & 1)
                    == 0
            {
                let iv2 = rd(s, 0x50).wrapping_sub(rd(s, 200));
                let iv3 = rd(s, 0x54).wrapping_sub(rd(s, 0xcc));
                let iv4 = rd(s, 0x58).wrapping_sub(rd(s, 0xd0));
                let iv5 = FUN_1003c190(iv2, iv3, iv4, rd(a_bec.wrapping_add(iv6 as usize), 0));
                let b1: bool;
                let mut local_14: u32;
                let local_10: u32;
                if iv5 == 0
                    && FUN_1003c190(iv2, iv3, iv4, rd(a_bf0.wrapping_add(iv6 as usize), 0)) != 0
                {
                    b1 = true;
                    let m = (rd(s, 0x60) % 0x1680000).wrapping_add(0x1680000) % 0x1680000;
                    let mut local_18 = rd(s, 0xd4).wrapping_sub(m);
                    if local_18 < 0xb40001 {
                        if local_18 < -0xb40000 {
                            local_18 = local_18.wrapping_add(0x1680000);
                        }
                    } else {
                        local_18 = local_18.wrapping_sub(0x1680000);
                    }
                    local_14 = (local_18 as u32).wrapping_sub((rd(s, 0x6c) % 0x1680000) as u32);
                    if (local_14 as i32) < 0xb40001 {
                        if (local_14 as i32) < -0xb40000 {
                            local_14 = local_14.wrapping_add(0x1680000);
                        }
                    } else {
                        local_14 = local_14.wrapping_add(0xfe980000);
                    }
                    local_10 = (rd(s, 0xd8).wrapping_add(rd(p, 0x1c)) % 0x1680000) as u32;
                } else {
                    b1 = false;
                    local_14 = 0x100001;
                    local_10 = 0x100001;
                    wr_u16(p, 0x440, rd_u16(p, 0x440) & 0x7fff);
                }
                let a14 = {
                    let x = local_14 as i32;
                    (x ^ (x >> 31)).wrapping_sub(x >> 31)
                };
                let a10 = {
                    let x = local_10 as i32;
                    (x ^ (x >> 31)).wrapping_sub(x >> 31)
                };
                if b1 && a14 < 0x100000 && a10 < 0x100000 {
                    wr_u16(p, 0x440, rd_u16(p, 0x440) | 0x8000);
                    if (rd_u16(p, 0x440) & 0x40) == 0 {
                        wr_u16(p, 0x440, rd_u16(p, 0x440) | 0x40);
                        wr(p, 0x3bc, 0x16a);
                    } else if rd(p, 0x3bc) < 1 {
                        wr(p, 0x3bc, 0);
                        wr_u16(p, 0x440, rd_u16(p, 0x440) | 0x80);
                    } else {
                        wr(p, 0x3bc, rd(p, 0x3bc).wrapping_sub(DAT_100b3904 as i32));
                    }
                } else {
                    wr_u16(p, 0x440, rd_u16(p, 0x440) & 0x7fff);
                    if (rd_u16(p, 0x440) & 0x40) != 0 {
                        wr(p, 0x3bc, rd(p, 0x3bc).wrapping_add(DAT_100b3904 as i32));
                        if 0x169 < rd(p, 0x3bc) {
                            wr_u16(p, 0x440, rd_u16(p, 0x440) & 0xffbf);
                        }
                    }
                }
            } else {
                wr(p, 0x3bc, 0);
                wr_u16(p, 0x440, rd_u16(p, 0x440) | 0x40);
                wr_u16(p, 0x440, rd_u16(p, 0x440) | 0x80);
                wr_u16(p, 0x440, rd_u16(p, 0x440) | 0x8000);
            }
        }
    }
}
