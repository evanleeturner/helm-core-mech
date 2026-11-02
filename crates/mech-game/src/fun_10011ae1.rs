// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9f20: u32;
    static mut DAT_100a9f24: u32;
    static mut DAT_100a9f28: u32;
    static mut DAT_100a9f2c: u32;
    static mut DAT_100a9f30: u32;
    static mut DAT_100a9f34: u32;
    static mut DAT_100a9f38: u32;
    static mut DAT_100a9f40: u32;
    static mut DAT_100a9f48: u8;
    static mut DAT_100a9f4c: u8;
    static mut DAT_100a9f50: u8;
    static mut DAT_100a9f54: u8;
    static mut DAT_100af17c: u32;
    static mut DAT_100c6988: u8;
    static mut DAT_100c698c: u8;
    static mut DAT_100c69c8: u8;
    static mut DAT_100c69cc: u8;
    static mut DAT_100c6b08: u32;
    static mut DAT_100c6b0c: u32;
    static mut DAT_100eebbe: u8;
    static mut DAT_101686f0: u32;
    static mut PTR_DAT_100af0b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10016b61(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
}

unsafe fn rd(addr: usize) -> i32 {
    // SAFETY: caller passes an address inside the game's own tables.
    unsafe { (addr as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011ae1(param_1: i32, param_2: *mut i32) {
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's structure, and the tables are read where the original reads.
    unsafe {
        DAT_100eebbe = 6;
        let puvar2 = FUN_1008668b(
            DAT_100af17c as i32,
            DAT_101686f0.wrapping_add(1),
            PTR_DAT_100af0b8 as *mut i32,
        );
        if !puvar2.is_null() {
            FUN_1005acaa(
                param_2,
                DAT_100a9f20 as i32,
                DAT_100a9f24 as i32,
                puvar2 as i32,
                &raw mut DAT_100a9f48,
                0x100eebb0,
            );
            FUN_1005acaa(
                param_2,
                DAT_100a9f28 as i32,
                DAT_100a9f2c as i32,
                puvar2 as i32,
                &raw mut DAT_100a9f4c,
                0x100eebb0,
            );
            FUN_1005acaa(
                param_2,
                DAT_100a9f30 as i32,
                DAT_100a9f34 as i32,
                puvar2 as i32,
                &raw mut DAT_100a9f50,
                0x100eebb0,
            );
            FUN_1005acaa(
                param_2,
                DAT_100a9f38 as i32,
                DAT_100a9f2c as i32,
                puvar2 as i32,
                &raw mut DAT_100a9f54,
                0x100eebb0,
            );
            DAT_100eebbe = 0xe;
            FUN_1008664f(DAT_101686f0.wrapping_add(1), PTR_DAT_100af0b8 as *mut i32);
        }
        let a6988 = (&raw mut DAT_100c6988) as usize;
        let a698c = (&raw mut DAT_100c698c) as usize;
        let a69c8 = (&raw mut DAT_100c69c8) as usize;
        let a69cc = (&raw mut DAT_100c69cc) as usize;

        let mut local_c: i32 = 0;
        while local_c < 8 {
            let off = (local_c as usize).wrapping_mul(8);
            let pivar1 = (rd((param_1 as usize).wrapping_add(0x58)) as usize)
                .wrapping_add((local_c as usize).wrapping_mul(0x28));
            let mut local_1c = DAT_100c6b08 as i32;
            if 0 < local_c && local_c < 4 {
                local_1c /= 2;
            }
            let flags = (pivar1.wrapping_add(0x26) as *const u16).read_unaligned();
            let local_18: i32;
            let mut local_10: i32;
            if flags & 0x2000 == 0 {
                if (rd(a69c8.wrapping_add(off)) >> 2) < rd(pivar1) {
                    local_10 = 3;
                } else {
                    local_10 = 0xb;
                }
                local_18 = FUN_10011e60(rd(pivar1), DAT_100c6b0c as i32, DAT_100a9f40 as i32);
            } else {
                local_18 = 0;
                local_10 = 0xf3;
            }
            let i_var3 = FUN_10011e60(
                rd(a69c8.wrapping_add(off)),
                DAT_100c6b0c as i32,
                DAT_100a9f40 as i32,
            );
            if local_18 != 0 {
                FUN_10016b61(
                    param_2,
                    rd(a6988.wrapping_add(off)),
                    rd(a698c.wrapping_add(off)).wrapping_add(local_18),
                    local_1c,
                    local_18,
                    0xf,
                );
            }
            if local_18 < i_var3 {
                FUN_10016b61(
                    param_2,
                    rd(a6988.wrapping_add(off)),
                    rd(a698c.wrapping_add(off)).wrapping_add(i_var3),
                    local_1c,
                    i_var3.wrapping_sub(local_18),
                    local_10,
                );
            }
            if 0 < local_c && local_c < 4 {
                let local_18b: i32;
                if flags & 0x2000 == 0 {
                    if (rd(a69cc.wrapping_add(off)) >> 2) < rd(pivar1.wrapping_add(4)) {
                        local_10 = 3;
                    } else {
                        local_10 = 0xb;
                    }
                    local_18b = FUN_10011e60(
                        rd(pivar1.wrapping_add(4)),
                        DAT_100c6b0c as i32,
                        DAT_100a9f40 as i32,
                    );
                } else {
                    local_18b = 0;
                    local_10 = 0xf3;
                }
                let i_var3b = FUN_10011e60(
                    rd(a69cc.wrapping_add(off)),
                    DAT_100c6b0c as i32,
                    DAT_100a9f40 as i32,
                );
                if local_18b != 0 {
                    FUN_10016b61(
                        param_2,
                        rd(a6988.wrapping_add(off)).wrapping_add(local_1c),
                        rd(a698c.wrapping_add(off)).wrapping_add(local_18b),
                        local_1c,
                        local_18b,
                        0xf,
                    );
                }
                if local_18b < i_var3b {
                    FUN_10016b61(
                        param_2,
                        rd(a6988.wrapping_add(off)).wrapping_add(local_1c),
                        rd(a698c.wrapping_add(off)).wrapping_add(i_var3b),
                        local_1c,
                        i_var3b.wrapping_sub(local_18b),
                        local_10,
                    );
                }
            }
            local_c += 1;
        }
    }
}
