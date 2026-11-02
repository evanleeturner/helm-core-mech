// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(a: i32, b: i32, c: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller guarantees the address is readable game memory.
    unsafe { mech_trace::rd!("100150df", (addr as usize as *const i32), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100150df(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("100150df", param_1, param_2, param_3, param_4);
    let mut b_var1: bool = false;
    let mut b_var2: bool = false;
    // SAFETY: param_1 and param_2 point at the game's own structures; every
    // read stays where the original reads.
    unsafe {
        let mut u_var6: u32 = u32::from(mech_trace::rd!(
            "100150df",
            ((param_1.wrapping_add(2)) as usize as *const u16),
            read_unaligned
        ));
        if 2 < u_var6 {
            let pbvar3: *const u8 = (rd(param_1.wrapping_add(4)) as u32)
                .wrapping_add(param_1 as u32) as usize
                as *const u8;
            let mut local_10: u32 = 0;
            let mut local_18: u32 = u_var6;
            loop {
                if local_18 == 0 {
                    break;
                }
                let idx = local_18.wrapping_sub(1) as usize;
                let i_var4 = u32::from(mech_trace::rd!("100150df", pbvar3.add(idx), read))
                    .wrapping_mul(0x44)
                    .wrapping_add(param_2 as u32) as i32;
                if param_3 < rd(i_var4.wrapping_add(0xc)) {
                    local_10 |= 2;
                } else {
                    local_10 |= 1;
                }
                if param_4 < rd(i_var4.wrapping_add(0x14)) {
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
                let mut i_var4: i32 = u32::from(mech_trace::rd!("100150df", pbvar3, read))
                    .wrapping_mul(0x44)
                    .wrapping_add(param_2 as u32) as i32;
                loop {
                    let local_c = i_var4;
                    u_var6 = local_18.wrapping_sub(1);
                    if local_18 == 0 {
                        break;
                    }
                    i_var4 = u32::from(mech_trace::rd!(
                        "100150df",
                        pbvar3.add(u_var6 as usize),
                        read
                    ))
                    .wrapping_mul(0x44)
                    .wrapping_add(param_2 as u32) as i32;
                    local_18 = u_var6;
                    let a14 = rd(i_var4.wrapping_add(0x14));
                    let c14 = rd(local_c.wrapping_add(0x14));
                    if (param_4 <= a14 || param_4 <= c14) && (a14 <= param_4 || c14 <= param_4) {
                        if a14 == param_4 && c14 == param_4 {
                            let a0c = rd(i_var4.wrapping_add(0xc));
                            let c0c = rd(local_c.wrapping_add(0xc));
                            if a0c < param_3 && c0c < param_3 {
                                return mech_trace::rt!("100150df", 0);
                            }
                            if param_3 < a0c && param_3 < c0c {
                                return mech_trace::rt!("100150df", 0);
                            }
                            return mech_trace::rt!("100150df", 1);
                        }
                        if a14 != param_4 {
                            let a0c = rd(i_var4.wrapping_add(0xc));
                            let c0c = rd(local_c.wrapping_add(0xc));
                            if a0c < param_3 && c0c < param_3 {
                                if b_var1 {
                                    return mech_trace::rt!("100150df", 0);
                                }
                                if b_var2 {
                                    return mech_trace::rt!("100150df", 1);
                                }
                                b_var1 = true;
                            } else if param_3 < a0c && param_3 < c0c {
                                if b_var2 {
                                    return mech_trace::rt!("100150df", 0);
                                }
                                if b_var1 {
                                    return mech_trace::rt!("100150df", 1);
                                }
                                b_var2 = true;
                            } else {
                                let mut i_var5 = mech_trace::cr!(
                                    "100150df",
                                    "10011e60",
                                    FUN_10011e60(
                                        c0c.wrapping_sub(a0c),
                                        param_4.wrapping_sub(a14),
                                        c14.wrapping_sub(a14),
                                    )
                                );
                                i_var5 = a0c.wrapping_add(i_var5).wrapping_sub(param_3);
                                if i_var5 < 0 {
                                    if b_var1 {
                                        return mech_trace::rt!("100150df", 0);
                                    }
                                    if b_var2 {
                                        return mech_trace::rt!("100150df", 1);
                                    }
                                    b_var1 = true;
                                } else {
                                    if i_var5 < 1 {
                                        return mech_trace::rt!("100150df", 1);
                                    }
                                    if b_var2 {
                                        return mech_trace::rt!("100150df", 0);
                                    }
                                    if b_var1 {
                                        return mech_trace::rt!("100150df", 1);
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
    mech_trace::rt!("100150df", 0)
}
