// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082be8: i32;
    static mut DAT_10092000: i32;
    static mut DAT_10098224: i32;
    static mut DAT_10092010: i32;
    static mut DAT_100a67b8: i32;
    static mut DAT_10082bf0: i32;
    static mut DAT_10082c38: i32;
    static mut DAT_10082c2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> i32;
    fn FUN_10031890(param_1: i32) -> i32;
    fn FUN_10031a00(param_1: i32, param_2: i32);
    fn FUN_10031910(param_1: i32);
    fn FUN_1000a8c0(param_1: i32);
    fn FUN_1000aa80(param_1: i32);
    fn FUN_1000a890(param_1: i32);
    fn HeapAlloc(param_1: i32, param_2: u32, param_3: u32) -> *mut i32;
    fn FUN_10050390(param_1: *mut i32);
    fn FUN_1000c770(param_1: *mut i32, param_2: i32);
    fn FUN_10017df0(param_1: i32, param_2: *mut i32);
    fn FUN_100318e0(param_1: i32);
    fn FUN_100503d0(param_1: *mut i32);
    fn FUN_10031980(param_1: i32);
    fn FUN_10031950(param_1: i32);
    fn FUN_100319b0(param_1: i32);
    fn FUN_10050400(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100315b0(param_1: i32) {
    mech_trace::enter!("100315b0", param_1);
    if 0 < param_1 {
        // SAFETY: all callees and globals are the game's own; the allocated
        // record is written within its allocated size, as the original does.
        unsafe {
            let mut c_var1 = mech_trace::cr!("100315b0", "1000d230", FUN_1000d230(param_1));
            let mut i_var2 = mech_trace::cr!("100315b0", "1000d260", FUN_1000d260(c_var1, param_1));
            if c_var1 == 2 {
                let v = mech_trace::rd!(
                    "100315b0",
                    (&raw mut DAT_10082be8).byte_offset(i_var2.wrapping_mul(0xc) as isize),
                    read_unaligned
                );
                if v == 9000 {
                    FUN_10031a00(mech_trace::gr!("100315b0", "10092000", DAT_10092000), 9000);
                    return;
                }
                if v == 0x251c {
                    FUN_10031a00(
                        mech_trace::gr!("100315b0", "10092000", DAT_10092000),
                        0x251c,
                    );
                    return;
                }
            }
            i_var2 = mech_trace::cr!("100315b0", "10031890", FUN_10031890(param_1));
            if (-1 < param_1) && (i_var2 != 0) {
                c_var1 = mech_trace::cr!("100315b0", "1000d230", FUN_1000d230(param_1));
                i_var2 = c_var1;
                let i_var3 = mech_trace::cr!("100315b0", "1000d260", FUN_1000d260(i_var2, param_1));
                if i_var2 == 0 {
                    FUN_10031910(param_1);
                    FUN_1000a8c0(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    FUN_1000aa80(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    FUN_1000a890(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    let p = mech_trace::cr!(
                        "100315b0",
                        "HeapAlloc",
                        HeapAlloc(
                            mech_trace::gr!("100315b0", "10098224", DAT_10098224),
                            1,
                            0x1a
                        )
                    );
                    mech_trace::wr!("100315b0", p, write_unaligned, i_var3);
                    mech_trace::wr!("100315b0", p.byte_add(8).cast::<i8>(), write, 0);
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(10).cast::<i32>(),
                        write_unaligned,
                        -1
                    );
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(0xe).cast::<i32>(),
                        write_unaligned,
                        -1
                    );
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(0x12).cast::<i32>(),
                        write_unaligned,
                        100
                    );
                    FUN_10050390(p);
                    FUN_1000c770(p, 0);
                    FUN_10017df0(mech_trace::gr!("100315b0", "10092010", DAT_10092010), p);
                    return;
                }
                if i_var2 == 1 {
                    FUN_100318e0(param_1);
                    FUN_1000a8c0(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    FUN_1000aa80(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    FUN_1000a890(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    let p = mech_trace::cr!(
                        "100315b0",
                        "HeapAlloc",
                        HeapAlloc(
                            mech_trace::gr!("100315b0", "10098224", DAT_10098224),
                            1,
                            0x1a
                        )
                    );
                    mech_trace::wr!("100315b0", p, write_unaligned, i_var3);
                    mech_trace::wr!("100315b0", p.byte_add(8).cast::<i8>(), write, 1);
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(0xe).cast::<i32>(),
                        write_unaligned,
                        1
                    );
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(0x12).cast::<i32>(),
                        write_unaligned,
                        -1
                    );
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(0x16).cast::<i32>(),
                        write_unaligned,
                        100
                    );
                    FUN_100503d0(p);
                    FUN_10017df0(mech_trace::gr!("100315b0", "100a67b8", DAT_100a67b8), p);
                    FUN_10017df0(mech_trace::gr!("100315b0", "10092010", DAT_10092010), p);
                    return;
                }
                if i_var2 != 2 {
                    return;
                }
                i_var2 = mech_trace::rd!(
                    "100315b0",
                    (&raw mut DAT_10082be8).byte_offset(i_var3.wrapping_mul(0xc) as isize),
                    read_unaligned
                );
                let p = mech_trace::cr!(
                    "100315b0",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("100315b0", "10098224", DAT_10098224),
                        1,
                        0x16
                    )
                );
                if i_var2 < 0x1f41 {
                    if i_var2 == 8000 {
                        let v = mech_trace::rd!(
                            "100315b0",
                            ((mech_trace::gr!("100315b0", "10092000", DAT_10092000)
                                .wrapping_add(0x228)) as *const i32),
                            read_unaligned
                        );
                        mech_trace::wr!("100315b0", p, write_unaligned, 1);
                        if v != 0 {
                            mech_trace::wr!("100315b0", p, write_unaligned, 2);
                        }
                        let idx = mech_trace::rd!("100315b0", p, read_unaligned);
                        mech_trace::wr!(
                            "100315b0",
                            p.byte_add(4),
                            write_unaligned,
                            mech_trace::rd!(
                                "100315b0",
                                (&raw mut DAT_10082bf0).byte_offset(idx.wrapping_mul(0xc) as isize),
                                read_unaligned
                            )
                        );
                        FUN_10031980(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                    } else {
                        if i_var2 != 7000 {
                            return;
                        }
                        mech_trace::wr!("100315b0", p, write_unaligned, 10);
                    }
                } else if i_var2 == 0x2134 {
                    mech_trace::wr!("100315b0", p, write_unaligned, 6);
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(4),
                        write_unaligned,
                        mech_trace::gr!("100315b0", "10082c38", DAT_10082c38)
                    );
                    FUN_10031950(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                } else {
                    if i_var2 != 0x254e {
                        return;
                    }
                    mech_trace::wr!("100315b0", p, write_unaligned, 5);
                    mech_trace::wr!(
                        "100315b0",
                        p.byte_add(4),
                        write_unaligned,
                        mech_trace::gr!("100315b0", "10082c2c", DAT_10082c2c)
                    );
                    FUN_100319b0(mech_trace::gr!("100315b0", "10092000", DAT_10092000));
                }
                mech_trace::wr!("100315b0", p.byte_add(8).cast::<i8>(), write, 2);
                mech_trace::wr!("100315b0", p.byte_add(4), write_unaligned, 0);
                mech_trace::wr!(
                    "100315b0",
                    p.byte_add(10).cast::<i32>(),
                    write_unaligned,
                    -1
                );
                mech_trace::wr!(
                    "100315b0",
                    p.byte_add(0xe).cast::<i32>(),
                    write_unaligned,
                    -1
                );
                mech_trace::wr!(
                    "100315b0",
                    p.byte_add(0x12).cast::<i32>(),
                    write_unaligned,
                    -1
                );
                FUN_1000c770(p, 0);
                FUN_10050400(p);
                FUN_10017df0(mech_trace::gr!("100315b0", "10092010", DAT_10092010), p);
            }
        }
    }
}
