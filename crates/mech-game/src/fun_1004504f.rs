// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10044761(param_1: i32, param_2: i16) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10051f43(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004504f(param_1: i32) -> i32 {
    // SAFETY: param_1 points at the game's object; the globals and callees
    // are the game's own, and reads happen in the original's order.
    unsafe {
        let idx =
            ((param_1.wrapping_add(0x17a) as usize as *const u16).read_unaligned() & 0xff) as usize;
        let i_var3: i32 = (*(&raw const DAT_100eef40).cast::<u32>().add(idx)) as i32;
        let i_var4: i32 = 0x2d0000;
        let w = (param_1.wrapping_add(4) as usize as *const u32).read_unaligned();
        let u_var2 = FUN_10044761(i_var3, ((w as u16) | 0x200) as i16) as u32;
        let i_var4 = FUN_10002570(u_var2, i_var4) as i32;
        let mut local_10: i32 = i_var4.wrapping_add(0x8000) >> 0x10;
        if 7 < local_10 {
            local_10 = 0;
        }
        if local_10 == 0 || local_10 == 1 || local_10 == 7 {
            if local_10 == 0 {
                let r = FUN_10051f43(2);
                if r == 0 {
                    local_10 = 6;
                } else {
                    local_10 = 2;
                }
                let addr = local_10.wrapping_add(0x1f6).wrapping_add(param_1);
                if (addr as usize as *const i8).read() != 0 {
                    if local_10 == 2 {
                        local_10 = 6;
                    } else {
                        local_10 = 2;
                    }
                }
            }
            if local_10 == 1 {
                local_10 = 3;
            }
            if local_10 == 7 {
                local_10 = 5;
            }
        } else {
            let mut b_var1 = false;
            let mut local_c: i32;
            if local_10 < 5 {
                local_c = 2;
                while local_c < 5 && !b_var1 {
                    let addr = local_c.wrapping_add(0x1f6).wrapping_add(i_var3);
                    if (addr as usize as *const i8).read() == 0 {
                        b_var1 = true;
                    }
                    local_c = local_c.wrapping_add(1);
                }
                local_c = local_c.wrapping_sub(1);
            } else {
                local_c = 6;
                while 3 < local_c && !b_var1 {
                    let addr = local_c.wrapping_add(0x1f6).wrapping_add(i_var3);
                    if (addr as usize as *const i8).read() == 0 {
                        b_var1 = true;
                    }
                    local_c = local_c.wrapping_sub(1);
                }
                local_c = local_c.wrapping_add(1);
            }
            local_10 = local_c;
        }
        local_10.wrapping_mul(2)
    }
}
