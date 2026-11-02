// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100adf38: i32;
    static mut DAT_10123240: [u8; 1];
    static mut DAT_10123130: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> i32;
    fn FUN_1002b7de(param_1: i32, param_2: *mut i16);
    fn FUN_1002b6fe(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1002a1e0(param_1: u32, param_2: u32) -> i32;
    fn FUN_10044933(param_1: i32) -> i32;
    fn FUN_1002b4cf(param_1: *mut i8, param_2: i32, param_3: i16, param_4: i16);
    fn FUN_1002b48b(param_1: i32) -> u32;
    fn FUN_1004c173(param_1: i32);
    fn FUN_1002a3e0(param_1: i32, param_2: i16, param_3: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002bd38(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; the object and
    // record pointers come from the game's tables and are read and written
    // at the same offsets as the original.
    unsafe {
        let arr = (&raw mut DAT_100eef40).cast::<i32>();
        let i_var1 = FUN_1004d47f(param_1);
        if i_var1 == -1 {
            return;
        }
        let mut local_14: *mut u8 = arr.offset(i_var1 as isize).read() as *mut u8;
        if local_14.byte_add(0x170).cast::<i16>().read_unaligned() != 0 {
            FUN_1002b7de(param_1, local_14.byte_add(0x170).cast::<i16>());
            local_14.byte_add(0x170).cast::<i16>().write_unaligned(0);
        }
        if DAT_100adf38 == i_var1 {
            return;
        }
        let local_18: *mut u8 = (&raw mut DAT_10123240)
            .cast::<u8>()
            .wrapping_offset((param_1 as u32).wrapping_mul(0x3c0a) as i32 as isize);
        let local_34: i32 = (&raw mut DAT_10123130)
            .cast::<i32>()
            .offset(param_1 as isize)
            .read();
        if local_34 == -1 {
            return;
        }
        let rec: *mut u8 =
            local_18.wrapping_offset((local_34 as u32).wrapping_mul(0x13f) as i32 as isize);
        let local_c: *mut u8 = rec.wrapping_offset(0x3a);
        let local_20: u32 = rec.wrapping_offset(0x3c).cast::<u32>().read_unaligned();
        if local_20 == 0 {
            return;
        }
        if (local_20 & 0xe10) != 0 {
            return;
        }
        let mut local_54: [i32; 8] = [0; 8];
        let mut i_var2 = FUN_1002b6fe(param_1, local_54.as_mut_ptr() as i32, 0);
        if i_var2 == 0 {
            return;
        }

        let mut local_24: u16 = 0;
        let mut local_28: i32 = 0;
        let mut local_60: *mut u8 = local_c.byte_add(0xef);
        while local_28 < local_c.byte_add(0xee).read() as i32 && local_24 == 0 {
            let mut local_30: i32 = local_c.byte_add(0x79).cast::<i32>().read_unaligned();
            if local_30 == 0 {
                local_30 = i_var2;
            }
            local_24 = local_60.cast::<u16>().read_unaligned();
            let i_var3 = FUN_1002a1e0(local_24 as u32, local_20);
            if i_var3 == 0 {
                let mut local_2c: i32 = 0;
                while local_2c < i_var2 && local_30 != 0 {
                    local_14 = local_54[local_2c as usize] as *mut u8;
                    if local_14.byte_add(0x17a).cast::<u16>().read_unaligned() as u32
                        == local_24 as u32
                    {
                        let kind = local_14.byte_add(0x176).cast::<i16>().read_unaligned();
                        let l = local_20 as i32;
                        let mut lab = false;
                        if l < 5 {
                            if local_20 == 4 {
                                if kind == 7 {
                                    local_30 = local_30.wrapping_sub(1);
                                }
                            } else if 0 < l && l < 3 && (kind == 2 || kind == 3) {
                                local_30 = local_30.wrapping_sub(1);
                            }
                        } else if l < 0x21 {
                            if local_20 == 0x20 {
                                lab = true;
                            } else if local_20 == 8 && kind == 2 {
                                local_30 = local_30.wrapping_sub(1);
                            }
                        } else if l < 0x201 {
                            if local_20 == 0x200 || local_20 == 0x100 {
                                lab = true;
                            }
                        } else if local_20 == 0x400 {
                            lab = true;
                        } else if local_20 == 0x1000 {
                            local_30 = local_30.wrapping_sub(1);
                        } else if local_20 == 0x2000 && kind == 4 {
                            local_30 = local_30.wrapping_sub(1);
                        }
                        if lab && local_14.byte_add(0x176).cast::<i16>().read_unaligned() == 8 {
                            local_30 = local_30.wrapping_sub(1);
                        }
                    }
                    local_2c = local_2c.wrapping_add(1);
                }
                if local_30 == 0 {
                    local_24 = 0;
                }
            } else {
                local_24 = 0;
            }
            local_28 = local_28.wrapping_add(1);
            local_60 = local_60.byte_add(2);
        }
        if local_24 == 0 {
            return;
        }

        local_14 = core::ptr::null_mut();
        local_28 = 0;
        while local_28 < i_var2 {
            let e = local_54[local_28 as usize];
            let k = (e as *const u8)
                .byte_add(0x176)
                .cast::<i16>()
                .read_unaligned();
            if (k == 0
                || (k == 5
                    && FUN_10044933(
                        (e as *const u8)
                            .byte_add(0x20)
                            .cast::<i32>()
                            .read_unaligned(),
                    ) == 0))
                && e != arr.offset(i_var1 as isize).read()
            {
                local_14 = e as *mut u8;
                break;
            }
            local_28 = local_28.wrapping_add(1);
        }
        if local_14.is_null() {
            let cur = arr.offset(i_var1 as isize).read() as *const u8;
            if cur.byte_add(0x176).cast::<i16>().read_unaligned() != 0 {
                return;
            }
            i_var2 = FUN_10044933(cur.byte_add(0x20).cast::<i32>().read_unaligned());
            if i_var2 != 0 {
                return;
            }
            local_14 = arr.offset(i_var1 as isize).read() as *mut u8;
        }

        local_28 = 0;
        while local_28 < 0x10 {
            if (local_20 & 1u32.wrapping_shl(local_28 as u8 as u32 & 0x1f)) != 0 {
                let u_var4 = FUN_1004d47f(param_1);
                FUN_1002b4cf(
                    local_14 as *mut i8,
                    2,
                    (local_28 as i16).wrapping_add(1),
                    u_var4 as i16,
                );
                break;
            }
            local_28 = local_28.wrapping_add(1);
        }

        let mut local_8: u32 = 0x40;
        let i_var3 = rec.wrapping_offset(0xb7).cast::<i32>().read_unaligned();
        let u_var5 = FUN_1002b48b(i_var3);
        local_8 |= u_var5;

        let mut local_1c: i16 = -1;
        let l = local_20 as i32;
        let mut set8 = false;
        if l < 5 {
            if local_20 == 4 {
                local_1c = 7;
            } else if 0 < l && l < 3 {
                local_1c = 2;
            }
        } else if l < 0x21 {
            if local_20 != 0x20 {
                if local_20 == 8 {
                    local_1c = 2;
                }
            } else {
                set8 = true;
            }
        } else if l < 0x201 {
            if local_20 == 0x200 || local_20 == 0x100 {
                set8 = true;
            }
        } else if local_20 != 0x400 {
            if local_20 == 0x1000 {
                local_1c = 0;
                FUN_1004c173(local_14 as i32);
            } else if local_20 == 0x2000 {
                local_1c = 4;
            }
        } else {
            set8 = true;
        }
        if set8 {
            local_1c = 8;
        }

        if local_1c != -1 {
            FUN_1002a3e0(local_14 as i32, local_1c, local_24);
            let mut fld: u32 = local_14.byte_add(0x17e).cast::<u32>().read_unaligned();
            fld |= local_8;
            local_14.byte_add(0x17e).cast::<u32>().write_unaligned(fld);
        }
    }
}
