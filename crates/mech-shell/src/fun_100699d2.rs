// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10069984: i16;
    static mut DAT_1006998c: i16;
    static mut DAT_100a9cb1: u8;
    static mut DAT_100a8fb1: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069990(param_1: *mut i16, param_2: i32) -> *mut u8;
    fn FUN_10069881(param_1: *mut i32, param_2: i32, param_3: *mut u32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100699d2(param_1: *mut i32, param_2: i32) -> u32 {
    // SAFETY: all pointers are the game's own memory; reads, writes and
    // calls happen in the same order as the original.
    unsafe {
        let mut local_3c: u32 = 0;
        let p1 = param_1.add(1).read();
        let p2 = param_1.add(2).read();
        let p3 = param_1.add(3).read();
        let p4 = param_1.add(4).read();
        let pu_var6: i32 = (p3 as u32).wrapping_sub(p1 as u32).wrapping_add(1) as i32;
        let mut u_var7: i32 = (p4 as u32).wrapping_sub(p2 as u32).wrapping_add(1) as i32;
        let mut local_2c: i32 = 0;
        let i_var2: i32 =
            ((param_2 as u32 as usize).wrapping_add(8) as *const i32).read_unaligned();

        let pu_var8 = FUN_10069990(&raw mut DAT_10069984, param_2);
        let pu_var9: i32 = ((u32::from(*pu_var8) << 8) | u32::from(*pu_var8.add(1))) as i32;
        let mut local_c: u32 = (u32::from(*pu_var8.add(2)) << 8) | u32::from(*pu_var8.add(3));
        if u_var7 <= local_c as i32 {
            local_c = u_var7 as u32;
        }

        if *pu_var8.add(9) != 1 {
            let u_var4 = *pu_var8.add(10);
            local_3c = u32::from(*pu_var8.add(0xd));
            u_var7 = ((pu_var9 as u32 >> 3).wrapping_add(1))
                .wrapping_sub(u32::from((*pu_var8.add(1) & 7) == 0)) as i32;
            let i_var15: i32 = (u_var7 as u32).wrapping_add(u_var7 as u32 & 1) as i32;
            let mut pu_var10 = pu_var9;
            if pu_var6 <= pu_var9 {
                pu_var10 = pu_var6;
            }
            let mut local_14: *mut u8 = FUN_10069990(&raw mut DAT_1006998c, param_2);
            let buf: *mut u8 = &raw mut DAT_100a9cb1;
            let out: *mut u8 = &raw mut DAT_100a8fb1;

            loop {
                let mut pbvar18: *mut u8;
                if u_var4 == 1 {
                    pbvar18 = buf;
                    let end = buf.wrapping_offset(pu_var9.wrapping_add(pu_var9 & 1) as isize);
                    while pbvar18 < end {
                        let pbvar17 = local_14;
                        local_14 = pbvar17.wrapping_add(1);
                        let mut u7: u32 = u32::from(*pbvar17);
                        if u7 != 0x80 {
                            if u7 < 0x81 {
                                let mut c = u7.wrapping_add(1) & 3;
                                while c != 0 {
                                    *pbvar18 = *local_14;
                                    local_14 = local_14.wrapping_add(1);
                                    pbvar18 = pbvar18.wrapping_add(1);
                                    c -= 1;
                                }
                                u7 = u7.wrapping_add(1) >> 2;
                                while u7 != 0 {
                                    let v = (local_14 as *const u32).read_unaligned();
                                    (pbvar18 as *mut u32).write_unaligned(v);
                                    local_14 = local_14.wrapping_add(4);
                                    pbvar18 = pbvar18.wrapping_add(4);
                                    u7 -= 1;
                                }
                            } else {
                                let b14 = *local_14;
                                let b12 = 1u32.wrapping_sub(u32::from(*pbvar17)) as u8;
                                let mut n = u32::from(b12 & 3);
                                while n != 0 {
                                    *pbvar18 = b14;
                                    pbvar18 = pbvar18.wrapping_add(1);
                                    n -= 1;
                                }
                                n = u32::from(b12 >> 2);
                                while n != 0 {
                                    *pbvar18 = b14;
                                    *pbvar18.add(1) = b14;
                                    *pbvar18.add(2) = b14;
                                    *pbvar18.add(3) = b14;
                                    pbvar18 = pbvar18.wrapping_add(4);
                                    n -= 1;
                                }
                                local_14 = pbvar17.wrapping_add(2);
                            }
                        }
                    }
                    pbvar18 = buf;
                } else {
                    pbvar18 = local_14;
                    local_14 = local_14.wrapping_offset(pu_var9.wrapping_add(pu_var9 & 1) as isize);
                }

                if i_var2 == 0x4d424c49 {
                    let mut pu_var19 = out;
                    let mut local_1c: u32 = i_var15 as u32;
                    let mut local_18: u32 = pu_var10 as u32;
                    'blk: loop {
                        let mut b14: u8 = 0x80;
                        loop {
                            let mut i_var16: i32 = 0;
                            let mut c11: u8 = 1;
                            let mut u5: u8 = 0;
                            loop {
                                if (*pbvar18.wrapping_offset(i_var16 as isize) & b14) != 0 {
                                    u5 |= c11;
                                }
                                i_var16 = i_var16.wrapping_add(i_var15);
                                c11 = c11.wrapping_shl(1);
                                if c11 == 0 {
                                    break;
                                }
                            }
                            *pu_var19 = u5;
                            pu_var19 = pu_var19.wrapping_add(1);
                            local_18 = local_18.wrapping_sub(1);
                            if local_18 == 0 {
                                break 'blk;
                            }
                            b14 >>= 1;
                            if b14 == 0 {
                                break;
                            }
                        }
                        pbvar18 = pbvar18.wrapping_add(1);
                        local_1c = local_1c.wrapping_sub(1);
                        if local_1c == 0 {
                            break;
                        }
                    }
                    FUN_10069881(param_1, local_2c, out as *mut u32, pu_var10);
                } else {
                    FUN_10069881(param_1, local_2c, pbvar18 as *mut u32, pu_var10);
                }
                local_2c = local_2c.wrapping_add(1);
                local_c = local_c.wrapping_sub(1);
                if local_c == 0 {
                    break;
                }
            }
        }
        local_3c
    }
}
