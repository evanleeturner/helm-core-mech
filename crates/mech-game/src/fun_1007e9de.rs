// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_10123130: [u8; 0];
    static mut DAT_1012327a: [u8; 0];
    static mut DAT_1012327c: [u8; 0];
    static mut DAT_10123368: [u8; 0];
    static mut DAT_10123369: [u8; 0];
    static mut DAT_1012336a: [u8; 0];
    static mut DAT_10215688: [u8; 0];
    static mut DAT_1021568c: [u8; 0];
    static mut DAT_10215690: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10026f0b(param_1: *mut i32);
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_1003c190(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32);
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1006b2cc(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004fed5();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007e9de(param_1: *mut i32) {
    mech_trace::enter!("1007e9de", param_1);
    // SAFETY: all globals, callees and the object pointed to by param_1 are
    // the game's own memory; accesses mirror the original's reads and writes.
    unsafe {
        let local_8 = param_1;
        let base = mech_trace::rd!("1007e9de", local_8, read) as *mut u8;
        FUN_10026f0b(base as *mut i32);
        if mech_trace::rd!("1007e9de", local_8.add(0xf5), read) == 2 {
            let base = mech_trace::rd!("1007e9de", local_8, read) as *mut u8;
            let local_c =
                mech_trace::rd!("1007e9de", base.byte_add(8).cast::<i32>(), read_unaligned);
            let tbl = (&raw const DAT_10123130)
                .cast::<u8>()
                .byte_offset(local_c.wrapping_mul(4) as isize);
            let iv = mech_trace::rd!("1007e9de", tbl.cast::<i32>(), read_unaligned)
                .wrapping_mul(0x13f)
                .wrapping_add(local_c.wrapping_mul(0x3c0a));
            mech_trace::wr!(
                "1007e9de",
                base.byte_add(0xdc).cast::<u32>(),
                write_unaligned,
                0
            );
            let p4c = mech_trace::rd!(
                "1007e9de",
                base.byte_add(0x4c).cast::<i32>(),
                read_unaligned
            ) as *mut u8;
            mech_trace::wr!("1007e9de", p4c.byte_add(0x43), write, 0);
            let a = (&raw const DAT_1012327c)
                .cast::<u8>()
                .byte_offset(iv as isize);
            let b = (&raw const DAT_10123368)
                .cast::<u8>()
                .byte_offset(iv as isize);
            let c = (&raw const DAT_1012336a)
                .cast::<u8>()
                .byte_offset(iv as isize);
            if mech_trace::rd!("1007e9de", a.cast::<i32>(), read_unaligned) == 0x100
                && mech_trace::rd!("1007e9de", b, read) != 0
                && mech_trace::rd!("1007e9de", c, read) == 1
            {
                let d = (&raw const DAT_10123369)
                    .cast::<u8>()
                    .byte_offset(iv as isize);
                let local_18 = u32::from(mech_trace::rd!("1007e9de", d, read));
                let mut local_1c: u32 = 0;
                let mut local_28: u32 = 0;
                let mut local_30: u32 = 0;
                let base = mech_trace::rd!("1007e9de", local_8, read) as *mut u8;
                FUN_1002f322(
                    mech_trace::rd!(
                        "1007e9de",
                        base.byte_add(0x40).cast::<i32>(),
                        read_unaligned
                    ),
                    &raw mut local_1c,
                    &raw mut local_28,
                    &raw mut local_30,
                );
                let off = local_18.wrapping_mul(0x54) as usize;
                let local_20 = mech_trace::rd!(
                    "1007e9de",
                    (&raw const DAT_10215688)
                        .cast::<u8>()
                        .byte_add(off)
                        .cast::<u32>(),
                    read_unaligned
                );
                let local_2c = mech_trace::rd!(
                    "1007e9de",
                    (&raw const DAT_1021568c)
                        .cast::<u8>()
                        .byte_add(off)
                        .cast::<u32>(),
                    read_unaligned
                );
                let local_34 = mech_trace::rd!(
                    "1007e9de",
                    (&raw const DAT_10215690)
                        .cast::<u8>()
                        .byte_add(off)
                        .cast::<u32>(),
                    read_unaligned
                );
                let mut local_38 = local_20.wrapping_sub(local_1c);
                let mut local_3c = local_2c.wrapping_sub(local_28);
                let mut local_14 = local_34.wrapping_sub(local_30);
                let arg = (mech_trace::rd!("1007e9de", local_8.add(0xee), read) as u32)
                    .wrapping_mul(mech_trace::gr!("1007e9de", "100b3904", DAT_100b3904));
                let local_24 =
                    mech_trace::cr!("1007e9de", "10002570", FUN_10002570(arg, 0x697e98)) as i32;
                let r = mech_trace::cr!(
                    "1007e9de",
                    "1003c190",
                    FUN_1003c190(local_38 as i32, local_3c as i32, local_14 as i32, local_24)
                );
                if r == 0 {
                    local_38 = local_20;
                    local_3c = local_2c;
                    local_14 = local_34;
                } else {
                    FUN_1000fef4(&raw mut local_38, &raw mut local_3c, &raw mut local_14);
                    local_38 = mech_trace::cr!(
                        "1007e9de",
                        "10003d60",
                        FUN_10003d60(local_38 as i32, local_24)
                    ) as u32;
                    local_3c = mech_trace::cr!(
                        "1007e9de",
                        "10003d60",
                        FUN_10003d60(local_3c as i32, local_24)
                    ) as u32;
                    local_14 = mech_trace::cr!(
                        "1007e9de",
                        "10003d60",
                        FUN_10003d60(local_14 as i32, local_24)
                    ) as u32;
                }
                mech_trace::wr!("1007e9de", local_8.add(10), write, local_38 as i32);
                mech_trace::wr!("1007e9de", local_8.add(0x12), write, local_3c as i32);
                mech_trace::wr!("1007e9de", local_8.add(0xe), write, local_14 as i32);
            }
        } else {
            let base = mech_trace::rd!("1007e9de", local_8, read) as *mut u8;
            mech_trace::wr!(
                "1007e9de",
                base.byte_add(0x8c).cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "1007e9de",
                base.byte_add(0x88).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
        }
        match mech_trace::rd!("1007e9de", local_8.add(0xf5), read) {
            0 => {
                mech_trace::wr!("1007e9de", local_8.add(0xf5), write, 2);
                mech_trace::wr!("1007e9de", local_8.add(0xef), write, 0);
            }
            1 => {
                mech_trace::wr!("1007e9de", local_8.add(0xf5), write, 2);
            }
            2 => {}
            3 => {
                mech_trace::wr!("1007e9de", local_8.add(0xf5), write, 2);
            }
            4 => {
                if mech_trace::rd!("1007e9de", local_8.add(0xef), read) == 0 {
                    mech_trace::wr!(
                        "1007e9de",
                        local_8.add(0xef),
                        write,
                        mech_trace::gr!("1007e9de", "100b38fc", DAT_100b38fc).wrapping_add(0x712)
                            as i32
                    );
                    FUN_1004fed5();
                }
                if (mech_trace::gr!("1007e9de", "100b38fc", DAT_100b38fc) as i32)
                    < mech_trace::rd!("1007e9de", local_8.add(0xef), read)
                {
                    FUN_1006b2cc(local_8);
                }
            }
            _ => {}
        }
    }
}
