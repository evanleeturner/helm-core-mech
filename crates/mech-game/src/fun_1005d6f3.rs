// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: u32;
    static mut DAT_100b3218: u32;
    static mut DAT_100b32a8: u32;
    static mut DAT_100b32ac: u32;
    static mut DAT_100b32b0: u32;
    static mut DAT_100b32b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006b1f5(param_1: u32) -> u8;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005d6f3(param_1: i32) {
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's structure, read at the same offsets as the original.
    unsafe {
        let rd = |off: i32| -> i32 { (param_1.wrapping_add(off) as *const i32).read_unaligned() };
        let flags = (param_1.wrapping_add(0x440) as *const u16).read_unaligned();

        if DAT_100b3218 != 0 {
            DAT_100b32b4 = DAT_100b32b4.wrapping_add(1);
            FUN_1006b1f5(DAT_100b32b4.wrapping_mul(3));
            DAT_100b3218 = 0;
        }
        if DAT_100ab094 != 0 {
            if (flags & 0x80) == 0 {
                DAT_100b32ac = 0;
                if (flags & 0x40) == 0 {
                    DAT_100b32b0 = 0;
                } else if DAT_100b32b0 == 0 && rd(0x3d4) == 2 {
                    DAT_100b32b0 = 1;
                    FUN_10003742(0xe0, 100, 0x1f, 5, 0x32);
                }
            } else if DAT_100b32ac == 0 {
                let addr = rd(0x3e0)
                    .wrapping_mul(0x40)
                    .wrapping_add(8)
                    .wrapping_add(rd(0x54));
                if (addr as *const i32).read_unaligned() == 1 && rd(0x3d4) == 2 {
                    DAT_100b32ac = 1;
                    FUN_10003742(0x113, 100, 0x5f, 5, 0x32);
                }
            }
            if (rd(0x3d4) as u32) != DAT_100b32a8 {
                let i_var1 = rd(0x3d4);
                if i_var1 != 0 {
                    if i_var1 == 2 {
                        FUN_10003742(0xdf, 100, 0x2f, 5, 0x32);
                        DAT_100b32a8 = rd(0x3d4) as u32;
                        return;
                    }
                    if i_var1 != 4 {
                        DAT_100b32a8 = rd(0x3d4) as u32;
                        return;
                    }
                }
                FUN_10003742(0x10b, 100, 0x40, 5, 0x32);
            }
        }
        DAT_100b32a8 = rd(0x3d4) as u32;
    }
}
