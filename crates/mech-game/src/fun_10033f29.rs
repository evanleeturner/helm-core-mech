// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bb0: u8;
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bec: u8;
    static mut DAT_100a7bf0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10034b8e(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10033f29(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let mut local_28: u32 = 0;
    let mut local_8: i32 = 0;
    let mut local_2c: i32 = 0;
    let local_10: i32;
    // SAFETY: param_1 and the game's tables are the game's own memory; every
    // read happens at the offsets and in the order the original uses.
    unsafe {
        let local_1c = param_1
            .add(0xf8)
            .read()
            .wrapping_mul(0x40)
            .wrapping_add(param_1.add(0x15).read());
        let i_var1 = ((local_1c.wrapping_add(4)) as *const i32)
            .read_unaligned()
            .wrapping_mul(0x68);
        if ((local_1c.wrapping_add(8)) as *const i32).read_unaligned() == 1 {
            let a = (&raw mut DAT_100a7bb0 as usize).wrapping_add(i_var1 as usize) as *const i32;
            let local_18: i32;
            let local_14: i32;
            if a.read_unaligned() == 3 {
                local_18 = 0x4f;
                local_14 = 0x52;
            } else {
                local_14 = 0x5b;
                local_18 = 0x58;
            }
            let b = (&raw mut DAT_100a7bc8 as usize).wrapping_add(i_var1 as usize) as *const i32;
            if b.read_unaligned() == 0 {
                let g = param_1.read() as u32 as usize;
                let c =
                    (&raw mut DAT_100a7bec as usize).wrapping_add(i_var1 as usize) as *const i32;
                let d =
                    (&raw mut DAT_100a7bf0 as usize).wrapping_add(i_var1 as usize) as *const i32;
                if ((g.wrapping_add(0xdc)) as *const i32).read_unaligned() == 0
                    || (((g.wrapping_add(0xdd)) as *const u8).read() & 0x11) != 0
                    || param_4 <= c.read_unaligned()
                    || d.read_unaligned() <= param_4
                    || 0x2ffff < param_3
                    || param_3 < -0x2ffff
                    || 2 < param_2
                    || param_2 < -2
                {
                    local_10 = local_14;
                } else {
                    local_28 = 1;
                    local_10 = local_18;
                }
            } else {
                let w = param_1.add(0x110).cast::<u16>().read_unaligned();
                if (w & 0x80) == 0 {
                    if (w & 0x8000) == 0 {
                        local_10 = 0x52;
                    } else {
                        local_10 = 0x55;
                    }
                } else {
                    local_10 = 0x46;
                }
            }
        } else {
            local_10 = 0x4c;
        }
        let r = FUN_10034b8e(param_1, &raw mut local_8, &raw mut local_2c);
        if r != 0 {
            FUN_1003510c(local_8, local_2c, local_10);
        }
    }
    local_28
}
