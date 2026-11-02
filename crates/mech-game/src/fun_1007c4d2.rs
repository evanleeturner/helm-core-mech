// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: i32;
    static mut DAT_100b67f8: i32;
    static mut DAT_100cded0: [u8; 0];
    static mut DAT_100cded4: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f28d(param_1: i32) -> i32;
    fn FUN_1001cac9(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32) -> i32;
    fn FUN_1001b520(param_1: *mut i32, param_2: *mut i32, param_3: i32, param_4: i32);
    fn FUN_100142b4(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1007cbc4(param_1: *mut i32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002f6a8(param_1: *mut c_void, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1002f322(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_1007c97d(param_1: i32);
    fn FUN_10024b86(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c4d2(param_1: i32) {
    mech_trace::enter!("1007c4d2", param_1);
    // SAFETY: the globals, callees and the table at DAT_100cded0/4 are the
    // game's own memory; entries are 0x24 bytes and accessed as the original does.
    unsafe {
        let mut local_5c: i32;
        let local_58: u32;
        let mut local_54: i32 = 0;
        let mut local_3c: i32 = 0;
        let mut local_30: i32 = 0;
        let mut local_28: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_10: i32;

        let off = (param_1 as u32).wrapping_mul(0x24) as i32;
        let local_50: *mut i32 = (&raw mut DAT_100cded0)
            .cast::<u8>()
            .byte_offset(off as isize)
            .cast::<i32>();
        let tab4 = mech_trace::rd!(
            "1007c4d2",
            (&raw mut DAT_100cded4)
                .cast::<u8>()
                .byte_offset(off as isize)
                .cast::<i32>(),
            read_unaligned
        );
        if tab4 != 0 {
            let i_var1 = mech_trace::cr!("1007c4d2", "1002f28d", FUN_1002f28d(tab4));
            let local_48 = mech_trace::cr!(
                "1007c4d2",
                "1001cac9",
                FUN_1001cac9(
                    i_var1,
                    &raw mut local_28,
                    &raw mut local_30,
                    &raw mut local_3c,
                )
            );
            let local_38 = local_28;
            let local_40 = local_30;
            let local_44 = local_3c;
            local_30 = local_30.wrapping_sub(local_48 >> 1);
            local_5c = mech_trace::rd!("1007c4d2", local_50.add(3), read_unaligned);
            local_58 = (0 < local_5c) as u32;
            local_10 = local_30;
            FUN_1001b520(
                &raw mut local_10,
                &raw mut local_5c,
                mech_trace::rd!("1007c4d2", local_50.add(8), read_unaligned),
                mech_trace::gr!("1007c4d2", "100b3904", DAT_100b3904),
            );
            if local_5c < 1 {
                let local_4c = mech_trace::cr!(
                    "1007c4d2",
                    "100142b4",
                    FUN_100142b4(local_28, local_30, local_3c)
                );
                if local_10 <= local_4c {
                    if local_58 != 0 {
                        FUN_1007cbc4(mech_trace::rd!("1007c4d2", local_50.add(1), read_unaligned)
                            as *mut i32);
                        local_54 = 1;
                    }
                    if -0x8d6f < local_5c {
                        local_54 = 1;
                    }
                    local_10 = local_4c;
                    local_5c = 0i32.wrapping_sub(local_5c >> 2);
                    let r = mech_trace::cr!("1007c4d2", "10051f43", FUN_10051f43(2));
                    if r != 0 {
                        local_5c >>= 1;
                        let v = mech_trace::rd!("1007c4d2", local_50.add(2), read_unaligned);
                        mech_trace::wr!(
                            "1007c4d2",
                            local_50.add(2),
                            write_unaligned,
                            0i32.wrapping_sub(v >> 1)
                        );
                        let v = mech_trace::rd!("1007c4d2", local_50.add(4), read_unaligned);
                        mech_trace::wr!(
                            "1007c4d2",
                            local_50.add(4),
                            write_unaligned,
                            0i32.wrapping_sub(v >> 1)
                        );
                    }
                    let v = mech_trace::rd!("1007c4d2", local_50.add(5), read_unaligned);
                    mech_trace::wr!(
                        "1007c4d2",
                        local_50.add(5),
                        write_unaligned,
                        0i32.wrapping_sub(v >> 1)
                    );
                    let v = mech_trace::rd!("1007c4d2", local_50.add(6), read_unaligned);
                    mech_trace::wr!(
                        "1007c4d2",
                        local_50.add(6),
                        write_unaligned,
                        0i32.wrapping_sub(v >> 1)
                    );
                    let v = mech_trace::rd!("1007c4d2", local_50.add(7), read_unaligned);
                    mech_trace::wr!(
                        "1007c4d2",
                        local_50.add(7),
                        write_unaligned,
                        0i32.wrapping_sub(v >> 1)
                    );
                }
            }
            mech_trace::wr!("1007c4d2", local_50.add(3), write_unaligned, local_5c);
            let local_c = mech_trace::cr!(
                "1007c4d2",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("1007c4d2", local_50.add(2), read_unaligned),
                    mech_trace::gr!("1007c4d2", "100b3904", DAT_100b3904)
                )
            );
            let local_8 = local_10.wrapping_sub(local_30);
            let local_14 = mech_trace::cr!(
                "1007c4d2",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("1007c4d2", local_50.add(4), read_unaligned),
                    mech_trace::gr!("1007c4d2", "100b3904", DAT_100b3904)
                )
            );
            let local_20 = mech_trace::cr!(
                "1007c4d2",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("1007c4d2", local_50.add(5), read_unaligned),
                    ((mech_trace::gr!("1007c4d2", "100b3904", DAT_100b3904) as u32)
                        .wrapping_shl(0x10)) as i32,
                )
            );
            let local_2c = mech_trace::cr!(
                "1007c4d2",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("1007c4d2", local_50.add(6), read_unaligned),
                    ((mech_trace::gr!("1007c4d2", "100b3904", DAT_100b3904) as u32)
                        .wrapping_shl(0x10)) as i32,
                )
            );
            let local_34 = mech_trace::cr!(
                "1007c4d2",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("1007c4d2", local_50.add(7), read_unaligned),
                    ((mech_trace::gr!("1007c4d2", "100b3904", DAT_100b3904) as u32)
                        .wrapping_shl(0x10)) as i32,
                )
            );
            FUN_1002f3a7(
                mech_trace::rd!("1007c4d2", local_50.add(1), read_unaligned),
                local_c,
                local_8,
                local_14,
            );
            FUN_1002f6a8(
                mech_trace::rd!("1007c4d2", local_50.add(1), read_unaligned) as *mut c_void,
                local_20,
                local_2c,
                local_34,
                0,
            );
            FUN_1002fbc0(mech_trace::rd!("1007c4d2", local_50.add(1), read_unaligned) as *mut i32);
            FUN_1002f322(
                mech_trace::rd!("1007c4d2", local_50.add(1), read_unaligned),
                &raw mut local_18,
                &raw mut local_1c,
                &raw mut local_24,
            );
            if local_54 == 0 {
                if mech_trace::rd!("1007c4d2", local_50, read_unaligned) == 1 {
                    FUN_10024b86(
                        local_38,
                        local_40,
                        local_44,
                        local_18.wrapping_sub(local_38),
                        local_1c.wrapping_sub(local_40),
                        local_24.wrapping_sub(local_44),
                    );
                }
            } else {
                mech_trace::gw!(
                    "1007c4d2",
                    "100b67f8",
                    DAT_100b67f8,
                    mech_trace::gr!("1007c4d2", "100b67f8", DAT_100b67f8).wrapping_add(-1)
                );
                FUN_1007c97d(param_1);
            }
        }
    }
}
