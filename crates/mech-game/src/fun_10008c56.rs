// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7178: i32;
    static mut DAT_100a7310: u32;
    static mut DAT_100aafc8: u32;
    static mut DAT_100aafd0: u32;
    static mut DAT_100aafd4: u32;
    static mut DAT_100ad6a4: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100f3040: [u8; 0];
    static mut DAT_10215658: i32;
    static mut DAT_10215664: u32;
    static mut DAT_100aaf68: u32;
    static mut DAT_100a7b40: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10035220(param_1: *mut u32, param_2: i32, param_3: i32, param_4: u32) -> u8;
    fn FUN_1001a5e1(param_1: i32) -> u8;
    fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32;
    fn FUN_100353df(param_1: u8) -> u8;
    fn FUN_10009896(param_1: *mut i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_10008ed5(param_1: *mut i32) -> u8;
    fn FUN_10009e05(param_1: *mut u16) -> u32;
    fn FUN_10009edf(param_1: *mut u16, param_2: i32, param_3: u32) -> u32;
    fn FUN_1000a1d2(param_1: i32, param_2: *mut u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001a620() -> u32;
    fn FUN_10050b20() -> u8;
    fn FUN_1003545c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10008c56(param_1: *mut i32) {
    mech_trace::enter!("10008c56", param_1);
    // SAFETY: globals and callees are the game's own; param_1 and the
    // structures reached through the globals are the caller's memory, read
    // and written at the same offsets and in the same order as the original.
    unsafe {
        let local_3c = mech_trace::gr!("10008c56", "10215658", DAT_10215658);
        mech_trace::gw!("10008c56", "10215658", DAT_10215658, 4);
        let local_38 = mech_trace::gr!("10008c56", "100a7b40", DAT_100a7b40) as *mut i32;
        if mech_trace::gr!("10008c56", "100a7b40", DAT_100a7b40).is_null() {
            mech_trace::gw!("10008c56", "10215658", DAT_10215658, 4);
            return;
        }
        let i_var1 = mech_trace::rd!(
            "10008c56",
            (&raw mut DAT_100eef40).cast::<u32>().add(mech_trace::gr!(
                "10008c56",
                "100adf38",
                DAT_100adf38
            ) as usize),
            read
        ) as i32;
        mech_trace::gw!("10008c56", "100a7310", DAT_100a7310, param_1 as u32);
        let i_var2 = mech_trace::rd!(
            "10008c56",
            mech_trace::gr!("10008c56", "100a7b40", DAT_100a7b40)
                .byte_add(8)
                .cast::<i32>(),
            read_unaligned
        );
        let mut pi4 = param_1;
        let mut pi5 = (&raw mut DAT_100f3040)
            .cast::<u8>()
            .byte_offset(i_var2.wrapping_mul(0x14) as isize)
            .cast::<i32>();
        let mut i_var3 = 5;
        while i_var3 != 0 {
            mech_trace::wr!(
                "10008c56",
                pi5,
                write_unaligned,
                mech_trace::rd!("10008c56", pi4, read_unaligned)
            );
            pi4 = pi4.add(1);
            pi5 = pi5.add(1);
            i_var3 -= 1;
        }
        let mut local_10 = mech_trace::rd!(
            "10008c56",
            ((i_var1 as u32).wrapping_add(0x20) as *const u32),
            read_unaligned
        )
        .wrapping_add(0xc) as *const i32;
        let mut local_10v = mech_trace::rd!("10008c56", local_10, read_unaligned);
        let local_c = mech_trace::rd!(
            "10008c56",
            ((i_var1 as u32).wrapping_add(0x60) as *const i32),
            read_unaligned
        );
        let _ = &mut local_10;
        if 0 < mech_trace::gr!("10008c56", "10215658", DAT_10215658) {
            if mech_trace::gr!("10008c56", "10215658", DAT_10215658) < 3 {
            } else if mech_trace::gr!("10008c56", "10215658", DAT_10215658) == 4 {
                local_10v = local_10v.wrapping_add(local_c);
            } else {
                local_10v = 0;
            }
        } else {
            local_10v = 0;
        }
        let mut local_34: u32;
        if mech_trace::gr!("10008c56", "10215664", DAT_10215664) == 0 {
            let mut local_44: u32 = 0;
            let mut local_48: u32 = 0;
            let mut local_4c: u32 = 0;
            let local_14 = mech_trace::rd!(
                "10008c56",
                ((i_var1 as u32).wrapping_add(0x44) as *const i32),
                read_unaligned
            );
            if local_14 != 0 {
                mech_trace::cr!(
                    "10008c56",
                    "1002f322",
                    FUN_1002f322(
                        local_14,
                        &raw mut local_4c,
                        &raw mut local_48,
                        &raw mut local_44,
                    )
                );
            }
            local_34 = local_4c;
        } else {
            local_34 = mech_trace::rd!(
                "10008c56",
                (mech_trace::gr!("10008c56", "100aaf68", DAT_100aaf68) as *const u32),
                read_unaligned
            );
        }
        let local_40 = mech_trace::rd!("10008c56", local_38.add(6), read_unaligned);
        let mut local_8 = local_40;
        if mech_trace::gr!("10008c56", "100a7178", DAT_100a7178) < 0 {
            local_8 = local_40.wrapping_sub(mech_trace::gr!("10008c56", "100a7178", DAT_100a7178));
        }
        mech_trace::cr!(
            "10008c56",
            "10035220",
            FUN_10035220(&raw mut local_34, i_var2, local_40, local_8 as u32)
        );
        if mech_trace::gr!("10008c56", "10215658", DAT_10215658) == 4 {
            mech_trace::gw!(
                "10008c56",
                "100aafc8",
                DAT_100aafc8,
                FUN_10009e05 as usize as u32
            );
            mech_trace::gw!(
                "10008c56",
                "100aafd0",
                DAT_100aafd0,
                FUN_10009edf as usize as u32
            );
            mech_trace::gw!(
                "10008c56",
                "100aafd4",
                DAT_100aafd4,
                FUN_1000a1d2 as usize as u32
            );
            let local_18 = mech_trace::cr!("10008c56", "1001a620", FUN_1001a620());
            mech_trace::cr!("10008c56", "1001a5e1", FUN_1001a5e1(6));
            mech_trace::cr!("10008c56", "10050b20", FUN_10050b20());
            mech_trace::cr!("10008c56", "1001a5e1", FUN_1001a5e1(local_18 as i32));
            mech_trace::cr!(
                "10008c56",
                "10058e74",
                FUN_10058e74(
                    param_1,
                    mech_trace::gr!("10008c56", "100ad6a4", DAT_100ad6a4) as u8
                )
            );
            mech_trace::cr!("10008c56", "100353df", FUN_100353df(0));
        }
        if mech_trace::gr!("10008c56", "10215658", DAT_10215658) != 4
            || (mech_trace::rd!(
                "10008c56",
                ((i_var1 as u32).wrapping_add(0x14) as *const u16),
                read_unaligned
            ) & 0x16)
                == 0
        {
            mech_trace::cr!(
                "10008c56",
                "10009896",
                FUN_10009896(param_1, local_38 as i32, local_10v)
            );
        }
        mech_trace::cr!("10008c56", "10008ed5", FUN_10008ed5(local_38));
        mech_trace::cr!("10008c56", "1003545c", FUN_1003545c());
        mech_trace::gw!("10008c56", "10215658", DAT_10215658, local_3c);
        mech_trace::gw!("10008c56", "100a7310", DAT_100a7310, 0);
    }
}
