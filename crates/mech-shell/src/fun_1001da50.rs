// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100a67a4: u32;
    static mut DAT_100a67a8: u32;
    static mut DAT_100902b4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1005b1b0(param_1: *const i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001da50() -> i32 {
    mech_trace::enter!("1001da50");
    // SAFETY: the globals, the collection pointers and the items they yield
    // are the game's own memory, read at the offsets the original uses.
    unsafe {
        let mut i_var2: i32 = 0;
        let mut i_var3: i32 = 0;
        if 0 < mech_trace::rd!(
            "1001da50",
            ((mech_trace::gr!("1001da50", "100a67a4", DAT_100a67a4).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let i_var1 = mech_trace::cr!(
                    "1001da50",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1001da50", "100a67a4", DAT_100a67a4) as i32,
                        i_var3
                    )
                );
                if (mech_trace::rd!(
                    "1001da50",
                    ((i_var1.wrapping_add(0x20)) as *const i32),
                    read_unaligned
                ) != -1)
                    && (mech_trace::rd!(
                        "1001da50",
                        ((i_var1.wrapping_add(0x2064)) as *const i8),
                        read
                    ) == 0)
                {
                    i_var2 = i_var2.wrapping_add(1);
                }
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3
                    < mech_trace::rd!(
                        "1001da50",
                        ((mech_trace::gr!("1001da50", "100a67a4", DAT_100a67a4).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        if i_var2 < 3 {
            i_var3 = 0;
            if 0 < mech_trace::rd!(
                "1001da50",
                ((mech_trace::gr!("1001da50", "100a67a8", DAT_100a67a8).wrapping_add(4))
                    as *const i32),
                read_unaligned
            ) {
                loop {
                    let i_var1 = mech_trace::cr!(
                        "1001da50",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1001da50", "100a67a8", DAT_100a67a8) as i32,
                            i_var3
                        )
                    );
                    if mech_trace::rd!("1001da50", ((i_var1.wrapping_add(0x2c)) as *const i8), read)
                        == 1
                    {
                        i_var2 = i_var2.wrapping_add(1);
                    }
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3
                        < mech_trace::rd!(
                            "1001da50",
                            ((mech_trace::gr!("1001da50", "100a67a8", DAT_100a67a8).wrapping_add(4))
                                as *const i32),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
            return mech_trace::rt!("1001da50", i_var2);
        }
        FUN_1005b1b0((&raw mut DAT_100902b4) as *const i8);
        mech_trace::rt!(
            "1001da50",
            mech_trace::rd!(
                "1001da50",
                ((mech_trace::gr!("1001da50", "10083508", DAT_10083508).wrapping_add(0x2df7))
                    as *const i32),
                read_unaligned
            )
            .wrapping_add(1)
        )
    }
}
