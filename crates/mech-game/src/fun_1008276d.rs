// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b38d8: u32;
    static mut DAT_100b38dc: u32;
    static mut DAT_100b9f60: u32;
    static mut DAT_100b9f68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10082adc(param_1: u32, param_2: i32, param_3: u32) -> u32;
    fn FUN_10082d2f(param_1: i32, param_2: i32, param_3: u32) -> u32;
    fn FUN_1008304a(param_1: i32, param_2: i32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008276d(param_1: i32, mut param_2: i32, param_3: u32) {
    // SAFETY: param_1 points at the game's object; the globals and callees
    // are the game's own, accessed in the original's order.
    unsafe {
        let mut b_var2 = false;
        let p1 = param_1 as usize;
        if ((p1 + 4) as *const u32).read_unaligned() == DAT_100adf38 {
            DAT_100b9f68 = 0;
        }
        let u_var1 = ((p1 + 0xdc) as *const u32).read_unaligned();
        let mut local_20: u32 = u_var1 & 0xf00;
        let mut local_14: u32;
        if param_2 == 0 || (u_var1 & 0x1000) != 0 {
            local_20 = 0x200;
            local_14 = 0;
            param_2 = 1;
        } else {
            local_14 = (u_var1 & 0xff).wrapping_add(param_2 as u32);
        }
        let i_var3: i32 = (DAT_100b9f60 as i32)
            .wrapping_add(DAT_100b38dc as i32)
            .wrapping_add(DAT_100b38d8 as i32);
        let mut local_1c: i32 = 0;
        while local_1c < i_var3.wrapping_add(3) && !b_var2 {
            local_1c = local_1c.wrapping_add(1);
            let f4 = ((p1 + 4) as *const u32).read_unaligned();
            if local_20 == 0x100 {
                let r = FUN_10082adc(f4, local_14 as i32, param_3) as i32;
                if r == -2 {
                    local_14 = 0;
                    local_20 = 0x200;
                } else if r == -1 {
                    local_14 = DAT_100b38dc.wrapping_sub(1);
                    local_20 = 0x400;
                } else if r == 1 {
                    b_var2 = true;
                } else {
                    local_14 = local_14.wrapping_add(param_2 as u32);
                }
            } else if local_20 == 0x200 {
                let r = FUN_10082d2f(f4 as i32, local_14 as i32, param_3) as i32;
                if r == -2 {
                    local_14 = 0;
                    local_20 = 0x400;
                } else if r == -1 {
                    local_14 = DAT_100b9f60.wrapping_sub(1);
                    local_20 = 0x100;
                } else if r == 1 {
                    b_var2 = true;
                } else {
                    local_14 = local_14.wrapping_add(param_2 as u32);
                }
            } else if local_20 == 0x400 {
                let r = FUN_1008304a(f4 as i32, local_14 as i32, param_3) as i32;
                if r == -2 {
                    local_14 = 0;
                    local_20 = 0x100;
                } else if r == -1 {
                    local_14 = DAT_100b38d8.wrapping_sub(1);
                    local_20 = 0x200;
                } else if r == 1 {
                    b_var2 = true;
                } else {
                    local_14 = local_14.wrapping_add(param_2 as u32);
                }
            } else {
                local_14 = 0;
                local_20 = 0x200;
            }
        }
        if b_var2 {
            ((p1 + 0xdc) as *mut u32).write_unaligned(local_14 | local_20);
            if local_20 == 0x100 {
                ((p1 + 0x104) as *mut u32).write_unaligned(local_14 | 0x100);
            }
        } else {
            let v = ((p1 + 0xdc) as *const u32).read_unaligned();
            ((p1 + 0xdc) as *mut u32).write_unaligned(v | 0x1000);
        }
        let b105 = ((p1 + 0x105) as *const u8).read();
        let p20 = ((p1 + 0x20) as *const u32).read_unaligned() as usize;
        if (b105 & 1) == 0 && ((p20 + 0x3f0) as *const i32).read_unaligned() != 0 {
            let p4c = ((p1 + 0x4c) as *const u32).read_unaligned() as usize;
            ((p4c + 0x43) as *mut u8).write(1);
        }
    }
}
