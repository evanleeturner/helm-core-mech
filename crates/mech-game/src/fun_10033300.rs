// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adb00: i32;
    static mut DAT_100adb04: i32;
    static mut DAT_100adb08: i32;
    static mut DAT_100adb0c: i32;
    static mut DAT_100adb10: i32;
    static mut DAT_100adb14: *mut i8;
    static mut DAT_100adb18: i8;
    static mut DAT_100adb4c: i8;
    static mut DAT_100adb9c: i8;
    static mut DAT_100adbc0: i8;
    static mut DAT_100adb50: i8;
    static mut DAT_100adb60: i8;
    static mut DAT_100adb68: i8;
    static mut DAT_100adba0: i8;
    static mut DAT_100adba8: i8;
    static mut DAT_100adbb0: i8;
    static mut DAT_100adbc4: i8;
    static mut DAT_100adbcc: i8;
    static mut DAT_100adbdc: i8;
    static mut DAT_100adf74: *mut u8;
    static mut DAT_100adf80: i32;
    static mut DAT_100b9a18: *mut u32;
    static mut DAT_100b9a1c: i32;
    static mut DAT_100c967c: i32;
    static mut DAT_100c9680: i32;
    static mut DAT_101f5300: u32;
    static mut DAT_100c6b58: u32;
    static mut DAT_100c6b5c: u32;
    static mut _DAT_100c6b60: u32;
    static mut _DAT_100c6b40: u32;
    static mut DAT_100c6b40: u32;
    static mut DAT_100c6b4c: u32;
    static mut DAT_100c6b50: u32;
    static mut DAT_100c6b3c: i32;
    static mut DAT_100c6b38: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetCurrentDirectoryA(n: u32, buf: *mut i8) -> u32;
    fn HeapAlloc(heap: *mut u8, flags: u32, size: u32) -> *mut u8;
    fn HeapFree(heap: *mut u8, flags: u32, mem: *mut u8) -> i32;
    fn _FILE_read_8(name: *mut i8, a: i32) -> i32;
    fn _AIL_register_timer_4(f: *mut u8) -> i32;
    fn _AIL_set_timer_period_8(t: i32, p: i32);
    fn _AIL_start_timer_4(t: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_10081830() -> i8;
    fn FUN_10002168(s: *mut i8);
    fn FUN_10058e74(p: *mut i32, a: i32);
    fn FUN_10056fc4(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_1005b5ea(a: i32, b: i32, c: i32);
    fn FUN_1005b6c7(a: i32) -> i32;
    fn __ftol() -> i64;
    fn FUN_100337fe();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10033300(param_1: i32) {
    mech_trace::enter!("10033300", param_1);
    let mut local_204: [i8; 256] = [0; 256];
    let mut local_104: [i8; 256] = [0; 256];
    // SAFETY: globals and callees are the game's own; buffers are local and
    // large enough, function-pointer tables come from the game's structures.
    unsafe {
        mech_trace::cr!(
            "10033300",
            "GetCurrentDirectoryA",
            GetCurrentDirectoryA(0x100, local_204.as_mut_ptr())
        );
        let c_var1 = local_204[0];
        mech_trace::gw!("10033300", "100adb10", DAT_100adb10, 0);
        mech_trace::gw!("10033300", "100adb14", DAT_100adb14, core::ptr::null_mut());
        let off = (((mech_trace::gr!("10033300", "100aa1ac", DAT_100aa1ac) == 0) as i32)
            .wrapping_sub(1)
            & 8) as usize;
        mech_trace::cr!(
            "10033300",
            "_sprintf",
            _sprintf(
                local_204.as_mut_ptr(),
                (&raw const DAT_100adb68).cast::<i8>(),
                local_204[0] as i32,
                (&raw const DAT_100adb60).cast::<i8>(),
                (&raw const DAT_100adb50).cast::<i8>().add(off),
                &raw const DAT_100adb4c,
            )
        );
        if mech_trace::gr!("10033300", "100adb14", DAT_100adb14).is_null()
            || mech_trace::dr!(
                "10033300",
                mech_trace::gr!("10033300", "100adb14", DAT_100adb14)
            ) == 0
        {
            mech_trace::cr!(
                "10033300",
                "_sprintf",
                _sprintf(
                    local_104.as_mut_ptr(),
                    (&raw const DAT_100adbb0).cast::<i8>(),
                    c_var1 as i32,
                    (&raw const DAT_100adba8).cast::<i8>(),
                    (&raw const DAT_100adba0).cast::<i8>(),
                    &raw const DAT_100adb9c,
                )
            );
        } else {
            mech_trace::cr!(
                "10033300",
                "_sprintf",
                _sprintf(
                    local_104.as_mut_ptr(),
                    (&raw const DAT_100adbcc).cast::<i8>(),
                    c_var1 as i32,
                    (&raw const DAT_100adbc4).cast::<i8>(),
                    mech_trace::gr!("10033300", "100adb14", DAT_100adb14),
                    &raw const DAT_100adbc0,
                )
            );
        }
        mech_trace::gw!(
            "10033300",
            "100adb04",
            DAT_100adb04,
            mech_trace::cr!(
                "10033300",
                "_FILE_read_8",
                _FILE_read_8(local_204.as_mut_ptr(), 0)
            )
        );
        let mut ok = mech_trace::gr!("10033300", "100adb04", DAT_100adb04) != 0;
        if !ok {
            local_204[0] = mech_trace::cr!("10033300", "10081830", FUN_10081830());
            if local_204[0] != 0 {
                local_104[0] = local_204[0];
                mech_trace::gw!(
                    "10033300",
                    "100adb04",
                    DAT_100adb04,
                    mech_trace::cr!(
                        "10033300",
                        "_FILE_read_8",
                        _FILE_read_8(local_204.as_mut_ptr(), 0)
                    )
                );
                ok = mech_trace::gr!("10033300", "100adb04", DAT_100adb04) != 0;
            }
        }
        if !ok {
            return;
        }
        let lp_mem = mech_trace::cr!(
            "10033300",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10033300", "100adf74", DAT_100adf74),
                9,
                0x300
            )
        );
        if lp_mem.is_null() {
            return;
        }
        let f5: unsafe extern "C" fn(i32, i32, *mut u8, i32) =
            core::mem::transmute(mech_trace::rd!(
                "10033300",
                mech_trace::gr!("10033300", "100b9a18", DAT_100b9a18).add(5),
                read_unaligned
            ));
        mech_trace::cr!("10033300", "ind", f5(0, 0x100, lp_mem, 1));
        let mut src = &raw mut DAT_101f5300;
        let mut dst = &raw mut DAT_100c6b58;
        let mut i_var2 = 5;
        while i_var2 != 0 {
            mech_trace::wr!(
                "10033300",
                dst,
                write_unaligned,
                mech_trace::rd!("10033300", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            i_var2 -= 1;
        }
        mech_trace::gw!(
            "10033300",
            "100c6b5c",
            DAT_100c6b5c,
            (mech_trace::gr!("10033300", "100c967c", DAT_100c967c) as u32).wrapping_sub(1)
        );
        mech_trace::gw!(
            "10033300",
            "_DAT_100c6b60",
            _DAT_100c6b60,
            (mech_trace::gr!("10033300", "100c9680", DAT_100c9680) as u32).wrapping_sub(1)
        );
        mech_trace::gw!(
            "10033300",
            "_DAT_100c6b40",
            _DAT_100c6b40,
            (&raw mut DAT_100c6b58) as usize as u32
        );
        mech_trace::gw!(
            "10033300",
            "100c6b4c",
            DAT_100c6b4c,
            (mech_trace::gr!("10033300", "100c967c", DAT_100c967c) as u32).wrapping_sub(1)
        );
        mech_trace::gw!(
            "10033300",
            "100c6b50",
            DAT_100c6b50,
            (mech_trace::gr!("10033300", "100c9680", DAT_100c9680) as u32).wrapping_sub(1)
        );
        let local_214: i32 = if mech_trace::gr!("10033300", "100adf80", DAT_100adf80) == 0 {
            -1
        } else {
            let f8: unsafe extern "C" fn() -> i32 = core::mem::transmute(mech_trace::rd!(
                "10033300",
                mech_trace::gr!("10033300", "100b9a18", DAT_100b9a18).add(8),
                read_unaligned
            ));
            mech_trace::cr!("10033300", "ind", f8())
        };
        if local_214 == 0 {
            mech_trace::gw!(
                "10033300",
                "100c6b58",
                DAT_100c6b58,
                mech_trace::gr!("10033300", "101f5300", DAT_101f5300)
            );
            FUN_10058e74((&raw mut DAT_100c6b40).cast::<i32>(), 0);
            FUN_10056fc4(
                (&raw mut DAT_100c6b40).cast::<i32>(),
                mech_trace::gr!("10033300", "100adb04", DAT_100adb04),
                0,
                0,
                0,
            );
            if mech_trace::gr!("10033300", "100adf80", DAT_100adf80) != 0 {
                let g: unsafe extern "C" fn() = core::mem::transmute(mech_trace::rd!(
                    "10033300",
                    ((mech_trace::gr!("10033300", "100b9a1c", DAT_100b9a1c).wrapping_add(0x18))
                        as usize as *const u32),
                    read_unaligned
                ));
                mech_trace::cr!("10033300", "ind", g());
            }
            FUN_1005b5ea(
                mech_trace::gr!("10033300", "100adb04", DAT_100adb04),
                0,
                lp_mem as usize as i32,
            );
            let f7: unsafe extern "C" fn(*mut u8, i32) = core::mem::transmute(mech_trace::rd!(
                "10033300",
                mech_trace::gr!("10033300", "100b9a18", DAT_100b9a18).add(7),
                read_unaligned
            ));
            if param_1 == 0 {
                mech_trace::cr!("10033300", "ind", f7(lp_mem, 2));
            } else {
                mech_trace::cr!("10033300", "ind", f7(lp_mem, 0x3c));
            }
            if mech_trace::gr!("10033300", "100aa1ac", DAT_100aa1ac) == 0 {
                let f5b: unsafe extern "C" fn(i32, i32, *mut i8, i32) =
                    core::mem::transmute(mech_trace::rd!(
                        "10033300",
                        mech_trace::gr!("10033300", "100b9a18", DAT_100b9a18).add(5),
                        read_unaligned
                    ));
                mech_trace::cr!("10033300", "ind", f5b(0, 0x10, &raw mut DAT_100adb18, 1));
            }
            mech_trace::cr!(
                "10033300",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10033300", "100adf74", DAT_100adf74),
                    1,
                    lp_mem
                )
            );
            mech_trace::gw!(
                "10033300",
                "100adb08",
                DAT_100adb08,
                mech_trace::cr!(
                    "10033300",
                    "_FILE_read_8",
                    _FILE_read_8(local_104.as_mut_ptr(), 0)
                )
            );
            if mech_trace::gr!("10033300", "100adb08", DAT_100adb08) != 0 {
                mech_trace::gw!(
                    "10033300",
                    "100adb0c",
                    DAT_100adb0c,
                    mech_trace::cr!(
                        "10033300",
                        "1005b6c7",
                        FUN_1005b6c7(mech_trace::gr!("10033300", "100adb08", DAT_100adb08))
                    )
                );
                let l = mech_trace::cr!("10033300", "__ftol", __ftol());
                mech_trace::gw!("10033300", "100c6b3c", DAT_100c6b3c, l as i32);
                let l = mech_trace::cr!("10033300", "__ftol", __ftol());
                mech_trace::gw!("10033300", "100c6b38", DAT_100c6b38, l as i32);
                if mech_trace::gr!("10033300", "100aa1ac", DAT_100aa1ac) == 0
                    && mech_trace::rd!(
                        "10033300",
                        mech_trace::gr!("10033300", "100b9a18", DAT_100b9a18),
                        read_unaligned
                    ) == 1
                {
                    let local_218: i32 =
                        if mech_trace::gr!("10033300", "100adf80", DAT_100adf80) == 0 {
                            -1
                        } else {
                            let f8b: unsafe extern "C" fn() -> i32 =
                                core::mem::transmute(mech_trace::rd!(
                                    "10033300",
                                    mech_trace::gr!("10033300", "100b9a18", DAT_100b9a18).add(8),
                                    read_unaligned
                                ));
                            mech_trace::cr!("10033300", "ind", f8b())
                        };
                    if local_218 == 0 {
                        FUN_10056fc4(
                            (&raw mut DAT_100c6b40).cast::<i32>(),
                            mech_trace::gr!("10033300", "100adb04", DAT_100adb04),
                            0,
                            0,
                            0,
                        );
                        FUN_10056fc4(
                            (&raw mut DAT_100c6b40).cast::<i32>(),
                            mech_trace::gr!("10033300", "100adb08", DAT_100adb08),
                            0,
                            mech_trace::gr!("10033300", "100c6b3c", DAT_100c6b3c),
                            mech_trace::gr!("10033300", "100c6b38", DAT_100c6b38),
                        );
                        if mech_trace::gr!("10033300", "100adf80", DAT_100adf80) != 0 {
                            let g: unsafe extern "C" fn() = core::mem::transmute(mech_trace::rd!(
                                "10033300",
                                ((mech_trace::gr!("10033300", "100b9a1c", DAT_100b9a1c)
                                    .wrapping_add(0x18)) as usize
                                    as *const u32),
                                read_unaligned
                            ));
                            mech_trace::cr!("10033300", "ind", g());
                        }
                    }
                } else if mech_trace::gr!("10033300", "100aa1ac", DAT_100aa1ac) == 0 {
                    mech_trace::gw!(
                        "10033300",
                        "100adb00",
                        DAT_100adb00,
                        mech_trace::cr!(
                            "10033300",
                            "_AIL_register_timer_4",
                            _AIL_register_timer_4(
                                (FUN_100337fe as unsafe extern "C" fn()) as usize as *mut u8,
                            )
                        )
                    );
                    _AIL_set_timer_period_8(
                        mech_trace::gr!("10033300", "100adb00", DAT_100adb00),
                        330000,
                    );
                    _AIL_start_timer_4(mech_trace::gr!("10033300", "100adb00", DAT_100adb00));
                }
            }
        } else {
            FUN_10002168((&raw mut DAT_100adbdc).cast::<i8>());
        }
    }
}
