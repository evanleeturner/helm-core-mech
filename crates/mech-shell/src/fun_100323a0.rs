// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_1009200c: i32;
    static mut DAT_100a67b8: i32;
    static mut DAT_100a67c8: i32;
    static mut DAT_10082be8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10032640(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1000d230(param_1: i32) -> i8;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000c5d0(param_1: i32, param_2: i32) -> *mut i32;
    fn FUN_10017df0(param_1: i32, param_2: *mut i32);
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100323a0(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    if param_2 < 1 {
        return 0;
    }
    // SAFETY: all callees and globals are the game's own memory; pointers
    // returned by the game's collection functions are dereferenced exactly
    // where the original does.
    unsafe {
        let i_var2 = FUN_10032640(param_1, param_2, param_3);
        if i_var2 != 1 {
            return 0;
        }
        let c_var1 = FUN_1000d230(param_2);
        let mut i_var2: i32 = c_var1 as u8 as i32;
        let i_var3 = FUN_1000d260(i_var2, param_2);
        let flag_ptr = || DAT_10083508.wrapping_add(0x23bb) as *const i32;
        if i_var2 == 0 {
            let i_flag = flag_ptr().read_unaligned();
            if i_flag == 1 {
                let pi = FUN_1000c5d0(i_var3, 0);
                if !pi.is_null() {
                    pi.add(2).cast::<u8>().write(0);
                    FUN_10017df0(DAT_1009200c, pi);
                    return 1;
                }
            }
        } else if i_var2 == 1 {
            i_var2 = 0;
            if 0 < ((DAT_100a67b8.wrapping_add(4)) as *const i32).read_unaligned() {
                let pi: *mut i32;
                loop {
                    let i_flag = flag_ptr().read_unaligned();
                    if i_flag == 1 {
                        let p = FUN_10018060(DAT_100a67b8, i_var2) as *mut i32;
                        if !p.is_null() && p.read_unaligned() == i_var3 {
                            pi = p;
                            break;
                        }
                    }
                    i_var2 = i_var2.wrapping_add(1);
                    if ((DAT_100a67b8.wrapping_add(4)) as *const i32).read_unaligned() <= i_var2 {
                        return 1;
                    }
                }
                FUN_10017ec0(DAT_100a67b8, pi as i32, 0);
                pi.add(2).cast::<u8>().write(1);
                FUN_10017df0(DAT_1009200c, pi);
                return 1;
            }
        } else if i_var2 == 2 {
            let mut i_var5: i32 = 0;
            let base = (&raw mut DAT_10082be8) as *const u8;
            let i_var2 = base
                .byte_add((i_var3 as u32).wrapping_mul(0xc) as usize)
                .cast::<i32>()
                .read_unaligned();
            if 0 < ((DAT_100a67c8.wrapping_add(4)) as *const i32).read_unaligned() {
                loop {
                    let i_flag = flag_ptr().read_unaligned();
                    if i_flag == 1 {
                        let p = FUN_10018060(DAT_100a67c8, i_var5) as *mut i32;
                        if p.add(2).cast::<u8>().read() as i8 == 2 {
                            let matched: bool;
                            if i_var2 < 0x1f41 {
                                if i_var2 == 8000 {
                                    let v = p.read_unaligned();
                                    matched = v == 1 || v == 2;
                                } else if i_var2 == 7000 && p.read_unaligned() == 10 {
                                    matched = true;
                                } else {
                                    matched = false;
                                }
                            } else if i_var2 == 0x2134 {
                                matched = p.read_unaligned() == 6;
                            } else if i_var2 != 0x251c
                                && i_var2 == 0x254e
                                && p.read_unaligned() == 5
                            {
                                matched = true;
                            } else {
                                matched = false;
                            }
                            if matched {
                                FUN_10017ec0(DAT_100a67c8, p as i32, 0);
                                FUN_10017df0(DAT_1009200c, p);
                                return 1;
                            }
                        }
                    }
                    i_var5 = i_var5.wrapping_add(1);
                    if ((DAT_100a67c8.wrapping_add(4)) as *const i32).read_unaligned() <= i_var5 {
                        return 1;
                    }
                }
            }
        }
    }
    1
}
