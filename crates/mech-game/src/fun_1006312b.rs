// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba8: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006312b(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("1006312b", param_1, param_2);
    if param_2 < 0 || 2 < param_2 {
        return mech_trace::rt!("1006312b", 0);
    }
    // SAFETY: param_1 points at the game's own structure, whose fields and
    // the table at +0x54 are read where the original reads them; the global
    // is the game's own table.
    unsafe {
        let base = param_1 as usize;
        let cnt = mech_trace::rd!("1006312b", ((base + 0x3dc) as *const i32), read_unaligned);
        if cnt < 1 {
            return mech_trace::rt!("1006312b", 0);
        }
        let tbl = mech_trace::rd!("1006312b", ((base + 0x54) as *const i32), read_unaligned);
        let sel = mech_trace::rd!("1006312b", ((base + 0x3e0) as *const i32), read_unaligned);
        let v = mech_trace::rd!(
            "1006312b",
            (sel.wrapping_mul(0x40).wrapping_add(0xc).wrapping_add(tbl) as usize as *const i32),
            read_unaligned
        );
        if v == param_2 {
            return mech_trace::rt!("1006312b", 1);
        }
        let mut i: i32 = 0;
        while i < 0xe {
            let tbl = mech_trace::rd!("1006312b", ((base + 0x54) as *const i32), read_unaligned);
            let off = i.wrapping_mul(0x40);
            let iv = mech_trace::rd!(
                "1006312b",
                (tbl.wrapping_add(4).wrapping_add(off) as usize as *const i32),
                read_unaligned
            );
            if -1 < iv {
                let tbl =
                    mech_trace::rd!("1006312b", ((base + 0x54) as *const i32), read_unaligned);
                if mech_trace::rd!(
                    "1006312b",
                    (tbl.wrapping_add(off) as usize as *const i32),
                    read_unaligned
                ) != -1
                {
                    let tbl =
                        mech_trace::rd!("1006312b", ((base + 0x54) as *const i32), read_unaligned);
                    if mech_trace::rd!(
                        "1006312b",
                        (tbl.wrapping_add(0xc).wrapping_add(off) as usize as *const i32),
                        read_unaligned
                    ) == param_2
                    {
                        let tbl = mech_trace::rd!(
                            "1006312b",
                            ((base + 0x54) as *const i32),
                            read_unaligned
                        );
                        if mech_trace::rd!(
                            "1006312b",
                            (tbl.wrapping_add(8).wrapping_add(off) as usize as *const i32),
                            read_unaligned
                        ) != -1
                        {
                            let b = mech_trace::rd!(
                                "1006312b",
                                (&raw mut DAT_100a7ba8)
                                    .cast::<u8>()
                                    .byte_add(iv.wrapping_mul(0x68) as isize as usize),
                                read
                            );
                            if (b & 4) != 0 {
                                mech_trace::wr!(
                                    "1006312b",
                                    ((base + 0x3e0) as *mut i32),
                                    write_unaligned,
                                    i
                                );
                                return mech_trace::rt!("1006312b", 1);
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    mech_trace::rt!("1006312b", 0)
}
