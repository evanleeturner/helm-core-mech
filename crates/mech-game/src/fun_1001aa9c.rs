// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab07c: i32;
    static mut DAT_100ab318: i32;
    static mut DAT_100ab060: i32;
    static mut DAT_100ab064: i32;
    static mut DAT_100ab068: i32;
    static mut DAT_100ab06c: i32;
    static mut DAT_100ab070: i32;
    static mut DAT_100ab078: i32;
    static mut DAT_100eef40: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100aaf68: *mut u8;

    static mut DAT_101eb394: i32;
    static mut DAT_101eb34c: i32;
    static mut DAT_101eb470: i32;
    static mut DAT_101eb474: i32;
    static mut DAT_101eb478: i32;
    static mut DAT_101eb3b0: i32;
    static mut DAT_101eb3b4: i32;
    static mut DAT_101eb3b8: i32;
    static mut DAT_101eb350: i32;
    static mut DAT_101eb354: i32;
    static mut DAT_101eb358: i32;
    static mut DAT_101eb380: i32;
    static mut DAT_101eb388: i32;
    static mut DAT_101eb360: i32;
    static mut DAT_101eb368: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a8a3(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
    fn FUN_100082c4(param_1: i32, param_2: i32);
    fn FUN_1001a635(param_1: i32);
    fn FUN_10083a59(
        param_1: u32,
        param_2: i32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    );
    fn FUN_10067ce5(param_1: i32, param_2: i32);
    fn FUN_10067bf5(param_1: *mut i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> i32;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10067af1(param_1: *mut i32) -> i32;
    fn FUN_1001a58f(param_1: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001aa9c(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("1001aa9c", param_1, param_2, param_3, param_4);
    let mut local_58: u32 = 0;
    let mut local_54: i32 = 0;
    let local_50: i32;
    let local_4c: i32;
    let mut local_48: u32 = 0;
    let mut local_44: i32 = 0;
    let local_40: i32;
    let mut local_3c: i32 = 0;
    let local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32 = 0;
    let local_2c: i32;
    let local_28: i32;
    let mut local_24: i32 = 0;
    let mut local_20: i32 = 0;
    let local_1c: u32;
    let mut local_18: i32;
    let local_14: i32;
    let mut local_10: i32 = 0;
    let local_c: u32;
    let mut local_8: i32 = 0;

    // SAFETY: all globals and callees are the game's own; the pointer global
    // refers to the game's state block, read and written where the original does.
    unsafe {
        FUN_1001a8a3(
            &raw mut local_30,
            &raw mut local_3c,
            &raw mut local_44,
            &raw mut local_54,
            &raw mut local_8,
            &raw mut local_10,
        );
        if mech_trace::gr!("1001aa9c", "100ab07c", DAT_100ab07c) != 1 {
            if mech_trace::gr!("1001aa9c", "100ab318", DAT_100ab318) == 0 {
                FUN_100082c4(0x11, -1);
            }
            if mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060) == 0 {
                let base = mech_trace::rd!(
                    "1001aa9c",
                    ((mech_trace::gr!("1001aa9c", "100eef40", DAT_100eef40).wrapping_add(0x20))
                        as usize as *const i32),
                    read_unaligned
                );
                let iv = mech_trace::rd!(
                    "1001aa9c",
                    (base as usize as *const u8).byte_add(0x41c).cast::<i32>(),
                    read_unaligned
                );
                mech_trace::gw!("1001aa9c", "100ab060", DAT_100ab060, iv.wrapping_mul(3));
                mech_trace::gw!(
                    "1001aa9c",
                    "100ab064",
                    DAT_100ab064,
                    mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060) >> 1
                );
                mech_trace::gw!("1001aa9c", "100ab068", DAT_100ab068, iv.wrapping_mul(0xc));
                mech_trace::gw!(
                    "1001aa9c",
                    "100ab06c",
                    DAT_100ab06c,
                    mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060) >> 2
                );
                let f400 = mech_trace::rd!(
                    "1001aa9c",
                    (base as usize as *const u8).byte_add(0x400).cast::<i32>(),
                    read_unaligned
                );
                mech_trace::gw!(
                    "1001aa9c",
                    "101eb394",
                    DAT_101eb394,
                    200i32.wrapping_sub(f400)
                );
                mech_trace::gw!(
                    "1001aa9c",
                    "101eb34c",
                    DAT_101eb34c,
                    mech_trace::gr!("1001aa9c", "100ab068", DAT_100ab068)
                );
            }
            mech_trace::gw!(
                "1001aa9c",
                "101eb470",
                DAT_101eb470,
                mech_trace::gr!("1001aa9c", "100b38fc", DAT_100b38fc)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb3b0",
                DAT_101eb3b0,
                mech_trace::gr!("1001aa9c", "100b38fc", DAT_100b38fc)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb350",
                DAT_101eb350,
                mech_trace::gr!("1001aa9c", "100b38fc", DAT_100b38fc)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb380",
                DAT_101eb380,
                mech_trace::gr!("1001aa9c", "100b38fc", DAT_100b38fc)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb360",
                DAT_101eb360,
                mech_trace::gr!("1001aa9c", "100b38fc", DAT_100b38fc)
            );
            let p = mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68);
            mech_trace::gw!(
                "1001aa9c",
                "101eb478",
                DAT_101eb478,
                mech_trace::rd!("1001aa9c", p.cast::<i32>(), read_unaligned).wrapping_sub(local_54)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb3b8",
                DAT_101eb3b8,
                mech_trace::rd!("1001aa9c", p.byte_add(4).cast::<i32>(), read_unaligned)
                    .wrapping_sub(local_8)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb358",
                DAT_101eb358,
                mech_trace::rd!("1001aa9c", p.byte_add(8).cast::<i32>(), read_unaligned)
                    .wrapping_sub(local_10)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb388",
                DAT_101eb388,
                mech_trace::rd!("1001aa9c", p.byte_add(0xc).cast::<i32>(), read_unaligned)
            );
            mech_trace::gw!(
                "1001aa9c",
                "101eb368",
                DAT_101eb368,
                mech_trace::rd!("1001aa9c", p.byte_add(0x10).cast::<i32>(), read_unaligned)
            );
            mech_trace::wr!(
                "1001aa9c",
                p.byte_add(0x14).cast::<i32>(),
                write_unaligned,
                0
            );
            if mech_trace::rd!("1001aa9c", p.cast::<i32>(), read_unaligned) == local_54 {
                let v = mech_trace::rd!("1001aa9c", p.cast::<i32>(), read_unaligned);
                mech_trace::wr!(
                    "1001aa9c",
                    p.cast::<i32>(),
                    write_unaligned,
                    v.wrapping_add(10)
                );
            }
            mech_trace::gw!("1001aa9c", "100ab078", DAT_100ab078, 0x10000);
            FUN_1001a635(0);
        }
        mech_trace::gw!(
            "1001aa9c",
            "100ab060",
            DAT_100ab060,
            mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060).wrapping_add(param_1)
        );
        if mech_trace::gr!("1001aa9c", "100ab068", DAT_100ab068)
            < mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060)
        {
            mech_trace::gw!(
                "1001aa9c",
                "100ab060",
                DAT_100ab060,
                mech_trace::gr!("1001aa9c", "100ab068", DAT_100ab068)
            );
        } else if mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060)
            < mech_trace::gr!("1001aa9c", "100ab064", DAT_100ab064)
        {
            mech_trace::gw!(
                "1001aa9c",
                "100ab060",
                DAT_100ab060,
                mech_trace::gr!("1001aa9c", "100ab064", DAT_100ab064)
            );
        }
        local_4c = mech_trace::gr!("1001aa9c", "100ab06c", DAT_100ab06c).wrapping_add(param_2);
        mech_trace::gw!(
            "1001aa9c",
            "100ab070",
            DAT_100ab070,
            mech_trace::gr!("1001aa9c", "100ab070", DAT_100ab070).wrapping_add(param_4)
        );
        mech_trace::gw!(
            "1001aa9c",
            "100ab070",
            DAT_100ab070,
            mech_trace::gr!("1001aa9c", "100ab070", DAT_100ab070) % 0x1680000
        );
        let p = mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68);
        local_14 = local_8.wrapping_sub(mech_trace::rd!(
            "1001aa9c",
            p.byte_add(4).cast::<i32>(),
            read_unaligned
        ));
        local_1c = (local_10.wrapping_sub(mech_trace::rd!(
            "1001aa9c",
            p.byte_add(8).cast::<i32>(),
            read_unaligned
        ))) as u32;
        local_c =
            (local_54.wrapping_sub(mech_trace::rd!("1001aa9c", p.cast::<i32>(), read_unaligned)))
                as u32;
        FUN_10083a59(
            local_c,
            local_14,
            local_1c,
            &raw mut local_20,
            &raw mut local_58,
            &raw mut local_48,
            &raw mut local_24,
        );
        FUN_10067ce5(
            (&raw mut DAT_101eb360) as usize as i32,
            0i32.wrapping_sub(param_3 >> 1).wrapping_sub(local_24),
        );
        let iv = mech_trace::cr!("1001aa9c", "10067bf5", FUN_10067bf5(&raw mut DAT_101eb360));
        mech_trace::wr!(
            "1001aa9c",
            mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68)
                .byte_add(0x10)
                .cast::<i32>(),
            write_unaligned,
            iv
        );
        FUN_10067ce5((&raw mut DAT_101eb380) as usize as i32, local_20);
        let iv = mech_trace::cr!("1001aa9c", "10067bf5", FUN_10067bf5(&raw mut DAT_101eb380));
        mech_trace::wr!(
            "1001aa9c",
            mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68)
                .byte_add(0xc)
                .cast::<i32>(),
            write_unaligned,
            iv
        );
        local_18 = local_3c.wrapping_sub(mech_trace::gr!("1001aa9c", "100ab070", DAT_100ab070))
            % 0x1680000;
        if local_18 < -0xb40000 {
            local_18 = local_18.wrapping_add(0x1680000);
        } else if 0xb40000 < local_18 {
            local_18 = local_18.wrapping_add(0xFE980000u32 as i32);
        }
        local_50 = mech_trace::cr!("1001aa9c", "10006b9a", FUN_10006b9a(local_18));
        local_38 = mech_trace::cr!("1001aa9c", "10006b20", FUN_10006b20(local_18));
        local_2c = mech_trace::cr!(
            "1001aa9c",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060),
                local_38
            )
        ) >> 0xd;
        local_40 = mech_trace::cr!(
            "1001aa9c",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1001aa9c", "100ab060", DAT_100ab060),
                local_50
            )
        ) >> 0xd;
        local_34 = local_4c;
        mech_trace::gw!("1001aa9c", "101eb474", DAT_101eb474, local_2c);
        let iv = mech_trace::cr!("1001aa9c", "10067af1", FUN_10067af1(&raw mut DAT_101eb470));
        mech_trace::wr!(
            "1001aa9c",
            mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68).cast::<i32>(),
            write_unaligned,
            local_54.wrapping_add(iv)
        );
        mech_trace::gw!("1001aa9c", "101eb354", DAT_101eb354, local_40);
        let iv = mech_trace::cr!("1001aa9c", "10067af1", FUN_10067af1(&raw mut DAT_101eb350));
        mech_trace::wr!(
            "1001aa9c",
            mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68)
                .byte_add(8)
                .cast::<i32>(),
            write_unaligned,
            local_10.wrapping_add(iv)
        );
        local_28 = mech_trace::cr!(
            "1001aa9c",
            "1001a58f",
            FUN_1001a58f(mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68).cast::<i32>())
        );
        if local_8.wrapping_add(local_34) < local_28 {
            local_34 = local_28.wrapping_sub(local_8);
        }
        mech_trace::gw!("1001aa9c", "100ab06c", DAT_100ab06c, local_4c);
        if mech_trace::gr!("1001aa9c", "101eb34c", DAT_101eb34c) < local_4c {
            mech_trace::gw!(
                "1001aa9c",
                "100ab06c",
                DAT_100ab06c,
                mech_trace::gr!("1001aa9c", "101eb34c", DAT_101eb34c)
            );
        } else if local_4c < mech_trace::gr!("1001aa9c", "101eb394", DAT_101eb394) {
            mech_trace::gw!(
                "1001aa9c",
                "100ab06c",
                DAT_100ab06c,
                mech_trace::gr!("1001aa9c", "101eb394", DAT_101eb394)
            );
        }
        mech_trace::gw!("1001aa9c", "101eb3b4", DAT_101eb3b4, local_34);
        let iv = mech_trace::cr!("1001aa9c", "10067af1", FUN_10067af1(&raw mut DAT_101eb3b0));
        mech_trace::wr!(
            "1001aa9c",
            mech_trace::gr!("1001aa9c", "100aaf68", DAT_100aaf68)
                .byte_add(4)
                .cast::<i32>(),
            write_unaligned,
            local_8.wrapping_add(iv)
        );
    }
}
