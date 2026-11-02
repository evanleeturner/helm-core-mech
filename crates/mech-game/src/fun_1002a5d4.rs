// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f60: u32;
    static mut DAT_10215678: u8;
    static mut DAT_10215694: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008255f(param_1: u32, param_2: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a5d4(param_1: i32) {
    // SAFETY: the globals and the callee are the game's own; param_1 points
    // at a live record and the tables are indexed as the original does.
    unsafe {
        if DAT_100b9f60 != 0xFFFF_FFFF {
            let mut local_8: u32 = 0;
            while (local_8 as i32) < (DAT_100b9f60 as i32) {
                let off = local_8.wrapping_mul(0x54) as usize;
                let flags = (&raw mut DAT_10215694)
                    .byte_add(off)
                    .cast::<u16>()
                    .read_unaligned();
                if (flags & 1) != 0 {
                    let key = (&raw mut DAT_10215678)
                        .byte_add(off)
                        .cast::<u32>()
                        .read_unaligned();
                    let id = ((param_1 as u32).wrapping_add(4) as *const u32).read_unaligned();
                    if key == (id | 0x200) {
                        let b = ((param_1 as u32).wrapping_add(0x1a3) as *const u8).read();
                        if (b & 0x10) == 0 {
                            let w = ((param_1 as u32).wrapping_add(0x1a2) as *const u32)
                                .read_unaligned();
                            if (w & 0xff) != local_8 {
                                let id2 = ((param_1 as u32).wrapping_add(4) as *const u32)
                                    .read_unaligned();
                                FUN_1008255f(id2, local_8 | 0x100);
                            }
                        } else {
                            let id2 =
                                ((param_1 as u32).wrapping_add(4) as *const u32).read_unaligned();
                            FUN_1008255f(id2, local_8 | 0x100);
                        }
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
        }
    }
}
