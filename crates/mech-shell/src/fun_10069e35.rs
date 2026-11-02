// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10069ccb(param_1: i32);
    fn FUN_10069d2c(param_1: u32, param_2: i32) -> u32;
    fn FUN_10069d72(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10069db8();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069e35(param_1: u32, param_2: i32, param_3: *mut u32) -> u8 {
    // SAFETY: the globals and callees are the game's own; param_2 points at
    // the GIF data and param_3 at the decoder state, accessed where the
    // original does.
    unsafe {
        DAT_100aa2c4 = param_1;
        let base = param_3 as *mut u8;
        let src = param_2 as *const u8;

        // zero 2 bytes then 0xb dwords
        base.write(0);
        base.add(1).write(0);
        for i in 0..0xb {
            base.add(2 + i * 4).cast::<u32>().write_unaligned(0);
        }

        let u_var1 = src.add(0xb).read();
        let mut p = src.add(0xd);
        let flags = src.add(10).read();
        if flags & 0x80 != 0 {
            let n = (1i32.wrapping_shl(((flags & 7) as u32).wrapping_add(1))).wrapping_mul(3);
            p = p.offset(n as isize);
        }
        base.add(36)
            .cast::<u32>()
            .write_unaligned(p.add(5).cast::<u16>().read_unaligned() as u32);
        base.add(40)
            .cast::<u32>()
            .write_unaligned(p.add(7).cast::<u16>().read_unaligned() as u32);
        let b_var2 = p.add(9).read();
        base.add(0xb).write(b_var2);
        base.add(0xb).write(base.add(0xb).read() & 0x40);
        let mut pb8 = p.add(10);
        if b_var2 & 0x80 != 0 {
            let n = (1i32.wrapping_shl(((b_var2 & 7) as u32).wrapping_add(1))).wrapping_mul(3);
            pb8 = pb8.offset(n as isize);
        }
        base.add(16).cast::<u32>().write_unaligned(0);
        let mut pb10 = pb8.add(1);
        let u_var6 = pb8.read() as u32;
        let u_var3: u32 = 1u32.wrapping_shl((pb8.read() & 0x1f) as u32);
        base.add(28)
            .cast::<u32>()
            .write_unaligned(u_var6.wrapping_add(1));
        FUN_10069ccb(u_var3 as i32);
        let mut local_14: u32 = 0xffff;
        let mut local_18: i32 = 0;
        base.add(0x2d).write(0);
        let v = base.add(36).cast::<u32>().read_unaligned();
        base.add(32).cast::<u32>().write_unaligned(v);
        base.add(8).cast::<u32>().write_unaligned(0);
        base.add(12).cast::<u32>().write_unaligned(0);

        loop {
            let i_var5 = base.add(28).cast::<u32>().read_unaligned() as i32;
            let mut u_var4: u32;
            if i_var5 < 9 {
                u_var4 = FUN_10069d2c(u_var6, i_var5);
            } else {
                u_var4 = FUN_10069d2c(u_var6, 8);
                let u_var7 = FUN_10069d2c(u_var6, i_var5.wrapping_sub(8));
                u_var4 = u_var7.wrapping_shl(8) | u_var4;
            }
            if u_var4 == u_var3 {
                FUN_10069ccb(u_var3 as i32);
                base.add(28)
                    .cast::<u32>()
                    .write_unaligned(u_var6.wrapping_add(1));
                local_14 = 0xffff;
            } else {
                let mut pb8 = pb10;
                if u_var4 == u_var3.wrapping_add(1) {
                    loop {
                        while base.add(16).cast::<u32>().read_unaligned() != 0 {
                            let c = base.add(16).cast::<u32>().read_unaligned();
                            base.add(16)
                                .cast::<u32>()
                                .write_unaligned(c.wrapping_sub(1));
                            pb8 = pb8.add(1);
                        }
                        pb10 = pb8.add(1);
                        base.add(16)
                            .cast::<u32>()
                            .write_unaligned(pb8.read() as u32);
                        pb8 = pb10;
                        if base.add(16).cast::<u32>().read_unaligned() == 0 {
                            break;
                        }
                    }
                    local_18 = 0xffff;
                } else {
                    let off = (u_var4.wrapping_mul(2).wrapping_add(0x302e)) as usize;
                    if base.add(off).cast::<i16>().read_unaligned() == -2 {
                        FUN_10069d72(local_14 as i32);
                    } else if local_14 != 0xffff {
                        FUN_10069d72(local_14 as i32);
                    }
                    let mut i_var5: i32 = 0;
                    let mut pu9 = base.add(0x2e);
                    let mut u_var7 = u_var4;
                    loop {
                        let o1 = u_var7.wrapping_add(0x202e) as usize;
                        pu9.write(base.add(o1).read());
                        pu9 = pu9.add(1);
                        i_var5 = i_var5.wrapping_add(1);
                        let o2 = u_var7.wrapping_mul(2).wrapping_add(0x302e) as usize;
                        u_var7 = base.add(o2).cast::<u16>().read_unaligned() as u32;
                        if u_var7 == 0xffff {
                            break;
                        }
                    }
                    loop {
                        FUN_10069db8();
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
