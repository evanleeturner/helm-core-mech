// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0a0: u32;
    static mut DAT_100af0a4: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100c7b60: u8;
    static mut DAT_100c7b70: u32;
    static mut DAT_100c7b78: u8;
    static mut DAT_100c7b88: u32;
    static mut DAT_100c7b90: u8;
    static mut DAT_100c7ba0: u8;
    static mut DAT_100c7bb0: u8;
    static mut DAT_100c7bd8: u8;
    static mut DAT_100c7cd8: u8;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e768(param_1: i32);
    fn FUN_1001b024(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
    fn FUN_10067af1(param_1: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003e5c0() -> u32 {
    mech_trace::enter!("1003e5c0");
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: all globals and callees belong to the game; the table and
    // output pointer are the game's own memory, accessed where the original does.
    unsafe {
        if (mech_trace::gr!("1003e5c0", "100c7b70", DAT_100c7b70) as i32) < 0 && false {
            mech_trace::gw!("1003e5c0", "100af0a4", DAT_100af0a4, 0);
        }
        if mech_trace::gr!("1003e5c0", "100af0a0", DAT_100af0a0)
            <= mech_trace::gr!("1003e5c0", "100c7b70", DAT_100c7b70)
        {
            mech_trace::gw!("1003e5c0", "100af0a4", DAT_100af0a4, 0);
        }
        if mech_trace::gr!("1003e5c0", "100af0a4", DAT_100af0a4) == 0 {
            return mech_trace::rt!("1003e5c0", 0);
        }
        mech_trace::gw!(
            "1003e5c0",
            "100c7b88",
            DAT_100c7b88,
            mech_trace::gr!("1003e5c0", "100c7b88", DAT_100c7b88).wrapping_add(mech_trace::gr!(
                "1003e5c0",
                "100b3904",
                DAT_100b3904
            ))
        );
        let off = mech_trace::gr!("1003e5c0", "100c7b70", DAT_100c7b70).wrapping_mul(0x1c) as usize;
        let lim = mech_trace::rd!(
            "1003e5c0",
            (&raw mut DAT_100c7bd8).byte_add(off).cast::<i32>(),
            read_unaligned
        );
        if lim <= mech_trace::gr!("1003e5c0", "100c7b88", DAT_100c7b88) as i32 {
            mech_trace::gw!("1003e5c0", "100c7b88", DAT_100c7b88, 0);
            mech_trace::gw!(
                "1003e5c0",
                "100c7b70",
                DAT_100c7b70,
                mech_trace::gr!("1003e5c0", "100c7b70", DAT_100c7b70).wrapping_add(1)
            );
            if mech_trace::gr!("1003e5c0", "100c7b70", DAT_100c7b70)
                < mech_trace::gr!("1003e5c0", "100af0a0", DAT_100af0a0)
            {
                FUN_1003e768(mech_trace::gr!("1003e5c0", "100c7b70", DAT_100c7b70) as i32);
            }
        }
        FUN_1001b024(
            &raw mut local_10,
            &raw mut local_14,
            &raw mut local_18,
            &raw mut local_1c,
            &raw mut local_8,
            &raw mut local_c,
        );
        let mut i_var2 = mech_trace::cr!(
            "1003e5c0",
            "10067af1",
            FUN_10067af1((&raw mut DAT_100c7ba0).cast::<i32>())
        );
        mech_trace::wr!(
            "1003e5c0",
            (mech_trace::gr!("1003e5c0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as *mut u8)
                .cast::<i32>(),
            write_unaligned,
            local_1c.wrapping_add(i_var2)
        );
        i_var2 = mech_trace::cr!(
            "1003e5c0",
            "10067af1",
            FUN_10067af1((&raw mut DAT_100c7cd8).cast::<i32>())
        );
        mech_trace::wr!(
            "1003e5c0",
            (mech_trace::gr!("1003e5c0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as *mut u8)
                .byte_add(4)
                .cast::<i32>(),
            write_unaligned,
            local_8.wrapping_add(i_var2)
        );
        i_var2 = mech_trace::cr!(
            "1003e5c0",
            "10067af1",
            FUN_10067af1((&raw mut DAT_100c7b78).cast::<i32>())
        );
        mech_trace::wr!(
            "1003e5c0",
            (mech_trace::gr!("1003e5c0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as *mut u8)
                .byte_add(8)
                .cast::<i32>(),
            write_unaligned,
            local_c.wrapping_add(i_var2)
        );
        i_var2 = mech_trace::cr!(
            "1003e5c0",
            "10067af1",
            FUN_10067af1((&raw mut DAT_100c7bb0).cast::<i32>())
        );
        mech_trace::wr!(
            "1003e5c0",
            (mech_trace::gr!("1003e5c0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as *mut u8)
                .byte_add(0x10)
                .cast::<i32>(),
            write_unaligned,
            local_10.wrapping_add(i_var2)
        );
        i_var2 = mech_trace::cr!(
            "1003e5c0",
            "10067af1",
            FUN_10067af1((&raw mut DAT_100c7b60).cast::<i32>())
        );
        mech_trace::wr!(
            "1003e5c0",
            (mech_trace::gr!("1003e5c0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as *mut u8)
                .byte_add(0xc)
                .cast::<i32>(),
            write_unaligned,
            local_14.wrapping_add(i_var2)
        );
        i_var2 = mech_trace::cr!(
            "1003e5c0",
            "10067af1",
            FUN_10067af1((&raw mut DAT_100c7b90).cast::<i32>())
        );
        mech_trace::wr!(
            "1003e5c0",
            (mech_trace::gr!("1003e5c0", "PTR_DAT_100aaf68", PTR_DAT_100aaf68) as *mut u8)
                .byte_add(0x14)
                .cast::<i32>(),
            write_unaligned,
            local_18.wrapping_add(i_var2)
        );
        mech_trace::rt!("1003e5c0", 1)
    }
}
