// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083878: u8;
    static mut DAT_100839fc: u32;
    static mut DAT_10083a00: u8;
    static mut DAT_10083c70: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a180(param_1: i32) -> *mut u32 {
    let i_var2: i32 = param_1 / 100;
    let mut local_14: *mut u32 = i_var2 as usize as *mut u32;
    let i_var4: i32 = param_1 % 100;
    if 0 < param_1 {
        // SAFETY: the globals and tables are the game's own memory; pointers
        // walk the tables exactly as the original does.
        unsafe {
            let mut outer: i32 = 8;
            let mut pi_var5: *mut i32 = (&raw mut DAT_10083878) as *mut i32;
            loop {
                let mut local_c: i32 = 0xc;
                loop {
                    let mut i_var6: i32 = pi_var5.read_unaligned();
                    if param_1 == i_var6 {
                        i_var6 = 0;
                    } else if 0 < i_var6 && (i_var6 < 7000 || 9999 < i_var6) {
                        if i_var6 / 100 == i_var2 && i_var4 < i_var6 % 100 {
                            i_var6 = i_var6.wrapping_sub(1);
                        } else if param_1 < 7000 && 9999 < i_var6 {
                            if i_var6.wrapping_sub(10000) / 100 == param_1 {
                                i_var6 = 0;
                            } else if i_var6.wrapping_sub(10000) / 10000 == i_var2
                                && i_var4 < (i_var6 / 100) % 100
                            {
                                i_var6 = i_var6.wrapping_sub(100);
                            }
                        }
                    }
                    pi_var5.write_unaligned(i_var6);
                    pi_var5 = pi_var5.wrapping_add(1);
                    local_c = local_c.wrapping_sub(1);
                    if local_c == 0 {
                        break;
                    }
                }
                outer = outer.wrapping_sub(1);
                if outer == 0 {
                    break;
                }
            }
            let mut count: i32 = 0;
            let mut pi_var3: *mut i32 = (&raw mut DAT_10083a00) as *mut i32;
            pi_var5 = pi_var3;
            let end: *mut i32 = (&raw mut DAT_10083c70) as *mut i32;
            loop {
                let mut i_var6: i32 = pi_var3.read_unaligned();
                if param_1 == i_var6 {
                    i_var6 = -1;
                    DAT_100839fc = DAT_100839fc.wrapping_sub(1);
                } else if 0 < i_var6 && (i_var6 < 7000 || 9999 < i_var6) {
                    if i_var6 / 100 == i_var2 && i_var4 < i_var6 % 100 {
                        i_var6 = i_var6.wrapping_sub(1);
                    } else if param_1 < 7000 && 9999 < i_var6 {
                        if i_var6.wrapping_sub(10000) / 100 == param_1 {
                            i_var6 = -1;
                            DAT_100839fc = DAT_100839fc.wrapping_sub(1);
                        } else if i_var6.wrapping_sub(10000) / 10000 == i_var2
                            && i_var4 < (i_var6 / 100) % 100
                        {
                            i_var6 = i_var6.wrapping_sub(100);
                        }
                    }
                }
                pi_var3.write_unaligned(i_var6);
                let mut pi_var7: *mut i32 = pi_var5;
                if i_var6 != -1 {
                    let i_var1: i32 = pi_var3.wrapping_add(1).read_unaligned();
                    pi_var5.write_unaligned(i_var6);
                    pi_var7 = pi_var5.wrapping_add(2);
                    count = count.wrapping_add(1);
                    pi_var5.wrapping_add(1).write_unaligned(i_var1);
                }
                pi_var3 = pi_var3.wrapping_add(2);
                pi_var5 = pi_var7;
                if !(pi_var3 < end) {
                    break;
                }
            }
            local_14 = count as usize as *mut u32;
            if count < 0x4e {
                let mut p: *mut u32 = (&raw mut DAT_10083a00)
                    .wrapping_byte_add((count as usize).wrapping_mul(2))
                    as *mut u32;
                loop {
                    p.write_unaligned(0xffff_ffff);
                    p = p.wrapping_add(2);
                    local_14 = p;
                    if !(p < end as *mut u32) {
                        break;
                    }
                }
            }
        }
    }
    local_14
}
