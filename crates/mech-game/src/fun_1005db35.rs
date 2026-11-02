// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b32b8: *mut i32;
    static mut DAT_100b38fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d9bf(param_1: *mut i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005db35(param_1: *mut i32) {
    mech_trace::enter!("1005db35", param_1);
    // SAFETY: the globals and callee are the game's own; param_1 and the
    // list nodes it links are valid game memory, accessed as the original does.
    unsafe {
        mech_trace::gw!(
            "1005db35",
            "100b32b8",
            DAT_100b32b8,
            mech_trace::rd!("1005db35", param_1, read_unaligned) as *mut i32
        );
        while !mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).is_null() {
            if mech_trace::rd!(
                "1005db35",
                mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8),
                read_unaligned
            ) != 0
                && mech_trace::rd!(
                    "1005db35",
                    mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).add(4),
                    read_unaligned
                ) <= mech_trace::gr!("1005db35", "100b38fc", DAT_100b38fc)
            {
                mech_trace::wr!(
                    "1005db35",
                    mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).add(4),
                    write_unaligned,
                    mech_trace::rd!(
                        "1005db35",
                        mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).add(2),
                        read_unaligned
                    )
                    .wrapping_add(mech_trace::gr!(
                        "1005db35",
                        "100b38fc",
                        DAT_100b38fc
                    ))
                );
                let f: extern "C" fn(i32, i32, i32, i32) -> i32 =
                    core::mem::transmute(mech_trace::rd!(
                        "1005db35",
                        mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8),
                        read_unaligned
                    ) as usize);
                let i_var1 = mech_trace::cr!(
                    "1005db35",
                    "ind",
                    f(
                        1,
                        0,
                        mech_trace::gr!("1005db35", "100b38fc", DAT_100b38fc),
                        mech_trace::rd!(
                            "1005db35",
                            mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).add(2),
                            read_unaligned
                        )
                    )
                );
                if i_var1 == 0 {
                    FUN_1005d9bf(
                        param_1,
                        mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8),
                    );
                } else {
                    mech_trace::wr!(
                        "1005db35",
                        mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).add(3),
                        write_unaligned,
                        mech_trace::gr!("1005db35", "100b38fc", DAT_100b38fc)
                    );
                }
            }
            mech_trace::gw!(
                "1005db35",
                "100b32b8",
                DAT_100b32b8,
                mech_trace::rd!(
                    "1005db35",
                    mech_trace::gr!("1005db35", "100b32b8", DAT_100b32b8).add(5),
                    read_unaligned
                ) as *mut i32
            );
        }
    }
}
