// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba8: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006312b(param_1: i32, param_2: i32) -> u32 {
    if param_2 < 0 || 2 < param_2 {
        return 0;
    }
    // SAFETY: param_1 points at the game's own structure, whose fields and
    // the table at +0x54 are read where the original reads them; the global
    // is the game's own table.
    unsafe {
        let base = param_1 as usize;
        let cnt = ((base + 0x3dc) as *const i32).read_unaligned();
        if cnt < 1 {
            return 0;
        }
        let tbl = ((base + 0x54) as *const i32).read_unaligned();
        let sel = ((base + 0x3e0) as *const i32).read_unaligned();
        let v = (sel.wrapping_mul(0x40).wrapping_add(0xc).wrapping_add(tbl) as usize as *const i32)
            .read_unaligned();
        if v == param_2 {
            return 1;
        }
        let mut i: i32 = 0;
        while i < 0xe {
            let tbl = ((base + 0x54) as *const i32).read_unaligned();
            let off = i.wrapping_mul(0x40);
            let iv =
                (tbl.wrapping_add(4).wrapping_add(off) as usize as *const i32).read_unaligned();
            if -1 < iv {
                let tbl = ((base + 0x54) as *const i32).read_unaligned();
                if (tbl.wrapping_add(off) as usize as *const i32).read_unaligned() != -1 {
                    let tbl = ((base + 0x54) as *const i32).read_unaligned();
                    if (tbl.wrapping_add(0xc).wrapping_add(off) as usize as *const i32)
                        .read_unaligned()
                        == param_2
                    {
                        let tbl = ((base + 0x54) as *const i32).read_unaligned();
                        if (tbl.wrapping_add(8).wrapping_add(off) as usize as *const i32)
                            .read_unaligned()
                            != -1
                        {
                            let b = (&raw mut DAT_100a7ba8)
                                .cast::<u8>()
                                .byte_add(iv.wrapping_mul(0x68) as isize as usize)
                                .read();
                            if (b & 4) != 0 {
                                ((base + 0x3e0) as *mut i32).write_unaligned(i);
                                return 1;
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    0
}
