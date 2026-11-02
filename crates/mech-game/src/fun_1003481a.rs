// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10114e20: i32;
    static mut DAT_100adc20: i32;
    static mut DAT_100adc24: i32;
    static mut DAT_100c6b74: i32;
    static mut DAT_100c6b78: i32;
    static mut DAT_100c6b7c: i32;
    static mut DAT_100c6b8c: i32;
    static mut DAT_100c6b90: i32;
    static mut DAT_100c6b98: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100351e7(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1003517a(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32);
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003481a(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    let _ = (param_2, param_3);
    let mut param_4 = param_4;
    // SAFETY: the globals and callees are the game's own; param_1 and the
    // structures reached through it and DAT_10114e20 are the caller's memory,
    // read where the original reads.
    unsafe {
        let mut i_var3 = DAT_10114e20;
        let pi_var1 = (i_var3.wrapping_add(0xa4) as *const *mut i32).read_unaligned();
        let flags = ((param_1.read_unaligned()).wrapping_add(0xdd) as *const u8).read();
        if (flags & 1) == 0 {
            param_4 = param_5;
        }
        if param_1.add(0xfc).read_unaligned() != 2 {
            let flags = ((param_1.read_unaligned()).wrapping_add(0xdd) as *const u8).read();
            if (flags & 0xf) == 0 {
                return;
            }
            let flags = ((param_1.read_unaligned()).wrapping_add(0xdd) as *const u8).read();
            if (flags & 0x10) != 0 {
                return;
            }
            if -0x30000 < param_6 {
                FUN_100351e7(
                    DAT_100adc20,
                    DAT_100adc24
                        .wrapping_sub(DAT_100c6b74)
                        .wrapping_sub(DAT_100c6b90),
                    0x25,
                    pi_var1 as i32,
                );
            }
            if param_6 < 0x30000 {
                FUN_100351e7(
                    DAT_100adc20,
                    DAT_100c6b74
                        .wrapping_add(DAT_100adc24)
                        .wrapping_add(DAT_100c6b98),
                    0x1c,
                    pi_var1 as i32,
                );
            }
        }
        if param_4 == 0 {
            FUN_1003517a(DAT_100adc20, DAT_100adc24, 0x10, pi_var1);
            FUN_1003517a(DAT_100adc20, DAT_100adc24, 0x16, pi_var1);
        } else {
            let i_var4 = DAT_100adc24;
            let i_var2 = FUN_10003d60(DAT_100c6b8c, param_4);
            FUN_1003517a(DAT_100adc20.wrapping_add(i_var2), i_var4, 0xd, pi_var1);
        }
        if -3 < param_4 {
            let s = (i_var3.wrapping_add(0xb8) as *const i16).read_unaligned();
            i_var3 = (s as i32).wrapping_add(DAT_100c6b78);
            let i_var4 = DAT_100adc24.wrapping_add(DAT_100c6b7c / 2);
            FUN_100351e7(i_var3.wrapping_add(-1), i_var4, 0x22, pi_var1 as i32);
            if 0x5a < param_4 {
                FUN_100351e7(i_var3, i_var4, 0x22, pi_var1 as i32);
            }
        }
        if param_4 < 3 {
            i_var3 = 0i32.wrapping_sub(DAT_100c6b78);
            let i_var4 = DAT_100adc24.wrapping_add(DAT_100c6b7c / 2);
            FUN_100351e7(i_var3, i_var4, 0x1f, pi_var1 as i32);
            if param_4 < -0x5a {
                FUN_100351e7(i_var3.wrapping_add(-1), i_var4, 0x1f, pi_var1 as i32);
            }
        }
    }
}
