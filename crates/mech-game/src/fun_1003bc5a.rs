// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: i32;
    static mut DAT_100aaf68: *mut i8;
    static mut DAT_10115154: i32;
    static mut DAT_10160ef0: i32;
    static mut DAT_10160f20: i32;
    static mut DAT_10160f24: i32;
    static mut DAT_10160f28: i32;
    static mut DAT_10160f2c: i32;
    static mut DAT_10160f30: i32;
    static mut DAT_10160f34: i32;
    static mut DAT_10160f38: i32;
    static mut DAT_10160f3c: i32;
    static mut DAT_10160f40: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f4c: i32;
    static mut DAT_10160f54: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003c190(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_10035980(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_1003c230(a: i32, b: i32, c: i32, d: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(off: usize) -> i32 {
    // SAFETY: the global points at the game's own structure, read at the
    // same offsets as the original.
    unsafe {
        mech_trace::rd!(
            "1003bc5a",
            mech_trace::gr!("1003bc5a", "100aaf68", DAT_100aaf68)
                .byte_add(off)
                .cast::<i32>(),
            read_unaligned
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003bc5a(param_1: i32) -> u32 {
    mech_trace::enter!("1003bc5a", param_1);
    // SAFETY: globals and callees are the game's own; param_1 is the
    // caller's object address, read at the original's offsets.
    unsafe {
        let p = param_1 as usize as *const u8;
        mech_trace::rt!(
            "1003bc5a",
            if mech_trace::gr!("1003bc5a", "100ab094", DAT_100ab094) == 0
                || (mech_trace::rd!("1003bc5a", p.byte_add(2).cast::<u16>(), read_unaligned) as u8
                    & 0xf0)
                    != 0xa0
            {
                let i_var1 =
                    mech_trace::rd!("1003bc5a", p.byte_add(0x40).cast::<i32>(), read_unaligned);
                let i_var3 =
                    mech_trace::rd!("1003bc5a", p.byte_add(0x34).cast::<i32>(), read_unaligned)
                        .wrapping_sub(mech_trace::gr!("1003bc5a", "10160f48", _DAT_10160f48));
                let i_var4 =
                    mech_trace::rd!("1003bc5a", p.byte_add(0x38).cast::<i32>(), read_unaligned)
                        .wrapping_sub(mech_trace::gr!("1003bc5a", "10160f44", _DAT_10160f44));
                let i_var5 =
                    mech_trace::rd!("1003bc5a", p.byte_add(0x3c).cast::<i32>(), read_unaligned)
                        .wrapping_sub(mech_trace::gr!("1003bc5a", "10160f4c", _DAT_10160f4c));
                let i_var6 = mech_trace::cr!(
                    "1003bc5a",
                    "1003c190",
                    FUN_1003c190(i_var3, i_var4, i_var5, rd(0xb4).wrapping_add(i_var1))
                );
                if i_var6 == 0 {
                    return mech_trace::rt!("1003bc5a", 5);
                }
                let i_var6 = mech_trace::cr!(
                    "1003bc5a",
                    "10035980",
                    FUN_10035980(
                        i_var3,
                        mech_trace::gr!("1003bc5a", "10160f38", DAT_10160f38),
                        i_var4,
                        mech_trace::gr!("1003bc5a", "10160f3c", DAT_10160f3c),
                        i_var5,
                        mech_trace::gr!("1003bc5a", "10160f40", DAT_10160f40),
                    )
                );
                mech_trace::gw!("1003bc5a", "10115154", DAT_10115154, i_var6);
                if i_var1.wrapping_add(i_var6)
                    < mech_trace::gr!("1003bc5a", "10160f54", DAT_10160f54)
                {
                    return mech_trace::rt!("1003bc5a", 4);
                }
                if mech_trace::gr!("1003bc5a", "10160ef0", DAT_10160ef0)
                    < i_var6.wrapping_sub(i_var1)
                {
                    return mech_trace::rt!("1003bc5a", 5);
                }
                let i_var7 = mech_trace::cr!(
                    "1003bc5a",
                    "10035980",
                    FUN_10035980(
                        i_var3,
                        mech_trace::gr!("1003bc5a", "10160f20", DAT_10160f20),
                        i_var4,
                        mech_trace::gr!("1003bc5a", "10160f24", DAT_10160f24),
                        i_var5,
                        mech_trace::gr!("1003bc5a", "10160f28", DAT_10160f28),
                    )
                );
                let local_14 = if i_var7 < 1 {
                    mech_trace::cr!(
                        "1003bc5a",
                        "1003c230",
                        FUN_1003c230(
                            rd(0x18),
                            i_var7.wrapping_neg(),
                            i_var6.wrapping_neg(),
                            rd(0xa8),
                        )
                    )
                } else {
                    mech_trace::cr!(
                        "1003bc5a",
                        "1003c230",
                        FUN_1003c230(rd(0x18), i_var7, i_var6.wrapping_neg(), rd(0xa8))
                    )
                };
                if i_var1 < local_14 {
                    return mech_trace::rt!("1003bc5a", 6);
                }
                let i_var3 = mech_trace::cr!(
                    "1003bc5a",
                    "10035980",
                    FUN_10035980(
                        i_var3,
                        mech_trace::gr!("1003bc5a", "10160f2c", DAT_10160f2c),
                        i_var4,
                        mech_trace::gr!("1003bc5a", "10160f30", DAT_10160f30),
                        i_var5,
                        mech_trace::gr!("1003bc5a", "10160f34", DAT_10160f34),
                    )
                );
                let local_14 = if i_var3 < 1 {
                    mech_trace::cr!(
                        "1003bc5a",
                        "1003c230",
                        FUN_1003c230(
                            rd(0xb0),
                            i_var3.wrapping_neg(),
                            i_var6.wrapping_neg(),
                            rd(0xac),
                        )
                    )
                } else {
                    mech_trace::cr!(
                        "1003bc5a",
                        "1003c230",
                        FUN_1003c230(rd(0xb0), i_var3, i_var6.wrapping_neg(), rd(0xac))
                    )
                };
                if i_var1 < local_14 { 7 } else { 0 }
            } else {
                1
            }
        )
    }
}
