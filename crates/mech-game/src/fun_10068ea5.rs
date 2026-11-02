// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a1338: i32;
    static mut DAT_100c7fac: i32;
    static mut DAT_100c7fb4: i32;
    static mut _DAT_100b3984: i32;
    static mut DAT_100b3988: i32;
    static mut DAT_100c2dec: i32;
    static mut DAT_100c2be0: *const u8;
    static mut _DAT_100b3980: i32;
    static mut DAT_100b3998: i32;
    static mut _DAT_100b398c: i32;
    static mut DAT_100b399c: i32;
    static mut _DAT_100b3990: i32;
    static mut DAT_100b39a0: i32;
    static mut _DAT_100b3994: i32;
    static mut DAT_100b39e4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn strchr(s: *const i8, c: i32) -> *mut i8;
    fn _isctype(c: i32, mask: i32) -> i32;
    fn atol(s: *const i8) -> i32;
    fn FUN_1001c5d1(a: i32, b: i32, c: i32, d: *mut i32) -> *mut u16;
    fn FUN_1001c509(a: i32, b: i32);
    fn FUN_1001c303(a: i32, b: i32, c: i32, d: i32, e: i32, f: *mut i32) -> *mut i32;
    fn FUN_1001c72d(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
    fn FUN_100696b1(a: u32) -> u32;
    fn FUN_1001c7df(a: i32, b: i16, c: i32) -> *mut u16;
    fn FUN_1001c85e(a: i32, b: i32, c: i8);
    fn FUN_1001cc6a(a: i32);
    fn FUN_1002ff96(a: i32, b: i32) -> *mut i32;
    fn FUN_1002f010(a: i32, b: i32) -> *mut i32;
    fn FUN_1002f37a(a: i32, b: i32, c: i32, d: i32);
    fn FUN_1002f272(a: i32, b: i32);
    fn FUN_1001d5e1(a: i32, b: *mut i32);
    fn FUN_1002fbc0(a: *mut i32);
    fn FUN_1003da99(a: *mut u16);
    fn FUN_100141c0(a: *mut u16, b: i32);
    fn FUN_1001ca37(a: i32, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068ea5(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: i32,
    param_5: *mut i32,
) -> u32 {
    mech_trace::enter!("10068ea5", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the globals and callees are the game's own; the pointers are
    // the caller's buffers, read and written where the original does.
    unsafe {
        let mut local_14: i32 = 0;
        let mut local_c: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_94: *mut i32 = core::ptr::null_mut();
        let mut local_24: *mut i32;

        if mech_trace::dr!("10068ea5", param_3) == 0 {
            mech_trace::gw!("10068ea5", "100c7fac", DAT_100c7fac, 0);
            mech_trace::gw!("10068ea5", "100c7fb4", DAT_100c7fb4, 0);
            mech_trace::gw!("10068ea5", "100b3984", _DAT_100b3984, 0);
            mech_trace::gw!("10068ea5", "100b3988", DAT_100b3988, 0);
        }
        let pi2 = (mech_trace::dr!("10068ea5", param_2)).wrapping_add(param_1) as usize as *mut i32;
        if mech_trace::rd!("10068ea5", pi2, read_unaligned)
            != mech_trace::gr!("10068ea5", "100a1338", DAT_100a1338)
        {
            mech_trace::gw!("10068ea5", "100b3980", _DAT_100b3980, 0xfffffffe_u32 as i32);
            return mech_trace::rt!("10068ea5", 0xffffffff);
        }
        let flags = mech_trace::rd!("10068ea5", pi2.add(7).cast::<u16>(), read_unaligned);
        if (flags & 0x2000) == 0 {
            if (flags & 0x1000) != 0 {
                mech_trace::gw!("10068ea5", "100c7fb4", DAT_100c7fb4, 1);
            }
        } else {
            mech_trace::gw!("10068ea5", "100c7fac", DAT_100c7fac, 1);
        }
        let f2 = mech_trace::rd!("10068ea5", pi2.add(7).cast::<u16>(), read_unaligned) & 0xcfff;
        if f2 == 1 {
            mech_trace::gw!("10068ea5", "100b3984", _DAT_100b3984, 0);
            mech_trace::gw!("10068ea5", "100b3988", DAT_100b3988, 1);
        } else if f2 == 2 {
            mech_trace::gw!("10068ea5", "100b3988", DAT_100b3988, 0);
            mech_trace::gw!("10068ea5", "100b3984", _DAT_100b3984, 1);
        } else {
            mech_trace::gw!("10068ea5", "100b3988", DAT_100b3988, 0);
            mech_trace::gw!("10068ea5", "100b3984", _DAT_100b3984, 0);
        }
        let i_var_sign = mech_trace::rd!("10068ea5", pi2.add(2).cast::<i8>(), read) as i32;
        if 0x80 < i_var_sign {
            let mut i: i32 = 0;
            while i < 0x10 {
                let p = pi2.cast::<u8>().byte_add(i as usize + 8).cast::<i8>();
                let v = mech_trace::rd!("10068ea5", p, read) as i32 as u32;
                mech_trace::wr!("10068ea5", p, write, (0u32.wrapping_sub(v)) as i8);
                i += 1;
            }
        }
        let mut local_68: [i8; 40] = [0; 40];
        mech_trace::cr!(
            "10068ea5",
            "strncpy",
            strncpy(local_68.as_mut_ptr(), pi2.add(2).cast::<i8>(), 0x10)
        );
        local_14 = 0;
        if mech_trace::gr!("10068ea5", "100c7fb4", DAT_100c7fb4) != 0 {
            let mut local_8 =
                mech_trace::cr!("10068ea5", "strchr", strchr(local_68.as_ptr(), 0x5f));
            if !local_8.is_null() {
                mech_trace::dw!("10068ea5", local_8, 0);
                local_8 = local_8.add(1);
                let local_98: u32 = if mech_trace::gr!("10068ea5", "100c2dec", DAT_100c2dec) < 2 {
                    let c = mech_trace::dr!("10068ea5", local_8) as i32 as isize;
                    (mech_trace::rd!(
                        "10068ea5",
                        mech_trace::gr!("10068ea5", "100c2be0", DAT_100c2be0)
                            .offset(c.wrapping_mul(2))
                            .cast::<u16>(),
                        read_unaligned
                    ) as u32)
                        & 4
                } else {
                    mech_trace::cr!(
                        "10068ea5",
                        "_isctype",
                        _isctype(mech_trace::dr!("10068ea5", local_8) as i32, 4)
                    ) as u32
                };
                if local_98 != 0 {
                    local_14 = mech_trace::cr!("10068ea5", "atol", atol(local_8));
                }
            }
        }
        let local_2c = pi2.add(8);
        let cnt = mech_trace::rd!("10068ea5", pi2.add(6).cast::<i16>(), read_unaligned);
        let puvar3 = local_2c.offset(cnt as isize * 4).cast::<u16>();
        let mut local_20: i32 = 0;
        let rec_count = mech_trace::rd!(
            "10068ea5",
            pi2.cast::<u8>().byte_add(0x1a).cast::<i16>(),
            read_unaligned
        );
        let mut local_7c: i32 = rec_count as i32;
        let mut local_70: *mut u16 = puvar3;
        while local_7c != 0 {
            let local_28 = mech_trace::rd!("10068ea5", local_70.add(1), read_unaligned) as u32;
            let local_6c: usize = if local_28 < 5 { 0xc } else { 0x12 };
            local_20 = (local_20 as u32).wrapping_add(local_28.wrapping_mul(4)) as i32;
            local_70 = local_70.byte_add(local_6c);
            local_7c -= 1;
        }
        let local_38: i32 = if mech_trace::gr!("10068ea5", "100c7fac", DAT_100c7fac) == 0 {
            cnt as i32
        } else {
            (cnt as i32).wrapping_sub(1)
        };
        if mech_trace::dr!("10068ea5", param_3) == 0
            || mech_trace::gr!("10068ea5", "100c7fac", DAT_100c7fac) != 0
        {
            let p4 = mech_trace::cr!(
                "10068ea5",
                "1001c5d1",
                FUN_1001c5d1(local_38, rec_count as i32, local_20, &raw mut local_1c)
            );
            mech_trace::dw!("10068ea5", param_3, p4 as usize as i32);
            if mech_trace::dr!("10068ea5", param_3) == 0 {
                mech_trace::gw!("10068ea5", "100b3980", _DAT_100b3980, 0xfffffffe_u32 as i32);
                return mech_trace::rt!("10068ea5", 0xffffffff);
            }
            FUN_1001c509(mech_trace::dr!("10068ea5", param_3), local_14);
        } else if mech_trace::gr!("10068ea5", "100c7fb4", DAT_100c7fb4) != 0 {
            let p5 = mech_trace::cr!(
                "10068ea5",
                "1001c303",
                FUN_1001c303(
                    mech_trace::dr!("10068ea5", param_3),
                    local_14,
                    local_38,
                    rec_count as i32,
                    local_20,
                    &raw mut local_1c,
                )
            );
            if p5.is_null() {
                mech_trace::gw!("10068ea5", "100b3980", _DAT_100b3980, 0xfffffffe_u32 as i32);
                return mech_trace::rt!("10068ea5", 0xffffffff);
            }
        }
        local_7c = 0;
        while local_7c
            < mech_trace::rd!("10068ea5", pi2.add(6).cast::<i16>(), read_unaligned) as i32
        {
            local_94 = local_2c.offset(local_7c as isize * 4);
            let local_40 = mech_trace::rd!("10068ea5", local_94, read_unaligned);
            let local_34 =
                mech_trace::rd!("10068ea5", local_94.add(3).cast::<i16>(), read_unaligned) as i32;
            let local_3c = mech_trace::rd!(
                "10068ea5",
                local_94.cast::<u8>().byte_add(0xe).cast::<i16>(),
                read_unaligned
            ) as i32;
            if local_7c < local_38 {
                FUN_1001c72d(
                    mech_trace::dr!("10068ea5", param_3),
                    (mech_trace::gr!("10068ea5", "100b3998", DAT_100b3998) as u32)
                        .wrapping_mul(local_40 as u32)
                        .wrapping_add(mech_trace::gr!("10068ea5", "100b398c", _DAT_100b398c) as u32)
                        as i32,
                    (mech_trace::gr!("10068ea5", "100b399c", DAT_100b399c) as u32)
                        .wrapping_mul(
                            mech_trace::rd!("10068ea5", local_94.add(1), read_unaligned) as u32
                        )
                        .wrapping_add(mech_trace::gr!("10068ea5", "100b3990", _DAT_100b3990) as u32)
                        as i32,
                    (mech_trace::rd!("10068ea5", local_94.add(2), read_unaligned) as u32)
                        .wrapping_mul(mech_trace::gr!("10068ea5", "100b39a0", DAT_100b39a0) as u32)
                        .wrapping_add(mech_trace::gr!("10068ea5", "100b3994", _DAT_100b3994) as u32)
                        as i32,
                    local_34,
                    local_3c,
                );
            }
            let u_var1 = (local_c as u32)
                .wrapping_add(mech_trace::rd!("10068ea5", local_94, read_unaligned) as u32)
                .wrapping_add(mech_trace::rd!("10068ea5", local_94.add(1), read_unaligned) as u32)
                .wrapping_add(mech_trace::rd!("10068ea5", local_94.add(2), read_unaligned) as u32)
                .wrapping_add(mech_trace::rd!(
                    "10068ea5",
                    local_94.add(3).cast::<i16>(),
                    read_unaligned
                ) as i32 as u32)
                .wrapping_add(mech_trace::rd!(
                    "10068ea5",
                    local_94.cast::<u8>().byte_add(0xe).cast::<i16>(),
                    read_unaligned
                ) as i32 as u32);
            let u_var7 = ((u_var1 as i32) >> 0x1f) as u32;
            local_c = (((u_var1 ^ u_var7).wrapping_sub(u_var7) & 0xfffff) ^ u_var7)
                .wrapping_sub(u_var7) as i32;
            local_7c += 1;
        }
        let mut local_18: i32 = 0;
        local_7c = 0;
        local_70 = puvar3;
        loop {
            if (mech_trace::rd!(
                "10068ea5",
                pi2.cast::<u8>().byte_add(0x1a).cast::<i16>(),
                read_unaligned
            ) as i32)
                <= local_7c
            {
                mech_trace::dw!(
                    "10068ea5",
                    param_2,
                    (mech_trace::dr!("10068ea5", param_2) as u32).wrapping_add(0x20) as i32
                );
                let c6 =
                    mech_trace::rd!("10068ea5", pi2.add(6).cast::<i16>(), read_unaligned) as i32;
                mech_trace::dw!(
                    "10068ea5",
                    param_2,
                    (c6.wrapping_mul(0x10) as u32)
                        .wrapping_add(mech_trace::dr!("10068ea5", param_2) as u32)
                        as i32
                );
                mech_trace::dw!(
                    "10068ea5",
                    param_2,
                    (mech_trace::dr!("10068ea5", param_2) as u32).wrapping_add(local_18 as u32)
                        as i32
                );
                if mech_trace::rd!("10068ea5", pi2.add(1), read_unaligned) != local_c {
                    mech_trace::gw!("10068ea5", "100b3980", _DAT_100b3980, 0xfffffffe_u32 as i32);
                    return mech_trace::rt!("10068ea5", 0xffffffff);
                }
                FUN_1001cc6a(mech_trace::dr!("10068ea5", param_3));
                mech_trace::gw!("10068ea5", "100b3980", _DAT_100b3980, 0);
                if mech_trace::gr!("10068ea5", "100c7fac", DAT_100c7fac) != 0 {
                    local_24 = core::ptr::null_mut();
                    let mut need = true;
                    if param_4 != 0 {
                        local_24 = mech_trace::cr!(
                            "10068ea5",
                            "1002ff96",
                            FUN_1002ff96(param_4, mech_trace::dr!("10068ea5", param_5))
                        );
                        if !local_24.is_null() {
                            need = false;
                        }
                    }
                    if need {
                        local_24 =
                            mech_trace::cr!("10068ea5", "1002f010", FUN_1002f010(param_4, 0x14));
                        if local_24.is_null() {
                            mech_trace::gw!(
                                "10068ea5",
                                "100b3980",
                                _DAT_100b3980,
                                0xfffffffe_u32 as i32
                            );
                            return mech_trace::rt!("10068ea5", 0xffffffff);
                        }
                    }
                    if mech_trace::rd!("10068ea5", local_24.add(0x1b), read_unaligned) != 0 {
                        local_24 =
                            mech_trace::cr!("10068ea5", "1002f010", FUN_1002f010(param_4, 0xc));
                        if local_24.is_null() {
                            mech_trace::gw!(
                                "10068ea5",
                                "100b3980",
                                _DAT_100b3980,
                                0xfffffffe_u32 as i32
                            );
                            return mech_trace::rt!("10068ea5", 0xffffffff);
                        }
                    }
                    FUN_1002f37a(
                        local_24 as usize as i32,
                        mech_trace::rd!("10068ea5", local_94, read_unaligned),
                        mech_trace::rd!("10068ea5", local_94.add(1), read_unaligned),
                        mech_trace::rd!("10068ea5", local_94.add(2), read_unaligned),
                    );
                    FUN_1002f272(
                        local_24 as usize as i32,
                        mech_trace::dr!("10068ea5", param_3),
                    );
                    FUN_1001d5e1(mech_trace::dr!("10068ea5", param_3), local_24);
                    FUN_1002fbc0(local_24);
                    if mech_trace::dr!("10068ea5", param_5) != 0 {
                        FUN_1003da99(mech_trace::dr!("10068ea5", param_3) as usize as *mut u16);
                        FUN_100141c0(
                            mech_trace::dr!("10068ea5", param_3) as usize as *mut u16,
                            mech_trace::gr!("10068ea5", "100b39e4", DAT_100b39e4),
                        );
                        FUN_1001ca37(mech_trace::dr!("10068ea5", param_3), 0x50);
                    }
                    mech_trace::dw!(
                        "10068ea5",
                        param_5,
                        (mech_trace::dr!("10068ea5", param_5) as u32).wrapping_add(1) as i32
                    );
                }
                return mech_trace::rt!("10068ea5", 0);
            }
            let local_28 = mech_trace::rd!("10068ea5", local_70.add(1), read_unaligned) as u32;
            let local_6c: i32 = if local_28 < 5 { 0xc } else { 0x12 };
            let local_30 = mech_trace::cr!(
                "10068ea5",
                "100696b1",
                FUN_100696b1(mech_trace::rd!("10068ea5", local_70, read_unaligned) as u32)
            );
            let puvar6 = mech_trace::cr!(
                "10068ea5",
                "1001c7df",
                FUN_1001c7df(
                    mech_trace::dr!("10068ea5", param_3),
                    local_30 as i16,
                    local_1c
                )
            );
            if puvar6.is_null() {
                break;
            }
            local_1c = (local_1c as u32).wrapping_add(local_28.wrapping_mul(4)) as i32;
            let local_10 = local_70.add(2);
            let mut local_8c: u32 = 0;
            while (local_8c as i32) < local_28 as i32 {
                let v =
                    mech_trace::rd!("10068ea5", local_10.add(local_8c as usize), read_unaligned);
                FUN_1001c85e(
                    mech_trace::dr!("10068ea5", param_3),
                    puvar6 as usize as i32,
                    v as i8,
                );
                local_c = (local_c as u32).wrapping_add(v as u32) as i32;
                local_8c = local_8c.wrapping_add(1);
            }
            local_8c = local_28;
            while (local_8c as i32) < 4 {
                let v =
                    mech_trace::rd!("10068ea5", local_10.add(local_8c as usize), read_unaligned);
                local_c = (local_c as u32).wrapping_add(v as u32) as i32;
                local_8c = local_8c.wrapping_add(1);
            }
            let u_var1 = (local_c as u32)
                .wrapping_add(mech_trace::rd!("10068ea5", local_70, read_unaligned) as u32)
                .wrapping_add(local_28);
            let u_var7 = ((u_var1 as i32) >> 0x1f) as u32;
            local_c = (((u_var1 ^ u_var7).wrapping_sub(u_var7) & 0xfffff) ^ u_var7)
                .wrapping_sub(u_var7) as i32;
            local_18 = (local_18 as u32).wrapping_add(local_6c as u32) as i32;
            local_70 = local_70.byte_add(local_6c as usize);
            local_7c += 1;
        }
        mech_trace::gw!("10068ea5", "100b3980", _DAT_100b3980, 0xfffffff7_u32 as i32);
        mech_trace::rt!("10068ea5", 0xffffffff)
    }
}
