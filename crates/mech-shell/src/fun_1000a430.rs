// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083878: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a430(param_1: i32, param_2: i32, mut param_3: i32) -> u32 {
    if param_2 < 1 {
        return 0;
    }
    // SAFETY: the table is the game's own memory; every pointer stays within
    // the rows the original indexes, read and written where it does.
    unsafe {
        let base = (&raw mut DAT_10083878) as *mut u8;
        let mut i_var2: i32 = 0;
        if param_3 != 0 {
            let mut p1 = base
                .wrapping_offset(param_1.wrapping_mul(0xc) as isize)
                .cast::<i32>();
            loop {
                if 0xb < i_var2 {
                    break;
                }
                if p1.read_unaligned() == 0 {
                    param_3 = param_3.wrapping_sub(1);
                    p1.write_unaligned(param_2);
                }
                p1 = p1.wrapping_add(1);
                i_var2 = i_var2.wrapping_add(1);
                if param_3 == 0 {
                    break;
                }
            }
        }
        if param_2.wrapping_sub(param_2 % 100) == 0xd48 {
            if param_3 != 0 {
                let mut local_4: i32 = 0;
                match param_1 {
                    1 => local_4 = 4,
                    3 => local_4 = 5,
                    4 => local_4 = 1,
                    5 => local_4 = 3,
                    _ => {}
                }
                i_var2 = 0;
                if param_3 != 0 {
                    let mut p1 = base
                        .wrapping_offset(local_4.wrapping_mul(0xc) as isize)
                        .cast::<i32>();
                    loop {
                        if 0xb < i_var2 {
                            break;
                        }
                        if p1.read_unaligned() == 0 {
                            param_3 = param_3.wrapping_sub(1);
                            p1.write_unaligned(param_2);
                        }
                        p1 = p1.wrapping_add(1);
                        i_var2 = i_var2.wrapping_add(1);
                        if param_3 == 0 {
                            break;
                        }
                    }
                    if param_3 != 0 {
                        let mut n: i32 = 0xc;
                        let mut p1 = base
                            .wrapping_offset(local_4.wrapping_mul(0xc) as isize)
                            .cast::<i32>();
                        let mut p3 = base
                            .wrapping_offset(param_1.wrapping_mul(0xc) as isize)
                            .cast::<i32>();
                        loop {
                            if p3.read_unaligned() == param_2 {
                                p3.write_unaligned(0);
                            }
                            if p1.read_unaligned() == param_2 {
                                p1.write_unaligned(0);
                            }
                            p1 = p1.wrapping_add(1);
                            p3 = p3.wrapping_add(1);
                            n = n.wrapping_sub(1);
                            if n == 0 {
                                break;
                            }
                        }
                        return 0;
                    }
                }
            }
        } else if param_3 != 0 {
            let mut n: i32 = 0xc;
            let mut p1 = base
                .wrapping_offset(param_1.wrapping_mul(0xc) as isize)
                .cast::<i32>();
            loop {
                if p1.read_unaligned() == param_2 {
                    p1.write_unaligned(0);
                }
                p1 = p1.wrapping_add(1);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
            return 0;
        }
    }
    1
}
