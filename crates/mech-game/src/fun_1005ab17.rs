// SPDX-License-Identifier: MIT

#[inline(always)]
fn scarry4(a: u32, b: u32) -> bool {
    let s = a.wrapping_add(b);
    (((s ^ a) & (s ^ b)) >> 31) != 0
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ab17(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    // SAFETY: all pointers are the game's own structures and buffers, read and
    // written at the offsets the original uses.
    unsafe {
        let pi1 = param_1.read_unaligned() as u32 as *const i32;
        let mut i_var10 = pi1.add(1).read_unaligned();
        let i_var5 = i_var10.wrapping_add(1);
        if i_var5 == 0 || i_var10 < -1 {
            return -1;
        }
        let i_var9 = pi1.add(2).read_unaligned();
        if i_var9 < 0 {
            return -1;
        }
        let i_var2 = param_1.add(1).read_unaligned();
        let mut i_var6 = if i_var2 < 1 { 0 } else { i_var2 };
        let i_var3 = param_1.add(2).read_unaligned();
        let mut i_var7 = if i_var3 < 1 { 0 } else { i_var3 };
        let mut i_var8 = param_1.add(3).read_unaligned();
        if i_var10 <= param_1.add(3).read_unaligned() {
            i_var8 = i_var10;
        }
        i_var10 = param_1.add(4).read_unaligned();
        if i_var9 <= param_1.add(4).read_unaligned() {
            i_var10 = i_var9;
        }
        if i_var8 < i_var6 || i_var10 < i_var7 {
            return -2;
        }
        param_2 = param_2.wrapping_add(i_var2);
        param_3 = param_3.wrapping_add(i_var3);
        let mut u_var15: u32 = ((param_4.wrapping_add(8)) as u32 as *const u32).read_unaligned();
        let off = (param_5
            .wrapping_mul(4)
            .wrapping_add(param_4)
            .wrapping_add(0x10)) as u32 as *const i32;
        let mut p16: u32 = (off.read_unaligned() as u32).wrapping_add(param_4 as u32);
        let mut local_c: u32 = 0;
        let mut local_8: u32 = (p16 as *const u32).read_unaligned();
        if local_8 != 0 {
            p16 = p16.wrapping_add(4);
            let i_var9 = (i_var8.wrapping_add(1))
                .wrapping_sub(local_8 as i32)
                .wrapping_sub(param_2);
            let mut u_var12 = local_8;
            let mut ok = true;
            if !(-1 < i_var9) {
                u_var12 = local_8.wrapping_add(i_var9 as u32);
                ok = u_var12 != 0 && scarry4(local_8, i_var9 as u32) == ((u_var12 as i32) < 0);
            }
            if ok {
                i_var6 = param_2.wrapping_sub(i_var6);
                if i_var6 < 0 {
                    let b = scarry4(u_var12, i_var6 as u32);
                    u_var12 = u_var12.wrapping_add(i_var6 as u32);
                    if u_var12 == 0 || b != ((u_var12 as i32) < 0) {
                        return local_8.wrapping_add(local_c) as i32;
                    }
                    p16 = p16.wrapping_sub(i_var6 as u32);
                    param_2 = param_2.wrapping_sub(i_var6);
                }
                i_var10 = (i_var10.wrapping_add(1))
                    .wrapping_sub(u_var15 as i32)
                    .wrapping_sub(param_3);
                let mut ok2 = true;
                if !(-1 < i_var10) {
                    let b = scarry4(u_var15, i_var10 as u32);
                    u_var15 = u_var15.wrapping_add(i_var10 as u32);
                    ok2 = u_var15 != 0 && b == ((u_var15 as i32) < 0);
                }
                if ok2 {
                    i_var7 = param_3.wrapping_sub(i_var7);
                    if i_var7 < 0 {
                        let b = scarry4(u_var15, i_var7 as u32);
                        u_var15 = u_var15.wrapping_add(i_var7 as u32);
                        if u_var15 == 0 || b != ((u_var15 as i32) < 0) {
                            return local_8.wrapping_add(local_c) as i32;
                        }
                        param_3 = param_3.wrapping_sub(i_var7);
                        p16 = p16.wrapping_sub((i_var7 as u32).wrapping_mul(local_8));
                    }
                    let mut pb: u32 = param_3
                        .wrapping_mul(i_var5)
                        .wrapping_add(pi1.read_unaligned())
                        .wrapping_add(param_2) as u32;
                    local_8 = local_8.wrapping_sub(u_var12);
                    let mut u_var13 = u_var12;
                    if param_6 == 0 {
                        loop {
                            u_var13 = u_var12 >> 1;
                            while u_var13 != 0 {
                                let v = (p16 as *const u16).read_unaligned();
                                (pb as *mut u16).write_unaligned(v);
                                p16 = p16.wrapping_add(2);
                                pb = pb.wrapping_add(2);
                                u_var13 = u_var13.wrapping_sub(1);
                            }
                            u_var13 = ((u_var12 & 1) != 0) as u32;
                            while u_var13 != 0 {
                                *(pb as *mut u8) = *(p16 as *const u8);
                                p16 = p16.wrapping_add(1);
                                pb = pb.wrapping_add(1);
                                u_var13 = u_var13.wrapping_sub(1);
                            }
                            p16 = p16.wrapping_add(local_8);
                            pb = pb.wrapping_add((i_var5 as u32).wrapping_sub(u_var12));
                            u_var15 = u_var15.wrapping_sub(1);
                            if u_var15 == 0 {
                                break;
                            }
                        }
                        let _ = u_var13;
                        return local_8.wrapping_add(u_var12) as i32;
                    }
                    loop {
                        let mut u_var14 = u_var12;
                        local_c = u_var12;
                        if u_var13 == 0 {
                            break;
                        }
                        loop {
                            let idx = *(p16 as *const u8) as u32;
                            let b4 = *((param_6 as u32).wrapping_add(idx) as *const u8);
                            if b4 != 0xff {
                                *(pb as *mut u8) = b4;
                            }
                            p16 = p16.wrapping_add(1);
                            pb = pb.wrapping_add(1);
                            u_var14 = u_var14.wrapping_sub(1);
                            if u_var14 == 0 {
                                break;
                            }
                        }
                        p16 = p16.wrapping_add(local_8);
                        pb = pb.wrapping_add((i_var5 as u32).wrapping_sub(u_var12));
                        u_var15 = u_var15.wrapping_sub(1);
                        u_var13 = u_var15;
                    }
                }
            }
        }
        local_8.wrapping_add(local_c) as i32
    }
}
