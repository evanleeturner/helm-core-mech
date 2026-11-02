// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef030: u8;
    static mut DAT_100ef038: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004aa97(
        param_1: u32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: i32,
    ) -> u8;
    fn FUN_1004bbc3(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004bd53(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "1004bd53", param_1, param_2, param_3, param_4, param_5, param_6
    );
    if param_2 != 0 {
        // SAFETY: param_2 points at the caller's record; the globals are the
        // game's own tables, indexed by the record's u16 field times 0x40,
        // read and written where the original does.
        unsafe {
            let p2 = param_2 as *const u8;
            let u_var1 =
                mech_trace::rd!("1004bd53", p2.byte_add(0x14).cast::<u16>(), read_unaligned);
            let i_var2 = (u_var1 as u32).wrapping_mul(0x40) as usize;
            let flags = (&raw mut DAT_100ef030).byte_add(i_var2).cast::<u16>();
            let cnt = (&raw mut DAT_100ef038).byte_add(i_var2).cast::<i32>();
            if (mech_trace::rd!("1004bd53", flags, read_unaligned) & 4) == 0
                && mech_trace::rd!("1004bd53", cnt, read_unaligned) != 0
            {
                let kind =
                    (mech_trace::rd!("1004bd53", p2.byte_add(2).cast::<u16>(), read_unaligned)
                        as u8)
                        & 0xf0;
                if kind == 0xb0 {
                    mech_trace::wr!("1004bd53", cnt, write_unaligned, 0);
                } else {
                    mech_trace::wr!(
                        "1004bd53",
                        cnt,
                        write_unaligned,
                        mech_trace::rd!("1004bd53", cnt, read_unaligned).wrapping_sub(param_3)
                    );
                }
                if mech_trace::rd!("1004bd53", cnt, read_unaligned) < 1 {
                    mech_trace::wr!("1004bd53", cnt, write_unaligned, 0);
                    let kind =
                        (mech_trace::rd!("1004bd53", p2.byte_add(2).cast::<u16>(), read_unaligned)
                            as u8)
                            & 0xf0;
                    if kind == 0xb0 {
                        mech_trace::cr!(
                            "1004bd53",
                            "1004aa97",
                            FUN_1004aa97(
                                param_1,
                                0xd,
                                param_4,
                                param_5,
                                param_6,
                                param_4,
                                param_5 as u32,
                                param_6,
                            )
                        );
                    } else {
                        mech_trace::cr!(
                            "1004bd53",
                            "1004aa97",
                            FUN_1004aa97(
                                param_1,
                                3,
                                param_4,
                                param_5,
                                param_6,
                                param_4,
                                param_5 as u32,
                                param_6,
                            )
                        );
                        mech_trace::cr!(
                            "1004bd53",
                            "1004aa97",
                            FUN_1004aa97(
                                param_1,
                                0x20b,
                                param_4,
                                param_5,
                                param_6,
                                param_4,
                                param_5 as u32,
                                param_6,
                            )
                        );
                    }
                    mech_trace::cr!("1004bd53", "1004bbc3", FUN_1004bbc3(u_var1 as u32));
                }
            }
        }
    }
}
