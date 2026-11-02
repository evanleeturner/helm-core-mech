// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eee64: i32;
    static mut DAT_1016a6c8: i16;
    static mut DAT_100ad0a8: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut PTR_FUN_100ad340: [Option<unsafe extern "C" fn(*mut i32, i16)>; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10028a85();
    fn FUN_1004d47f(param_1: i32) -> i32;
    fn FUN_1002bd38(param_1: i32);
    fn FUN_1004d5e3(param_1: i32) -> i32;
    fn FUN_1002a9bb(param_1: i32, param_2: u32);
    fn FUN_10023258(param_1: i32, param_2: *mut u32);
    fn FUN_10027088(param_1: i32) -> i32;
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_10027c05(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10026f0b(param_1: *mut i32) {
    mech_trace::enter!("10026f0b", param_1);
    let mut local_8: u32 = 0;
    // SAFETY: param_1 points at the game's object; the globals, tables and
    // callees are the game's own, accessed in the original's order.
    unsafe {
        if mech_trace::rd!("10026f0b", param_1.add(4), read) != 1 {
            if mech_trace::rd!("10026f0b", param_1.add(1), read)
                == mech_trace::gr!("10026f0b", "100adf38", DAT_100adf38)
                && mech_trace::gr!("10026f0b", "100eee64", DAT_100eee64) != 0
            {
                FUN_10028a85();
            }
            if mech_trace::rd!("10026f0b", param_1.add(1), read) == 0 {
                mech_trace::gw!("10026f0b", "1016a6c8", DAT_1016a6c8, 0);
            }
            if mech_trace::rd!(
                "10026f0b",
                param_1.byte_add(0x176).cast::<i16>(),
                read_unaligned
            ) != 0xc
            {
                let i_var1 = mech_trace::cr!(
                    "10026f0b",
                    "1004d47f",
                    FUN_1004d47f(mech_trace::rd!("10026f0b", param_1.add(2), read))
                );
                if i_var1 == mech_trace::rd!("10026f0b", param_1.add(1), read) {
                    FUN_1002bd38(mech_trace::rd!("10026f0b", param_1.add(2), read));
                }
                if mech_trace::rd!("10026f0b", param_1.add(4), read) == 2 {
                    let i_var1 =
                        mech_trace::cr!("10026f0b", "10027088", FUN_10027088(param_1 as i32));
                    if i_var1 == 0 {
                        let idx = mech_trace::rd!(
                            "10026f0b",
                            param_1.byte_add(0x176).cast::<i16>(),
                            read_unaligned
                        );
                        let f = mech_trace::rd!(
                            "10026f0b",
                            (&raw mut PTR_FUN_100ad340)
                                .cast::<Option<unsafe extern "C" fn(*mut i32, i16)>>()
                                .offset(idx as isize),
                            read
                        );
                        let arg = mech_trace::rd!("10026f0b", param_1.add(0x5e), read) as i16;
                        if let Some(f) = f {
                            mech_trace::cr!("10026f0b", "ind", f(param_1, arg));
                        }
                    }
                    FUN_1002950f(
                        param_1 as i32,
                        mech_trace::rd!(
                            "10026f0b",
                            param_1.byte_add(0x17a).cast::<i16>(),
                            read_unaligned
                        ),
                    );
                    FUN_10027c05(param_1);
                } else {
                    let p8 = mech_trace::rd!("10026f0b", param_1.add(8), read);
                    if mech_trace::rd!(
                        "10026f0b",
                        ((p8 as u32).wrapping_add(0x3d8) as *const i32),
                        read_unaligned
                    ) != 0
                        && mech_trace::rd!("10026f0b", param_1.add(0x1f), read) != -1
                    {
                        let i_var1 = mech_trace::cr!(
                            "10026f0b",
                            "1004d5e3",
                            FUN_1004d5e3(mech_trace::rd!("10026f0b", param_1.add(0x1f), read))
                        );
                        if i_var1 == 1 {
                            let base = (&raw mut DAT_100eef40).cast::<i32>();
                            let ent = mech_trace::rd!(
                                "10026f0b",
                                base.offset(
                                    mech_trace::rd!("10026f0b", param_1.add(0x1f), read) as isize
                                ),
                                read
                            );
                            let v = mech_trace::rd!(
                                "10026f0b",
                                ((ent as u32).wrapping_add(4) as *const u32),
                                read_unaligned
                            );
                            FUN_1002a9bb(mech_trace::rd!("10026f0b", param_1.add(1), read), v);
                        }
                    }
                    if mech_trace::gr!("10026f0b", "100ad0a8", DAT_100ad0a8) != 0 {
                        FUN_10023258(param_1 as i32, &raw mut local_8);
                    }
                }
            }
        }
    }
}
