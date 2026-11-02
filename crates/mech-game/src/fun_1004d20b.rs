// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: u32;
    static mut DAT_101600e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d34a(param_1: u32, param_2: *mut i32, param_3: *mut i32, param_4: *mut u32) -> u32;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d20b(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> u32 {
    mech_trace::enter!("1004d20b", param_1, param_2, param_3, param_4, param_5);
    let mut local_1c: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_18: u32 = 0;
    // SAFETY: the globals and callees are the game's own memory; the table
    // entries are pointers to records the game keeps live, accessed at the
    // same offsets as the original.
    unsafe {
        if param_1 < 0x10 {
            let local_10 = mech_trace::rd!(
                "1004d20b",
                (&raw mut DAT_101600e0)
                    .byte_offset((param_1.wrapping_mul(0x3c)) as isize)
                    .cast::<u32>(),
                read_unaligned
            );
            if local_10 < mech_trace::gr!("1004d20b", "100b38d8", DAT_100b38d8) {
                let table = &raw mut DAT_100eef40;
                let i_var1 =
                    mech_trace::rd!("1004d20b", table.add(local_10 as usize), read_unaligned)
                        as i32;
                let rec = i_var1 as u32 as *mut u8;
                mech_trace::wr!(
                    "1004d20b",
                    rec.byte_add(0x50).cast::<u32>(),
                    write_unaligned,
                    param_2
                );
                mech_trace::wr!(
                    "1004d20b",
                    rec.byte_add(0x54).cast::<u32>(),
                    write_unaligned,
                    param_3
                );
                mech_trace::wr!(
                    "1004d20b",
                    rec.byte_add(0x58).cast::<u32>(),
                    write_unaligned,
                    param_4
                );
                mech_trace::wr!(
                    "1004d20b",
                    rec.byte_add(0x60).cast::<u32>(),
                    write_unaligned,
                    param_5
                );
                let mut local_20: u32 = 0;
                while local_20 < mech_trace::gr!("1004d20b", "100b38d8", DAT_100b38d8) {
                    let local_14 =
                        mech_trace::rd!("1004d20b", table.add(local_20 as usize), read_unaligned)
                            as i32;
                    let e = local_14 as u32 as *mut u8;
                    if mech_trace::rd!("1004d20b", e.byte_add(8).cast::<i32>(), read_unaligned)
                        == param_1
                    {
                        mech_trace::cr!(
                            "1004d20b",
                            "1004d34a",
                            FUN_1004d34a(
                                local_20,
                                &raw mut local_1c,
                                &raw mut local_c,
                                &raw mut local_18,
                            )
                        );
                        let h = mech_trace::rd!(
                            "1004d20b",
                            e.byte_add(0x40).cast::<i32>(),
                            read_unaligned
                        );
                        FUN_1002f37a(h, local_1c as u32, param_3, local_c as u32);
                        let h = mech_trace::rd!(
                            "1004d20b",
                            e.byte_add(0x40).cast::<i32>(),
                            read_unaligned
                        );
                        FUN_1002f668(h, 0, local_18 as i32, 0, 0);
                        let h = mech_trace::rd!(
                            "1004d20b",
                            e.byte_add(0x40).cast::<u32>(),
                            read_unaligned
                        );
                        FUN_1002fbc0(h as *mut i32);
                        mech_trace::wr!(
                            "1004d20b",
                            e.byte_add(0x50).cast::<i32>(),
                            write_unaligned,
                            local_1c
                        );
                        mech_trace::wr!(
                            "1004d20b",
                            e.byte_add(0x54).cast::<u32>(),
                            write_unaligned,
                            param_3
                        );
                        mech_trace::wr!(
                            "1004d20b",
                            e.byte_add(0x58).cast::<i32>(),
                            write_unaligned,
                            local_c
                        );
                        mech_trace::wr!(
                            "1004d20b",
                            e.byte_add(0x60).cast::<u32>(),
                            write_unaligned,
                            local_18
                        );
                    }
                    local_20 = local_20.wrapping_add(1);
                }
                return mech_trace::rt!("1004d20b", 1);
            }
        }
    }
    mech_trace::rt!("1004d20b", 0)
}
