// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1f0: i32;
    static mut DAT_100adf28: i32;
    static mut DAT_100aa1b4: i16;
    static mut DAT_100b3900: i32;
    static mut _DAT_100aa1c4: i32;
    static mut _DAT_100aa1c8: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 8];
    static mut DAT_101eb4c0: *mut i8;
    static mut DAT_101eb4d0: [i32; 8];
    static mut DAT_100aa21c: [i8; 4];
    static mut DAT_100aa220: [i8; 4];
    static mut DAT_100aa224: [i8; 4];
    static mut DAT_100aa228: [i8; 4];
    static mut DAT_100aa22c: [i8; 4];
    static mut DAT_100aa230: [i8; 4];
    static mut DAT_100aa234: [i8; 4];
    static mut DAT_100aa250: [i8; 4];
    static mut s___s__exited_the_game__100aa238: [i8; 4];
    static mut DAT_100b0a1c: i32;
    static mut DAT_100aa218: i32;
    static mut DAT_100aa1a8: i32;
    static mut DAT_100aa1d0: i16;
    static mut DAT_100aa1f4: i32;
    static mut DAT_101eb510: [i32; 8];
    static mut DAT_100aa1c0: i32;
    static mut DAT_100b38fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_10065e16() -> i32;
    fn FUN_10006fc9() -> i32;
    fn FUN_10011e80(param_1: i32) -> i32;
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f853(param_1: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_100140da();
    fn FUN_10012c5d(param_1: i32, param_2: i32);
    fn FUN_100134c7(param_1: i32, param_2: i32);
    fn FUN_100135fe(param_1: i32, param_2: i32);
    fn FUN_10013763(param_1: *mut i8, param_2: i32);
    fn FUN_100138d2(param_1: i32, param_2: i32);
    fn FUN_100139e1(param_1: *mut i8, param_2: i32);
    fn FUN_10004b19(param_1: i32, param_2: *mut i32);
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_10012be0(param_1: i32, param_2: i32);
    fn FUN_1002f727(param_1: i32);
    fn FUN_1002f7f7(param_1: i32);
    fn FUN_1001261d();
    fn FUN_100133f8();
    fn FUN_10013817();
    fn FUN_10013716();
    fn FUN_10065d96() -> i32;
    fn FUN_10065db4();
}

unsafe fn rd16(addr: i32) -> u16 {
    // SAFETY: caller supplies the address of a live game structure field.
    unsafe { mech_trace::rd!("100120fc", (addr as *const u16), read_unaligned) }
}

