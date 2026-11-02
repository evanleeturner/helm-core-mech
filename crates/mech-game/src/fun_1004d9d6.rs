// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: u32;
    static mut DAT_100ef030: u8;
    static mut DAT_100ef032: u8;
    static mut DAT_10215694: u8;
    static mut DAT_10215696: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d9d6(param_1: *mut u8, param_2: u8) -> u32 {
    mech_trace::enter!("1004d9d6", param_1, param_2);
    // SAFETY: the globals are the game's own tables; param_1 points at the
    // caller's two bytes, and table entries are read where the original does.
    unsafe {
        let local_8: u32 = mech_trace::rd!("1004d9d6", param_1, read) as u32;
        let b_var1: u8 = mech_trace::rd!("1004d9d6", param_1.add(1), read);
        let mut local_10: u32 = 0;
        let bit: i32 = 1i32.wrapping_shl((param_2 & 0x1f) as u32);
        if b_var1 == 1 {
            let off = local_8.wrapping_mul(0x54) as usize;
            let f = mech_trace::rd!(
                "1004d9d6",
                (&raw mut DAT_10215694).byte_add(off).cast::<u16>(),
                read_unaligned
            );
            if (f & 0x20) != 0 {
                let s = mech_trace::rd!(
                    "1004d9d6",
                    (&raw mut DAT_10215696).byte_add(off).cast::<i16>(),
                    read_unaligned
                );
                if (bit & (s as i32)) != 0 {
                    local_10 = 1;
                }
            }
        } else if b_var1 == 2 {
            let p = mech_trace::rd!(
                "1004d9d6",
                (&raw mut DAT_100eef40).add(local_8 as usize),
                read_unaligned
            );
            let f = mech_trace::rd!(
                "1004d9d6",
                ((p.wrapping_add(0x14)) as *const u16),
                read_unaligned
            );
            if (f & 0x20) != 0 {
                let s = mech_trace::rd!(
                    "1004d9d6",
                    ((p.wrapping_add(0x16)) as *const i16),
                    read_unaligned
                );
                if (bit & (s as i32)) != 0 {
                    local_10 = 1;
                }
            }
        } else if b_var1 == 4 {
            let off = local_8.wrapping_mul(0x40) as usize;
            let f = mech_trace::rd!(
                "1004d9d6",
                (&raw mut DAT_100ef030).byte_add(off).cast::<u16>(),
                read_unaligned
            );
            if (f & 0x20) != 0 {
                let s = mech_trace::rd!(
                    "1004d9d6",
                    (&raw mut DAT_100ef032).byte_add(off).cast::<i16>(),
                    read_unaligned
                );
                if (bit & (s as i32)) != 0 {
                    local_10 = 1;
                }
            }
        }
        mech_trace::rt!("1004d9d6", local_10)
    }
}
