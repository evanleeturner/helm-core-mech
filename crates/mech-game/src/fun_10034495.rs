// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adc4c: i32;
    static mut DAT_100adc28: u32;
    static mut DAT_100adc2c: i32;
    static mut _DAT_100adc30: i32;
    static mut DAT_100adc34: i32;
    static mut DAT_100adc38: i32;
    static mut DAT_100adc3c: i32;
    static mut _DAT_100adc44: i32;
    static mut _DAT_100adc48: i32;
    static mut DAT_100adc50: u32;
    static mut DAT_100adc5c: i32;
    static mut DAT_100adc64: i32;
    static mut DAT_100adc78: u32;
    static mut DAT_100adc84: i32;
    static mut DAT_100adc8c: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_101f52f8: u32;
    static mut DAT_101f52fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003431f(param_1: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10034495(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
) -> u32 {
    // SAFETY: the globals and the callee are the game's own; the ten-word
    // copies walk the game's contiguous records exactly as the original does.
    unsafe {
        if DAT_100adc4c == param_2 {
            if param_1 == 0 {
                let mut p2: *mut u32 = &raw mut DAT_100adc50;
                let mut p3: *mut u32 = &raw mut DAT_100adc78;
                let mut i = 10;
                while i != 0 {
                    p3.write_unaligned(p2.read_unaligned());
                    p2 = p2.add(1);
                    p3 = p3.add(1);
                    i -= 1;
                }
                p2 = &raw mut DAT_100adc28;
                p3 = &raw mut DAT_100adc50;
                i = 10;
                while i != 0 {
                    p3.write_unaligned(p2.read_unaligned());
                    p2 = p2.add(1);
                    p3 = p3.add(1);
                    i -= 1;
                }
                if 0 < DAT_100adc34 {
                    let i_var1 =
                        ((DAT_100adc38 as u32).wrapping_mul(DAT_100b3904 as u32) as i32) >> 0x10;
                    DAT_100adc3c = ((DAT_100adc3c as u32).wrapping_add(0xf) as i32) % 0x168;
                    FUN_1003431f(&raw mut DAT_100adc28);
                    _DAT_100adc30 = (_DAT_100adc30 as u32).wrapping_sub(i_var1 as u32) as i32;
                    DAT_100adc34 = (DAT_100adc34 as u32).wrapping_sub(i_var1 as u32) as i32;
                    DAT_100adc28 = param_3;
                    DAT_100adc2c = param_4;
                }
                if 0 < DAT_100adc5c {
                    DAT_100adc64 = ((DAT_100adc64 as u32).wrapping_add(0xf) as i32) % 0x168;
                    FUN_1003431f(&raw mut DAT_100adc50);
                }
                if DAT_100adc84 < 1 {
                    return 1;
                }
                DAT_100adc8c = ((DAT_100adc8c as u32).wrapping_add(0x1e) as i32) % 0x168;
                FUN_1003431f(&raw mut DAT_100adc78);
            }
        } else {
            DAT_100adc28 = DAT_101f52f8;
            DAT_100adc2c = DAT_101f52fc;
            _DAT_100adc30 = DAT_101f52fc;
            DAT_100adc34 = DAT_101f52fc;
            _DAT_100adc48 = 1;
            if param_5 == 0 {
                _DAT_100adc44 = 0xe;
            } else if param_5 == 2 {
                _DAT_100adc44 = 6;
            } else {
                _DAT_100adc44 = 10;
            }
            DAT_100adc4c = param_2;
            let mut p2: *mut u32 = &raw mut DAT_100adc28;
            let mut p3: *mut u32 = &raw mut DAT_100adc78;
            let mut i = 10;
            while i != 0 {
                p3.write_unaligned(p2.read_unaligned());
                p2 = p2.add(1);
                p3 = p3.add(1);
                i -= 1;
            }
            p2 = &raw mut DAT_100adc78;
            p3 = &raw mut DAT_100adc50;
            i = 10;
            while i != 0 {
                p3.write_unaligned(p2.read_unaligned());
                p2 = p2.add(1);
                p3 = p3.add(1);
                i -= 1;
            }
            if param_1 == 0 {
                FUN_1003431f(&raw mut DAT_100adc28);
            }
        }
        0
    }
}
