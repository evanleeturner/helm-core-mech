// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8a79: [u8; 4];
    static mut DAT_100a8a81: [u8; 4];
    static mut DAT_100a8a85: [u8; 4];
    static mut DAT_100a8a89: [u8; 4];
    static mut DAT_100a8aa1: [u8; 4];
    static mut DAT_100a8aa5: [u8; 4];
    static mut DAT_100a8aa9: [u8; 4];
    static mut DAT_100a8aad: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100676ae(param_1: i32, param_2: u8, param_3: i32);
}

unsafe fn rd(p: *const [u8; 4]) -> i32 {
    // SAFETY: p points at one of the game's 4-byte globals.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr(p: *mut [u8; 4], v: i32) {
    // SAFETY: p points at one of the game's 4-byte globals.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067399(
    param_1: *mut i32,
    param_2: u8,
    param_3: i32,
    param_4: i32,
    param_5: *mut u32,
) -> i32 {
    // SAFETY: all pointers are the caller's buffers and the game's own
    // globals; accesses are made where the original makes them.
    unsafe {
        let pi_var1 = param_1.read_unaligned() as u32 as usize as *mut i32;
        let mut i_var11 = pi_var1.add(1).read_unaligned();
        let i_var6 = i_var11.wrapping_add(1);
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
        if i_var4 < 1 {
            local_14 = 0;
        }
        let mut i_var8 = param_1.add(3).read_unaligned();
        if i_var11 <= param_1.add(3).read_unaligned() {
            i_var8 = i_var11;
        }
        i_var11 = param_1.add(4).read_unaligned();
        if i_var2 <= param_1.add(4).read_unaligned() {
            i_var11 = i_var2;
        }
        if i_var8 < i_var7 || i_var11 < local_14 {
            return -2;
        }
        i_var2 = pi_var1.read_unaligned();
        wr(&raw mut DAT_100a8a79, param_5 as usize as u32 as i32);
        if !param_5.is_null() {
            let a = (param_1.add(3).read_unaligned() as i16 as i32)
                .wrapping_sub(param_1.add(1).read_unaligned() as i16 as i32);
            let b = (param_1.add(4).read_unaligned() as i16 as i32)
                .wrapping_sub(param_1.add(2).read_unaligned() as i16 as i32);
            param_5.write_unaligned(((a as u16 as u32) << 16) | (b as u16 as u32));
            param_5.add(1).write_unaligned(
                ((param_3 as i16 as u16 as u32) << 16) | (param_4 as i16 as u16 as u32),
            );
            param_5.add(2).write_unaligned(0);
            param_5.add(3).write_unaligned(0);
            param_5.add(4).write_unaligned(0xffff_ffff);
            param_5.add(5).write_unaligned(0xffff_ffff);
        }
        let i_var9 = (i_var8 as u32).wrapping_add(1).wrapping_sub(i_var7 as u32) as i32;
        wr(&raw mut DAT_100a8aa1, 0x7fff_ffff);
        wr(&raw mut DAT_100a8aa5, 0x7fff_ffff);
        wr(&raw mut DAT_100a8aa9, -0x7fff_ffff);
        wr(&raw mut DAT_100a8aad, -0x7fff_ffff);
        let p0 = (local_14 as u32)
            .wrapping_mul(i_var6 as u32)
            .wrapping_add(i_var2 as u32)
            .wrapping_add(i_var7 as u32);
        wr(&raw mut DAT_100a8a81, p0 as i32);
        loop {
            let mut b_var14 = local_14 == i_var11;
            if i_var11 < local_14 {
                if !param_5.is_null() {
                    let t0 = rd(&raw const DAT_100a8aa1) as u32;
                    param_5.add(2).write_unaligned(
                        t0.wrapping_sub((param_3 as u32).wrapping_add(i_var3 as u32)),
                    );
                    let t0 = rd(&raw const DAT_100a8aa5) as u32;
                    param_5.add(3).write_unaligned(
                        t0.wrapping_sub((param_4 as u32).wrapping_add(i_var4 as u32)),
                    );
                    let t0 = rd(&raw const DAT_100a8aa9) as u32;
                    param_5.add(4).write_unaligned(
                        t0.wrapping_sub((param_3 as u32).wrapping_add(i_var3 as u32)),
                    );
                    let t0 = rd(&raw const DAT_100a8aad) as u32;
                    param_5.add(5).write_unaligned(
                        t0.wrapping_sub((param_4 as u32).wrapping_add(i_var4 as u32)),
                    );
                }
                let pu = (param_5 as usize as u32).wrapping_add(24);
                wr(&raw mut DAT_100a8a89, pu as i32);
                let t0 = rd(&raw const DAT_100a8aa9);
                let t1 = rd(&raw const DAT_100a8aa1);
                i_var11 = (t0 as u32).wrapping_add(1).wrapping_sub(t1 as u32) as i32;
                let t0 = rd(&raw const DAT_100a8aa5);
                let t1 = rd(&raw const DAT_100a8aa1);
                let p0 = (t0 as u32)
                    .wrapping_mul(i_var6 as u32)
                    .wrapping_add(i_var2 as u32)
                    .wrapping_add(t1 as u32);
                wr(&raw mut DAT_100a8a81, p0 as i32);
                local_14 = rd(&raw const DAT_100a8aa5);
                loop {
                    let t0 = rd(&raw const DAT_100a8aad);
                    if !(local_14 <= t0) {
                        break;
                    }
                    FUN_100676ae(i_var11, param_2, i_var7);
                    let p0 = (rd(&raw const DAT_100a8a81) as u32).wrapping_add(i_var6 as u32);
                    wr(&raw mut DAT_100a8a81, p0 as i32);
                    local_14 = local_14.wrapping_add(1);
                }
                let pu = rd(&raw const DAT_100a8a89) as u32;
                return pu.wrapping_sub(param_5 as usize as u32) as i32;
            }
            let p0 = rd(&raw const DAT_100a8a81) as u32;
            wr(&raw mut DAT_100a8a85, p0 as i32);
            let mut i_var12 = i_var9;
            let mut pb = p0;
            loop {
                if i_var12 == 0 {
                    break;
                }
                i_var12 = i_var12.wrapping_sub(1);
                b_var14 = param_2 == (pb as usize as *const u8).read();
                pb = pb.wrapping_add(1);
                if !b_var14 {
                    break;
                }
            }
            let mut i_var5 = rd(&raw const DAT_100a8aa9);
            if !b_var14 {
                let t0 = rd(&raw const DAT_100a8aa5);
                if local_14 <= t0 {
                    wr(&raw mut DAT_100a8aa5, local_14);
                }
                let t0 = rd(&raw const DAT_100a8aad);
                if t0 <= local_14 {
                    wr(&raw mut DAT_100a8aad, local_14);
                }
                let mut i_var10 = i_var8.wrapping_sub(i_var12);
                let t0 = rd(&raw const DAT_100a8aa1);
                if t0 <= i_var8.wrapping_sub(i_var12) {
                    i_var10 = t0;
                }
                let p0 = rd(&raw const DAT_100a8a81) as u32;
                let p1 = p0.wrapping_add(i_var9 as u32).wrapping_sub(1);
                wr(&raw mut DAT_100a8a85, p1 as i32);
                b_var14 = p1 == 0;
                i_var12 = i_var9;
                let mut pb = p1;
                loop {
                    if i_var12 == 0 {
                        break;
                    }
                    i_var12 = i_var12.wrapping_sub(1);
                    b_var14 = param_2 == (pb as usize as *const u8).read();
                    pb = pb.wrapping_sub(1);
                    if !b_var14 {
                        break;
                    }
                }
                wr(&raw mut DAT_100a8aa1, i_var10);
                if !b_var14 {
                    i_var5 = i_var7.wrapping_add(i_var12);
                    let t0 = rd(&raw const DAT_100a8aa9);
                    if i_var7.wrapping_add(i_var12) <= t0 {
                        i_var5 = t0;
                    }
                }
            }
            wr(&raw mut DAT_100a8aa9, i_var5);
            let p0 = (rd(&raw const DAT_100a8a81) as u32).wrapping_add(i_var6 as u32);
            wr(&raw mut DAT_100a8a81, p0 as i32);
            local_14 = local_14.wrapping_add(1);
        }
    }
}
