// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2b0c: u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005b12b(param_1: i32);
    fn FUN_1005b18c(param_1: u32, param_2: i32) -> u32;
    fn FUN_1005b1d2(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005b218();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b295(param_1: u32, param_2: i32, param_3: *mut u32) -> u32 {
    // SAFETY: the globals and callees are the game's own; param_2 points at
    // GIF data and param_3 at the decoder state, accessed where the original does.
    unsafe {
        DAT_100b2b0c = param_1;
        let mut puvar11: *mut u8 = param_3.cast::<u8>();
        let mut i: i32 = 2;
        while i != 0 {
            puvar11.write(0);
            puvar11 = puvar11.byte_add(1);
            i -= 1;
        }
        let mut puvar11: *mut u32 = puvar11.cast::<u32>();
        i = 0xb;
        while i != 0 {
            puvar11.write_unaligned(0);
            puvar11 = puvar11.byte_add(4);
            i -= 1;
        }
        let rb = |a: i32| -> u8 { (a as u32 as *const u8).read() };
        let rw = |a: i32| -> u16 { (a as u32 as *const u16).read_unaligned() };

        let u_var1: u32 = rb(param_2.wrapping_add(0xb)) as u32;
        let mut i_var5: i32 = param_2.wrapping_add(0xd);
        let flags = rb(param_2.wrapping_add(10));
        if (flags & 0x80) != 0 {
            i_var5 = i_var5.wrapping_add(
                1i32.wrapping_shl(((flags & 7) as u32).wrapping_add(1))
                    .wrapping_mul(3),
            );
        }
        let p = param_3;
        let w = |idx: usize, v: u32| p.add(idx).write_unaligned(v);
        let r = |idx: usize| -> u32 { p.add(idx).read_unaligned() };

        w(9, rw(i_var5.wrapping_add(5)) as u32);
        w(10, rw(i_var5.wrapping_add(7)) as u32);
        let b_var2 = rb(i_var5.wrapping_add(9));
        p.add(0xb).cast::<u8>().write(b_var2);
        let t = p.add(0xb).cast::<u8>().read();
        p.add(0xb).cast::<u8>().write(t & 0x40);
        let mut pbvar8: i32 = i_var5.wrapping_add(10);
        if (b_var2 & 0x80) != 0 {
            pbvar8 = pbvar8.wrapping_add(
                1i32.wrapping_shl(((b_var2 & 7) as u32).wrapping_add(1))
                    .wrapping_mul(3),
            );
        }
        w(4, 0);
        let mut pbvar10: i32 = pbvar8.wrapping_add(1);
        let u_var6: u32 = rb(pbvar8) as u32;
        let u_var3: u32 = 1u32.wrapping_shl(u_var6 & 0x1f);
        w(7, u_var6.wrapping_add(1));
        FUN_1005b12b(u_var3 as i32);
        let mut local_14: u32 = 0xffff;
        let mut local_18: i32 = 0;
        param_3.cast::<u8>().byte_add(0x2d).write(0);
        w(8, r(9));
        w(2, 0);
        w(3, 0);
        let base = param_3 as u32;
        loop {
            let i_var5 = r(7) as i32;
            let mut u_var4: u32;
            if i_var5 < 9 {
                u_var4 = FUN_1005b18c(u_var6, i_var5);
            } else {
                u_var4 = FUN_1005b18c(u_var6, 8);
                let u_var7 = FUN_1005b18c(u_var6, i_var5.wrapping_sub(8));
                u_var4 = (u_var7 << 8) | u_var4;
            }
            if u_var4 == u_var3 {
                FUN_1005b12b(u_var3 as i32);
                w(7, u_var6.wrapping_add(1));
                local_14 = 0xffff;
            } else {
                pbvar8 = pbvar10;
                if u_var4 == u_var3.wrapping_add(1) {
                    loop {
                        while r(4) != 0 {
                            w(4, r(4).wrapping_sub(1));
                            pbvar8 = pbvar8.wrapping_add(1);
                        }
                        pbvar10 = pbvar8.wrapping_add(1);
                        w(4, rb(pbvar8) as u32);
                        pbvar8 = pbvar10;
                        if r(4) == 0 {
                            break;
                        }
                    }
                    local_18 = 0xffff;
                } else {
                    let s = base
                        .wrapping_add(u_var4.wrapping_mul(2))
                        .wrapping_add(0x302e) as *const i16;
                    if s.read_unaligned() == -2 {
                        FUN_1005b1d2(local_14 as i32);
                    } else if local_14 != 0xffff {
                        FUN_1005b1d2(local_14 as i32);
                    }
                    let mut i_var5: i32 = 0;
                    let mut puvar9: u32 = base.wrapping_add(0x2e);
                    let mut u_var7: u32 = u_var4;
                    loop {
                        let src = u_var7.wrapping_add(0x202e).wrapping_add(base) as *const u8;
                        (puvar9 as *mut u8).write(src.read());
                        puvar9 = puvar9.wrapping_add(1);
                        i_var5 = i_var5.wrapping_add(1);
                        let ws = base
                            .wrapping_add(u_var7.wrapping_mul(2))
                            .wrapping_add(0x302e) as *const u16;
                        u_var7 = ws.read_unaligned() as u32;
                        if u_var7 == 0xffff {
                            break;
                        }
                    }
                    loop {
                        FUN_1005b218();
                        i_var5 = i_var5.wrapping_sub(1);
                        local_14 = u_var4;
                        if i_var5 == 0 {
                            break;
                        }
                    }
                }
            }
            if local_18 != 0 {
                break;
            }
        }
        u_var1
    }
}
