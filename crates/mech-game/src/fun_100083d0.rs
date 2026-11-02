// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7188: *mut u8;
    static mut DAT_100f3040: *mut u8;
    static mut DAT_101f5300: u32;
    static mut DAT_10215658: i32;
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100b2cf0: *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10065e40(param_1: u32, param_2: i32, param_3: i32) -> i32;
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_10066145(param_1: i32, param_2: u32) -> i32;
    fn FUN_10066342(param_1: u32, param_2: *mut i32, param_3: i32) -> i32;
    fn FUN_10065ebf(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10009ce8(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100083d0(param_1: i32, param_2: *mut u32) {
    mech_trace::enter!("100083d0", param_1, param_2);
    if param_2.is_null() {
        return;
    }
    // SAFETY: the globals and callees are the game's own; param_2 and the
    // structures it links to are the caller's memory, accessed as the original does.
    unsafe {
        let pu_var1 = mech_trace::rd!("100083d0", param_2, read_unaligned) as *mut u32;
        mech_trace::gw!("100083d0", "10215658", DAT_10215658, param_1);
        mech_trace::wr!(
            "100083d0",
            pu_var1,
            write_unaligned,
            mech_trace::gr!("100083d0", "101f5300", DAT_101f5300)
        );
        mech_trace::cr!(
            "100083d0",
            "10065e40",
            FUN_10065e40(
                mech_trace::gr!("100083d0", "101f5300", DAT_101f5300),
                pu_var1 as i32,
                pu_var1 as i32
            )
        );
        if param_1 == 1 {
            let mut src = pu_var1;
            let mut dst = mech_trace::gr!("100083d0", "100b2cf0", DAT_100b2cf0);
            for _ in 0..5 {
                mech_trace::wr!(
                    "100083d0",
                    dst,
                    write_unaligned,
                    mech_trace::rd!("100083d0", src, read_unaligned)
                );
                src = src.add(1);
                dst = dst.add(1);
            }
        }
        let mut src = pu_var1;
        let idx = mech_trace::rd!("100083d0", param_2.add(2), read_unaligned);
        let mut dst = mech_trace::gr!("100083d0", "100f3040", DAT_100f3040)
            .byte_add(idx.wrapping_mul(0x14) as usize) as *mut u32;
        for _ in 0..5 {
            mech_trace::wr!(
                "100083d0",
                dst,
                write_unaligned,
                mech_trace::rd!("100083d0", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
        }
        mech_trace::cr!(
            "100083d0",
            "100660e1",
            FUN_100660e1(
                pu_var1 as i32,
                param_2.add(0x16) as *mut i32,
                param_2.add(0x16) as *mut i32,
            )
        );
        mech_trace::cr!(
            "100083d0",
            "100660e1",
            FUN_100660e1(
                pu_var1 as i32,
                param_2.add(0x18) as *mut i32,
                param_2.add(0x18) as *mut i32,
            )
        );
        mech_trace::cr!(
            "100083d0",
            "100660e1",
            FUN_100660e1(
                pu_var1 as i32,
                param_2.add(0x1a) as *mut i32,
                param_2.add(0x1a) as *mut i32,
            )
        );
        let i_var3 = mech_trace::rd!("100083d0", param_2.add(5), read_unaligned) as i32;
        if i_var3 != 0 {
            let inner = mech_trace::rd!(
                "100083d0",
                ((i_var3 as usize + 4) as *const i32),
                read_unaligned
            );
            let mut pi_var2 = mech_trace::rd!(
                "100083d0",
                ((inner as usize + 4) as *const i32),
                read_unaligned
            ) as *mut i32;
            mech_trace::wr!(
                "100083d0",
                pi_var2,
                write_unaligned,
                mech_trace::gr!("100083d0", "101f5300", DAT_101f5300) as i32
            );
            if param_1 == 4 {
                let v = mech_trace::rd!(
                    "100083d0",
                    mech_trace::gr!("100083d0", "100aaf68", DAT_100aaf68)
                        .byte_add(0x44)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!("100083d0", "10066145", FUN_10066145(pi_var2 as i32, v));
                mech_trace::cr!(
                    "100083d0",
                    "10066342",
                    FUN_10066342(
                        mech_trace::gr!("100083d0", "101f5300", DAT_101f5300),
                        pi_var2,
                        pi_var2 as i32
                    )
                );
            } else {
                mech_trace::cr!(
                    "100083d0",
                    "10065ebf",
                    FUN_10065ebf(pu_var1 as i32, pi_var2 as i32, pi_var2 as i32)
                );
            }
            let inner = mech_trace::rd!(
                "100083d0",
                ((i_var3 as usize + 4) as *const i32),
                read_unaligned
            );
            pi_var2 = mech_trace::rd!(
                "100083d0",
                ((inner as usize + 8) as *const i32),
                read_unaligned
            ) as *mut i32;
            mech_trace::wr!(
                "100083d0",
                pi_var2,
                write_unaligned,
                mech_trace::gr!("100083d0", "101f5300", DAT_101f5300) as i32
            );
            if param_1 == 4 {
                let v = mech_trace::rd!(
                    "100083d0",
                    mech_trace::gr!("100083d0", "100aaf68", DAT_100aaf68)
                        .byte_add(0x44)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!("100083d0", "10066145", FUN_10066145(pi_var2 as i32, v));
                mech_trace::cr!(
                    "100083d0",
                    "10066342",
                    FUN_10066342(
                        mech_trace::gr!("100083d0", "101f5300", DAT_101f5300),
                        pi_var2,
                        pi_var2 as i32
                    )
                );
            } else {
                mech_trace::cr!(
                    "100083d0",
                    "10065ebf",
                    FUN_10065ebf(pu_var1 as i32, pi_var2 as i32, pi_var2 as i32)
                );
            }
            let inner = mech_trace::rd!(
                "100083d0",
                ((i_var3 as usize + 4) as *const i32),
                read_unaligned
            );
            let p = mech_trace::rd!(
                "100083d0",
                ((inner as usize + 0xc) as *const i32),
                read_unaligned
            ) as *mut u32;
            mech_trace::wr!(
                "100083d0",
                p,
                write_unaligned,
                mech_trace::gr!("100083d0", "101f5300", DAT_101f5300)
            );
        }
        for k in [0x1fusize, 0x20, 0x21, 0x22] {
            let i = mech_trace::rd!("100083d0", param_2.add(k), read_unaligned);
            let v = mech_trace::rd!(
                "100083d0",
                mech_trace::gr!("100083d0", "100a7188", DAT_100a7188)
                    .byte_add(i.wrapping_mul(4) as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!("100083d0", param_2.add(k), write_unaligned, v);
        }
        mech_trace::cr!("100083d0", "10009ce8", FUN_10009ce8(0));
    }
}