unsafe fn wr16(addr: i32, v: u16) {
    // SAFETY: caller supplies the address of a live game structure field.
    unsafe { mech_trace::wr!("100120fc", (addr as *mut u16), write_unaligned, v) }
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: caller supplies the address of a live game structure field.
    unsafe { mech_trace::rd!("100120fc", (addr as *const i32), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100120fc() -> u32 {
    mech_trace::enter!("100120fc");
    // SAFETY: globals and callees are the game's own; pointers derive from
    // the game's tables and are accessed where the original does.
    unsafe {
        if mech_trace::gr!("100120fc", "100aa1f0", DAT_100aa1f0) == 1
            && mech_trace::gr!("100120fc", "100adf28", DAT_100adf28) == 0
        {
            let mut local_68: [i8; 80] = [0; 80];
            let mut local_c: [i8; 3] = [0; 3];
            let tbl = (&raw mut DAT_100eef40).cast::<i32>();
            let times = (&raw mut DAT_101eb4d0).cast::<i32>();

            mech_trace::gw!("100120fc", "100aa1b4", DAT_100aa1b4, 0);
            mech_trace::gw!(
                "100120fc",
                "100b3900",
                DAT_100b3900,
                mech_trace::cr!("100120fc", "10065e16", FUN_10065e16())
            );
            loop {
                let local_14 = mech_trace::cr!("100120fc", "10006fc9", FUN_10006fc9());
                if local_14 == -1 {
                    break;
                }
                mech_trace::gw!(
                    "100120fc",
                    "100aa1c4",
                    _DAT_100aa1c4,
                    mech_trace::gr!("100120fc", "100aa1c4", _DAT_100aa1c4).wrapping_add(1)
                );
                let local_8 = mech_trace::cr!("100120fc", "10011e80", FUN_10011e80(local_14));
                if local_8 == mech_trace::gr!("100120fc", "100adf38", DAT_100adf38) || local_8 <= -1
                {
                    continue;
                }
                let local_18 = mech_trace::rd!("100120fc", tbl.offset(local_8 as isize), read);
                if local_18 == 0 {
                    continue;
                }
                local_c[0] = mech_trace::dr!(
                    "100120fc",
                    mech_trace::gr!("100120fc", "101eb4c0", DAT_101eb4c0)
                );
                local_c[1] = mech_trace::dr!(
                    "100120fc",
                    mech_trace::gr!("100120fc", "101eb4c0", DAT_101eb4c0).add(1)
                );
                local_c[2] = 0;
                mech_trace::wr!(
                    "100120fc",
                    times.offset(local_8 as isize),
                    write,
                    mech_trace::gr!("100120fc", "100b3900", DAT_100b3900)
                );
                let flags_addr = local_18.wrapping_add(0x14);
                if (rd16(flags_addr) & 0x4000) != 0 {
                    FUN_1002f783(rd32(local_18.wrapping_add(0x40)));
                    FUN_1002f853(rd32(local_18.wrapping_add(0x40)));
                    FUN_1002fbc0(rd32(local_18.wrapping_add(0x40)) as *mut i32);
                    wr16(flags_addr, rd16(flags_addr) & 0xb7ff);
                    FUN_100140da();
                }
                let c = local_c.as_ptr();
                let p = mech_trace::gr!("100120fc", "101eb4c0", DAT_101eb4c0);
                if mech_trace::cr!(
                    "100120fc",
                    "__strcmpi",
                    __strcmpi(c, (&raw mut DAT_100aa21c).cast::<i8>())
                ) == 0
                {
                    FUN_10012c5d(p as i32, local_8);
                } else if mech_trace::cr!(
                    "100120fc",
                    "__strcmpi",
                    __strcmpi(c, (&raw mut DAT_100aa220).cast::<i8>())
                ) == 0
                {
                    FUN_100134c7(p as i32, local_8);
                } else if mech_trace::cr!(
                    "100120fc",
                    "__strcmpi",
                    __strcmpi(c, (&raw mut DAT_100aa224).cast::<i8>())
                ) == 0
                {
                    FUN_100135fe(p as i32, local_8);
                } else if mech_trace::cr!(
                    "100120fc",
                    "__strcmpi",
                    __strcmpi(c, (&raw mut DAT_100aa228).cast::<i8>())
                ) == 0
                {
                    FUN_10013763(p, local_8);
                } else if mech_trace::cr!(
                    "100120fc",
                    "__strcmpi",
                    __strcmpi(c, (&raw mut DAT_100aa22c).cast::<i8>())
                ) == 0
                {
                    FUN_100138d2(p as i32, local_8);
                } else if mech_trace::cr!(
                    "100120fc",
                    "__strcmpi",
                    __strcmpi(c, (&raw mut DAT_100aa230).cast::<i8>())
                ) == 0
                {
                    FUN_100139e1(p, local_8);
                } else {
                    if mech_trace::cr!(
                        "100120fc",
                        "__strcmpi",
                        __strcmpi(c, (&raw mut DAT_100aa234).cast::<i8>())
                    ) == 0
                    {
                        if mech_trace::gr!("100120fc", "100b0a1c", DAT_100b0a1c) == 0 {
                            FUN_10004b19(-2, rd32(local_18.wrapping_add(0x20)) as *mut i32);
                        }
                        FUN_100140da();
                        mech_trace::cr!(
                            "100120fc",
                            "_sprintf",
                            _sprintf(
                                local_68.as_mut_ptr(),
                                (&raw mut s___s__exited_the_game__100aa238).cast::<i8>(),
                                local_18.wrapping_add(0x110),
                            )
                        );
                        FUN_1005be24(local_68.as_mut_ptr(), 1, 0x712, 0x50);
                        break;
                    }
                    if mech_trace::cr!(
                        "100120fc",
                        "__strcmpi",
                        __strcmpi(c, (&raw mut DAT_100aa250).cast::<i8>())
                    ) == 0
                    {
                        FUN_10012be0(p as i32, local_8);
                    } else {
                        mech_trace::gw!(
                            "100120fc",
                            "100aa1c8",
                            _DAT_100aa1c8,
                            mech_trace::gr!("100120fc", "100aa1c8", _DAT_100aa1c8).wrapping_add(1)
                        );
                    }
                }
            }
            if mech_trace::gr!("100120fc", "100aa218", DAT_100aa218)
                <= mech_trace::gr!("100120fc", "100b3900", DAT_100b3900)
            {
                mech_trace::gw!(
                    "100120fc",
                    "100aa218",
                    DAT_100aa218,
                    mech_trace::gr!("100120fc", "100aa1a8", DAT_100aa1a8)
                        .wrapping_add(mech_trace::gr!("100120fc", "100b3900", DAT_100b3900))
                );
                let mut local_10: i32 = 0;
                while local_10 < 8 {
                    let ent = mech_trace::rd!("100120fc", tbl.offset(local_10 as isize), read);
                    if local_10 != mech_trace::gr!("100120fc", "100adf38", DAT_100adf38)
                        && ent != 0
                        && (rd16(ent.wrapping_add(0x14)) & 0x4000) == 0
                        && 0x38e
                            < mech_trace::gr!("100120fc", "100b3900", DAT_100b3900).wrapping_sub(
                                mech_trace::rd!("100120fc", times.offset(local_10 as isize), read),
                            )
                    {
                        let e = mech_trace::rd!("100120fc", tbl.offset(local_10 as isize), read);
                        FUN_1002f727(rd32(e.wrapping_add(0x40)));
                        let e = mech_trace::rd!("100120fc", tbl.offset(local_10 as isize), read);
                        FUN_1002f7f7(rd32(e.wrapping_add(0x40)));
                        let e = mech_trace::rd!("100120fc", tbl.offset(local_10 as isize), read);
                        let fa = e.wrapping_add(0x14);
                        wr16(fa, rd16(fa) | 0x4800);
                        FUN_100140da();
                    }
                    local_10 = local_10.wrapping_add(1);
                }
                if mech_trace::gr!("100120fc", "100aa1d0", DAT_100aa1d0) == 0 {
                    if mech_trace::gr!("100120fc", "100aa1f4", DAT_100aa1f4) == 2 {
                        FUN_10013716();
                    } else {
                        mech_trace::wr!(
                            "100120fc",
                            (&raw mut DAT_101eb510)
                                .cast::<i32>()
                                .offset(
                                    mech_trace::gr!("100120fc", "100adf38", DAT_100adf38) as isize
                                ),
                            write,
                            1
                        );
                    }
                } else {
                    FUN_1001261d();
                    FUN_100133f8();
                    mech_trace::gw!(
                        "100120fc",
                        "100aa1c0",
                        DAT_100aa1c0,
                        mech_trace::gr!("100120fc", "100aa1c0", DAT_100aa1c0).wrapping_add(1)
                    );
                    if (mech_trace::gr!("100120fc", "100aa1c0", DAT_100aa1c0) & 1) != 0 {
                        FUN_10013817();
                    }
                }
            }
            if mech_trace::gr!("100120fc", "100aa1b4", DAT_100aa1b4) == 0
                && mech_trace::gr!("100120fc", "100aa1f4", DAT_100aa1f4) == 2
            {
                let i_var1 = mech_trace::cr!("100120fc", "10065d96", FUN_10065d96());
                mech_trace::gw!(
                    "100120fc",
                    "100b38fc",
                    DAT_100b38fc,
                    mech_trace::gr!("100120fc", "100b38fc", DAT_100b38fc).wrapping_add(i_var1)
                );
            }
            if mech_trace::gr!("100120fc", "100aa1f4", DAT_100aa1f4) == 2 {
                FUN_10065db4();
            }
        }
        mech_trace::rt!("100120fc", 1)
    }
}
