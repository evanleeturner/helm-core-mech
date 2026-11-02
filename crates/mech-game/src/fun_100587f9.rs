// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b12c1: [u8; 4];
    static mut DAT_100b12c9: [u8; 4];
    static mut DAT_100b12cd: [u8; 4];
    static mut DAT_100b12d1: [u8; 4];
    static mut DAT_100b12e9: [u8; 4];
    static mut DAT_100b12ed: [u8; 4];
    static mut DAT_100b12f1: [u8; 4];
    static mut DAT_100b12f5: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10058b0e(param_1: i32, param_2: u8, param_3: i32);
}

unsafe fn rd(p: *const [u8; 4]) -> i32 {
    // SAFETY: p addresses one of the game's packed 4-byte globals.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr(p: *mut [u8; 4], v: i32) {
    // SAFETY: p addresses one of the game's packed 4-byte globals.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100587f9(
    param_1: *mut i32,
    param_2: u8,
    param_3: i32,
    param_4: i32,
    param_5: *mut u32,
) -> i32 {
    // SAFETY: all pointers are the caller's image/rect/output memory and the
    // game's own globals; accesses mirror the original in order.
    unsafe {
        let pi_var1 = (*param_1) as u32 as *const i32;
        let mut i_var11 = pi_var1.add(1).read_unaligned();
        let i_var6 = (i_var11 as u32).wrapping_add(1) as i32;
        if i_var6 == 0 || i_var11 < -1 {
            return -1;
        }
        let mut i_var2 = pi_var1.add(2).read_unaligned();
        if i_var2 < 0 {
            return -1;
        }

        let i_var3 = param_1.add(1).read_unaligned();
        let mut i_var7 = i_var3;
        if i_var3 < 1 {
            i_var7 = 0;
        }
        let i_var4 = param_1.add(2).read_unaligned();
        let mut local_14 = i_var4;
        if local_14 < 1 {
            local_14 = 0;
        }
        let mut i_var8 = param_1.add(3).read_unaligned();
        if i_var11 <= i_var8 {
            i_var8 = i_var11;
        }
        i_var11 = param_1.add(4).read_unaligned();
        if i_var2 <= i_var11 {
            i_var11 = i_var2;
        }
        if i_var8 < i_var7 || i_var11 < local_14 {
            return -2;
        }

        i_var2 = pi_var1.read_unaligned();
        wr(&raw mut DAT_100b12c1, param_5 as u32 as i32);
        if !param_5.is_null() {
            let p1 = param_1.add(1).read_unaligned();
            let p2 = param_1.add(2).read_unaligned();
            let p3 = param_1.add(3).read_unaligned();
            let p4 = param_1.add(4).read_unaligned();
            param_5.write_unaligned(
                ((p3.wrapping_sub(p1) as i16 as u16 as u32) << 16)
                    | (p4.wrapping_sub(p2) as i16 as u16 as u32),
            );
            param_5.add(1).write_unaligned(
                ((param_3 as i16 as u16 as u32) << 16) | (param_4 as i16 as u16 as u32),
            );
            param_5.add(2).write_unaligned(0);
            param_5.add(3).write_unaligned(0);
            param_5.add(4).write_unaligned(0xffff_ffff);
            param_5.add(5).write_unaligned(0xffff_ffff);
        }

        let i_var9 = i_var8.wrapping_add(1).wrapping_sub(i_var7);
        wr(&raw mut DAT_100b12e9, 0x7fff_ffff);
        wr(&raw mut DAT_100b12ed, 0x7fff_ffff);
        wr(&raw mut DAT_100b12f1, -0x7fff_ffff);
        wr(&raw mut DAT_100b12f5, -0x7fff_ffff);
        let p_row0 = (local_14 as u32)
            .wrapping_mul(i_var6 as u32)
            .wrapping_add(i_var2 as u32)
            .wrapping_add(i_var7 as u32);
        wr(&raw mut DAT_100b12c9, p_row0 as i32);

        loop {
            let mut b_var14 = local_14 == i_var11;
            if i_var11 < local_14 {
                break;
            }

            let p_row = rd(&raw const DAT_100b12c9) as u32;
            wr(&raw mut DAT_100b12cd, p_row as i32);
            let mut i_var12 = i_var9;
            let mut pb = p_row;
            loop {
                if i_var12 == 0 {
                    break;
                }
                i_var12 = i_var12.wrapping_sub(1);
                b_var14 = param_2 == (pb as *const u8).read();
                pb = pb.wrapping_add(1);
                if !b_var14 {
                    break;
                }
            }

            let mut i_var5 = rd(&raw const DAT_100b12f1);
            if !b_var14 {
                let g_ed = rd(&raw const DAT_100b12ed);
                if local_14 <= g_ed {
                    wr(&raw mut DAT_100b12ed, local_14);
                }
                let g_f5 = rd(&raw const DAT_100b12f5);
                if g_f5 <= local_14 {
                    wr(&raw mut DAT_100b12f5, local_14);
                }
                let g_e9 = rd(&raw const DAT_100b12e9);
                let mut i_var10 = i_var8.wrapping_sub(i_var12);
                if g_e9 <= i_var10 {
                    i_var10 = g_e9;
                }

                let p_end = p_row.wrapping_add(i_var9 as u32).wrapping_sub(1);
                wr(&raw mut DAT_100b12cd, p_end as i32);
                b_var14 = p_end == 0;
                i_var12 = i_var9;
                let mut pb = p_end;
                loop {
                    if i_var12 == 0 {
                        break;
                    }
                    i_var12 = i_var12.wrapping_sub(1);
                    b_var14 = param_2 == (pb as *const u8).read();
                    pb = pb.wrapping_sub(1);
                    if !b_var14 {
                        break;
                    }
                }

                wr(&raw mut DAT_100b12e9, i_var10);
                if !b_var14 {
                    let g_f1 = rd(&raw const DAT_100b12f1);
                    i_var5 = i_var7.wrapping_add(i_var12);
                    if i_var5 <= g_f1 {
                        i_var5 = g_f1;
                    }
                }
            }
            wr(&raw mut DAT_100b12f1, i_var5);
            let p_row = rd(&raw const DAT_100b12c9);
            wr(&raw mut DAT_100b12c9, p_row.wrapping_add(i_var6));
            local_14 = local_14.wrapping_add(1);
        }

        let g_e9 = rd(&raw const DAT_100b12e9);
        let g_ed = rd(&raw const DAT_100b12ed);
        let g_f1 = rd(&raw const DAT_100b12f1);
        let g_f5 = rd(&raw const DAT_100b12f5);
        if !param_5.is_null() {
            param_5
                .add(2)
                .write_unaligned(g_e9.wrapping_sub(param_3.wrapping_add(i_var3)) as u32);
            param_5
                .add(3)
                .write_unaligned(g_ed.wrapping_sub(param_4.wrapping_add(i_var4)) as u32);
            param_5
                .add(4)
                .write_unaligned(g_f1.wrapping_sub(param_3.wrapping_add(i_var3)) as u32);
            param_5
                .add(5)
                .write_unaligned(g_f5.wrapping_sub(param_4.wrapping_add(i_var4)) as u32);
        }
        let p_out = param_5.wrapping_add(6);
        wr(&raw mut DAT_100b12d1, p_out as u32 as i32);
        let i_var11 = g_f1.wrapping_add(1).wrapping_sub(g_e9);
        let p_row = (g_ed as u32)
            .wrapping_mul(i_var6 as u32)
            .wrapping_add(i_var2 as u32)
            .wrapping_add(g_e9 as u32);
        wr(&raw mut DAT_100b12c9, p_row as i32);

        local_14 = rd(&raw const DAT_100b12ed);
        loop {
            let g_f5 = rd(&raw const DAT_100b12f5);
            if !(local_14 <= g_f5) {
                break;
            }
            FUN_10058b0e(i_var11, param_2, i_var7);
            let p_row = rd(&raw const DAT_100b12c9);
            wr(&raw mut DAT_100b12c9, p_row.wrapping_add(i_var6));
            local_14 = local_14.wrapping_add(1);
        }

        let p_out = rd(&raw const DAT_100b12d1) as u32;
        p_out.wrapping_sub(param_5 as u32) as i32
    }
}
