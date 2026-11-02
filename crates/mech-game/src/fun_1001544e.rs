// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(a: i32, b: i32, c: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(addr: i32, off: i32) -> i32 {
    // SAFETY: addr is a game-provided record address; the offset is within the record.
    unsafe { (addr.wrapping_add(off) as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001544e(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let mut b_var1: u8 = 0;
    let mut b_var2: u8 = 0;
    // SAFETY: param_1 points at the game's polygon header, param_2 at its
    // vertex table; all reads stay where the original reads.
    unsafe {
        let mut u_var6: u32 =
            u32::from(((param_1.wrapping_add(2)) as u32 as usize as *const u16).read_unaligned());
        if 2 < u_var6 {
            let pbv = (rd(param_1, 4) as u32).wrapping_add(param_1 as u32) as usize as *const u8;
            let mut local_10: u32 = 0;
            let mut local_18: u32 = u_var6;
            loop {
                if local_18 == 0 {
                    break;
                }
                let idx = pbv.add(local_18.wrapping_sub(1) as usize).read();
                let i_var4 = u32::from(idx)
                    .wrapping_mul(0x44)
                    .wrapping_add(param_2 as u32) as i32;
                if param_3 < rd(i_var4, 0xc) {
                    local_10 |= 2;
                } else {
                    local_10 |= 1;
                }
                if param_4 < rd(i_var4, 0x10) {
                    local_10 |= 8;
                } else {
                    local_10 |= 4;
                }
                local_18 = local_18.wrapping_sub(1);
                if local_10 == 0xf {
                    break;
                }
            }
            if local_10 == 0xf {
                local_18 = u_var6;
                let mut i_var4 = u32::from(pbv.read())
                    .wrapping_mul(0x44)
                    .wrapping_add(param_2 as u32) as i32;
                loop {
                    let local_c = i_var4;
                    u_var6 = local_18.wrapping_sub(1);
                    if local_18 == 0 {
                        break;
                    }
                    let idx = pbv.add(u_var6 as usize).read();
                    i_var4 = u32::from(idx)
                        .wrapping_mul(0x44)
                        .wrapping_add(param_2 as u32) as i32;
                    local_18 = u_var6;
                    if (param_4 <= rd(i_var4, 0x10) || param_4 <= rd(local_c, 0x10))
                        && (rd(i_var4, 0x10) <= param_4 || rd(local_c, 0x10) <= param_4)
                    {
                        if rd(i_var4, 0x10) == param_4 && rd(local_c, 0x10) == param_4 {
                            if rd(i_var4, 0xc) < param_3 && rd(local_c, 0xc) < param_3 {
                                return 0;
                            }
                            if param_3 < rd(i_var4, 0xc) && param_3 < rd(local_c, 0xc) {
                                return 0;
                            }
                            return 1;
                        }
                        if rd(i_var4, 0x10) != param_4 {
                            if rd(i_var4, 0xc) < param_3 && rd(local_c, 0xc) < param_3 {
                                if b_var1 != 0 {
                                    return 0;
                                }
                                if b_var2 != 0 {
                                    return 1;
                                }
                                b_var1 = 1;
                            } else if param_3 < rd(i_var4, 0xc) && param_3 < rd(local_c, 0xc) {
                                if b_var2 != 0 {
                                    return 0;
                                }
                                if b_var1 != 0 {
                                    return 1;
                                }
                                b_var2 = 1;
                            } else {
                                let a = rd(local_c, 0xc).wrapping_sub(rd(i_var4, 0xc));
                                let b = param_4.wrapping_sub(rd(i_var4, 0x10));
                                let c = rd(local_c, 0x10).wrapping_sub(rd(i_var4, 0x10));
                                let i_var5 = FUN_10011e60(a, b, c);
                                let i_var5 =
                                    rd(i_var4, 0xc).wrapping_add(i_var5).wrapping_sub(param_3);
                                if i_var5 < 0 {
                                    if b_var1 != 0 {
                                        return 0;
                                    }
                                    if b_var2 != 0 {
                                        return 1;
                                    }
                                    b_var1 = 1;
                                } else {
                                    if i_var5 < 1 {
                                        return 1;
                                    }
                                    if b_var2 != 0 {
                                        return 0;
                                    }
                                    if b_var1 != 0 {
                                        return 1;
                                    }
                                    b_var2 = 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}
