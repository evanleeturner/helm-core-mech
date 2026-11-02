// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160eb0: i32;
    static mut DAT_10160ebc: i32;
    static mut DAT_1011515c: i32;
    static mut DAT_10115160: i32;
    static mut DAT_10160f48: i32;
    static mut DAT_10160f44: i32;
    static mut DAT_10160f4c: i32;
    static mut DAT_10160f0c: i32;
    static mut DAT_10160f10: i32;
    static mut DAT_10160f14: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a080(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_10080241(a: i32, b: i32) -> *mut u8;
    fn FUN_1008044f(a: i32, b: i32) -> *mut u8;
    fn FUN_10080639(p: *mut u8);
    fn FUN_10080130(a: i32) -> *mut u8;
}

unsafe fn rd(base: i32, off: i32) -> i32 {
    // SAFETY: caller guarantees the address is a valid game vertex record.
    unsafe {
        mech_trace::rd!(
            "10017bc1",
            (base.wrapping_add(off) as *const i32),
            read_unaligned
        )
    }
}

unsafe fn rb(base: i32, off: i32) -> i8 {
    // SAFETY: caller guarantees the address is a valid game vertex record.
    unsafe { mech_trace::rd!("10017bc1", (base.wrapping_add(off) as *const i8), read) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017bc1(
    param_1: i32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    mech_trace::enter!("10017bc1", param_1, param_2, param_3, param_4);
    // SAFETY: globals and callees are the game's own; param_1 points at an
    // array of 0x44-byte records, accessed where the original does.
    unsafe {
        let u_var2 = mech_trace::gr!("10017bc1", "10160ebc", DAT_10160ebc);
        let u_var1 = mech_trace::gr!("10017bc1", "10160eb0", DAT_10160eb0);
        let u_var5: i32;
        if mech_trace::gr!("10017bc1", "1011515c", DAT_1011515c) == 0 {
            u_var5 = 0;
        } else {
            mech_trace::gw!("10017bc1", "10115160", DAT_10115160, 0);
            let mut local_10 = param_1;
            mech_trace::gw!(
                "10017bc1",
                "10160eb0",
                DAT_10160eb0,
                param_3.wrapping_shl(2)
            );
            mech_trace::gw!(
                "10017bc1",
                "10160ebc",
                DAT_10160ebc,
                param_4.wrapping_shl(2)
            );
            let mut local_14: u32 = 0;
            while local_14 < param_2 {
                let i_var3 = mech_trace::cr!(
                    "10017bc1",
                    "1001a080",
                    FUN_1001a080(
                        rd(local_10, 0xc).wrapping_sub(mech_trace::gr!(
                            "10017bc1",
                            "10160f48",
                            DAT_10160f48
                        )),
                        mech_trace::gr!("10017bc1", "10160f0c", DAT_10160f0c),
                        rd(local_10, 0x10).wrapping_sub(mech_trace::gr!(
                            "10017bc1",
                            "10160f44",
                            DAT_10160f44
                        )),
                        mech_trace::gr!("10017bc1", "10160f10", DAT_10160f10),
                        rd(local_10, 0x14).wrapping_sub(mech_trace::gr!(
                            "10017bc1",
                            "10160f4c",
                            DAT_10160f4c
                        )),
                        mech_trace::gr!("10017bc1", "10160f14", DAT_10160f14),
                    )
                );
                mech_trace::wr!(
                    "10017bc1",
                    (local_10.wrapping_add(0x20) as *mut i32),
                    write_unaligned,
                    i_var3
                );
                if mech_trace::gr!("10017bc1", "10160eb0", DAT_10160eb0) < rd(local_10, 0x20) {
                    if param_4 < 1
                        || rd(local_10, 0x20)
                            < mech_trace::gr!("10017bc1", "10160ebc", DAT_10160ebc)
                    {
                        mech_trace::wr!(
                            "10017bc1",
                            (local_10.wrapping_add(0x28) as *mut i8),
                            write,
                            0
                        );
                    } else {
                        mech_trace::wr!(
                            "10017bc1",
                            (local_10.wrapping_add(0x28) as *mut i8),
                            write,
                            2
                        );
                    }
                } else {
                    mech_trace::wr!(
                        "10017bc1",
                        (local_10.wrapping_add(0x28) as *mut i8),
                        write,
                        1
                    );
                }
                local_10 = local_10.wrapping_add(0x44);
                local_14 = local_14.wrapping_add(1);
            }
            local_10 = param_1;
            let mut local_24 =
                (param_2.wrapping_sub(1).wrapping_mul(0x44) as i32).wrapping_add(param_1);
            local_14 = 0;
            while local_14 < param_2 && mech_trace::gr!("10017bc1", "1011515c", DAT_1011515c) != 0 {
                if rb(local_24, 0x28) != rb(local_10, 0x28) {
                    if rb(local_10, 0x28) == 0 {
                        let p = if rb(local_24, 0x28) == 1 {
                            mech_trace::cr!(
                                "10017bc1",
                                "10080241",
                                FUN_10080241(local_24, local_10)
                            )
                        } else {
                            mech_trace::cr!(
                                "10017bc1",
                                "1008044f",
                                FUN_1008044f(local_24, local_10)
                            )
                        };
                        FUN_10080639(p);
                    } else if rb(local_24, 0x28) == 0 {
                        let p = if rb(local_10, 0x28) == 1 {
                            mech_trace::cr!(
                                "10017bc1",
                                "10080241",
                                FUN_10080241(local_24, local_10)
                            )
                        } else {
                            mech_trace::cr!(
                                "10017bc1",
                                "1008044f",
                                FUN_1008044f(local_24, local_10)
                            )
                        };
                        FUN_10080639(p);
                    } else if rd(local_10, 0x20) < rd(local_24, 0x20) {
                        let p = mech_trace::cr!(
                            "10017bc1",
                            "1008044f",
                            FUN_1008044f(local_24, local_10)
                        );
                        FUN_10080639(p);
                        let p = mech_trace::cr!(
                            "10017bc1",
                            "10080241",
                            FUN_10080241(local_24, local_10)
                        );
                        FUN_10080639(p);
                    } else {
                        let p = mech_trace::cr!(
                            "10017bc1",
                            "10080241",
                            FUN_10080241(local_24, local_10)
                        );
                        FUN_10080639(p);
                        let p = mech_trace::cr!(
                            "10017bc1",
                            "1008044f",
                            FUN_1008044f(local_24, local_10)
                        );
                        FUN_10080639(p);
                    }
                }
                if rb(local_10, 0x28) == 0 {
                    let p = mech_trace::cr!("10017bc1", "10080130", FUN_10080130(local_10));
                    if mech_trace::gr!("10017bc1", "1011515c", DAT_1011515c) != 0 {
                        FUN_10080639(p);
                    }
                }
                local_24 = local_10;
                local_10 = local_10.wrapping_add(0x44);
                local_14 = local_14.wrapping_add(1);
            }
            let mut r = mech_trace::gr!("10017bc1", "10115160", DAT_10115160);
            if mech_trace::gr!("10017bc1", "1011515c", DAT_1011515c) == 0 {
                r = 0;
            }
            u_var5 = r;
        }
        mech_trace::gw!("10017bc1", "10160eb0", DAT_10160eb0, u_var1);
        mech_trace::gw!("10017bc1", "10160ebc", DAT_10160ebc, u_var2);
        mech_trace::rt!("10017bc1", u_var5)
    }
}
