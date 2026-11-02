// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(a: i32, b: i32, c: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller guarantees addr points at readable game memory.
    unsafe {
        mech_trace::rd!(
            "100157bd",
            (addr as u32 as usize as *const i32),
            read_unaligned
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100157bd(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("100157bd", param_1, param_2, param_3, param_4);
    let mut b_var1: bool = false;
    let mut b_var2: bool = false;
    // SAFETY: all addresses are the game's own structures, read where the original reads.
    unsafe {
        let u_var6: u32 = mech_trace::rd!(
            "100157bd",
            ((param_1.wrapping_add(2)) as u32 as usize as *const u16),
            read_unaligned
        ) as u32;
        if 2 < u_var6 {
            let pbv = (rd(param_1.wrapping_add(4)) as u32).wrapping_add(param_1 as u32) as usize
                as *const u8;
            let mut local_10: u32 = 0;
            let mut local_18: u32 = u_var6;
            loop {
                if local_18 == 0 {
                    break;
                }
                let idx = mech_trace::rd!(
                    "100157bd",
                    pbv.byte_add(local_18.wrapping_sub(1) as usize),
                    read
                ) as u32;
                let iv4 = idx.wrapping_mul(0x44).wrapping_add(param_2 as u32) as i32;
                if param_3 < rd(iv4.wrapping_add(0x10)) {
                    local_10 |= 2;
                } else {
                    local_10 |= 1;
                }
                if param_4 < rd(iv4.wrapping_add(0x14)) {
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
                let mut iv4: i32 = (mech_trace::rd!("100157bd", pbv, read) as u32)
                    .wrapping_mul(0x44)
                    .wrapping_add(param_2 as u32) as i32;
                loop {
                    let local_c = iv4;
                    let u6 = local_18.wrapping_sub(1);
                    if local_18 == 0 {
                        break;
                    }
                    iv4 = (mech_trace::rd!("100157bd", pbv.byte_add(u6 as usize), read) as u32)
                        .wrapping_mul(0x44)
                        .wrapping_add(param_2 as u32) as i32;
                    local_18 = u6;
                    let y4 = rd(iv4.wrapping_add(0x14));
                    let yc = rd(local_c.wrapping_add(0x14));
                    if (param_4 <= y4 || param_4 <= yc) && (y4 <= param_4 || yc <= param_4) {
                        let x4 = rd(iv4.wrapping_add(0x10));
                        let xc = rd(local_c.wrapping_add(0x10));
                        if y4 == param_4 && yc == param_4 {
                            if x4 < param_3 && xc < param_3 {
                                return mech_trace::rt!("100157bd", 0);
                            }
                            if param_3 < x4 && param_3 < xc {
                                return mech_trace::rt!("100157bd", 0);
                            }
                            return mech_trace::rt!("100157bd", 1);
                        }
                        if y4 != param_4 {
                            if x4 < param_3 && xc < param_3 {
                                if b_var1 {
                                    return mech_trace::rt!("100157bd", 0);
                                }
                                if b_var2 {
                                    return mech_trace::rt!("100157bd", 1);
                                }
                                b_var1 = true;
                            } else if param_3 < x4 && param_3 < xc {
                                if b_var2 {
                                    return mech_trace::rt!("100157bd", 0);
                                }
                                if b_var1 {
                                    return mech_trace::rt!("100157bd", 1);
                                }
                                b_var2 = true;
                            } else {
                                let i5 = mech_trace::cr!(
                                    "100157bd",
                                    "10011e60",
                                    FUN_10011e60(
                                        xc.wrapping_sub(x4),
                                        param_4.wrapping_sub(y4),
                                        yc.wrapping_sub(y4),
                                    )
                                );
                                let i5 = x4.wrapping_add(i5).wrapping_sub(param_3);
                                if i5 < 0 {
                                    if b_var1 {
                                        return mech_trace::rt!("100157bd", 0);
                                    }
                                    if b_var2 {
                                        return mech_trace::rt!("100157bd", 1);
                                    }
                                    b_var1 = true;
                                } else {
                                    if i5 < 1 {
                                        return mech_trace::rt!("100157bd", 1);
                                    }
                                    if b_var2 {
                                        return mech_trace::rt!("100157bd", 0);
                                    }
                                    if b_var1 {
                                        return mech_trace::rt!("100157bd", 1);
                                    }
                                    b_var2 = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mech_trace::rt!("100157bd", 0)
}
