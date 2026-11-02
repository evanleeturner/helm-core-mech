// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab324: u32;
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10005624(
        param_1: i32,
        param_2: *mut i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
    ) -> u8;
    fn FUN_100082c4(param_1: i32, param_2: i32) -> u8;
    fn FUN_10004b19(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100044c6(param_1: i32, param_2: *mut i32) {
    mech_trace::enter!("100044c6", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; param_2 and the
    // structures it points to are the caller's memory, read where the original does.
    unsafe {
        if mech_trace::gr!("100044c6", "100ab324", DAT_100ab324) != 0 {
            let p17 = mech_trace::rd!("100044c6", param_2.add(0x17), read_unaligned);
            let p18 = mech_trace::rd!("100044c6", param_2.add(0x18), read_unaligned);
            if p17 != 0 && p18 != 0 {
                let mut local_c: i32 = 0;
                while local_c < 8 {
                    let i_var1 = mech_trace::rd!("100044c6", param_2.add(0x16), read_unaligned)
                        .wrapping_add(local_c.wrapping_mul(0x28));
                    let flags = mech_trace::rd!(
                        "100044c6",
                        ((i_var1.wrapping_add(0x26)) as *const u16),
                        read_unaligned
                    );
                    if flags & 0x2000 == 0 {
                        let mut local_10: i32 = 0;
                        while local_10
                            < mech_trace::rd!(
                                "100044c6",
                                ((i_var1.wrapping_add(0x24)) as *const i16),
                                read_unaligned
                            ) as i32
                        {
                            let v = mech_trace::rd!(
                                "100044c6",
                                (i_var1
                                    .wrapping_add(0xc)
                                    .wrapping_add(local_10.wrapping_mul(2))
                                    as *const u16),
                                read_unaligned
                            );
                            if 10000 < v {
                                let mut local_1c =
                                    mech_trace::rd!("100044c6", param_2.add(0x17), read_unaligned);
                                let mut local_14: i32 = 0;
                                while local_14
                                    < mech_trace::rd!("100044c6", param_2.add(0xff), read_unaligned)
                                {
                                    let v2 = mech_trace::rd!(
                                        "100044c6",
                                        (i_var1
                                            .wrapping_add(0xc)
                                            .wrapping_add(local_10.wrapping_mul(2))
                                            as *const u16),
                                        read_unaligned
                                    );
                                    let s = mech_trace::rd!(
                                        "100044c6",
                                        ((local_1c.wrapping_add(4)) as *const i16),
                                        read_unaligned
                                    );
                                    if u32::from(v2) == (s as i32) as u32 {
                                        let idx = mech_trace::rd!(
                                            "100044c6",
                                            ((local_1c.wrapping_add(0x12)) as *const i32),
                                            read_unaligned
                                        );
                                        if idx
                                            < mech_trace::rd!(
                                                "100044c6",
                                                param_2.add(0x18),
                                                read_unaligned
                                            )
                                        {
                                            let addr = (param_2 as i32)
                                                .wrapping_add(idx.wrapping_mul(0x3a))
                                                .wrapping_add(0x66);
                                            if 0 < mech_trace::rd!(
                                                "100044c6",
                                                (addr as *const i32),
                                                read_unaligned
                                            ) {
                                                mech_trace::cr!(
                                                    "100044c6",
                                                    "10005624",
                                                    FUN_10005624(
                                                        param_1,
                                                        param_2,
                                                        local_c.wrapping_add(1) as u32,
                                                        local_10,
                                                        0,
                                                    )
                                                );
                                                return;
                                            }
                                        }
                                    }
                                    local_1c = local_1c.wrapping_add(0x1e);
                                    local_14 = local_14.wrapping_add(1);
                                }
                            }
                            local_10 = local_10.wrapping_add(1);
                        }
                    }
                    local_c = local_c.wrapping_add(1);
                }
            }
            let base = mech_trace::rd!("100044c6", param_2, read_unaligned);
            if mech_trace::rd!(
                "100044c6",
                ((base.wrapping_add(4)) as *const u32),
                read_unaligned
            ) == mech_trace::gr!("100044c6", "100adf38", DAT_100adf38)
            {
                mech_trace::cr!("100044c6", "100082c4", FUN_100082c4(0, -1));
            }
            mech_trace::cr!("100044c6", "10004b19", FUN_10004b19(param_1, param_2));
        }
    }
}
