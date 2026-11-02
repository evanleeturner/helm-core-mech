// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_1009b384: u32;
    static mut DAT_1009b394: u32;
    static mut DAT_100bacc4: i32;
    static mut DAT_100d08e8: u32;
    static mut DAT_1009b704: u8;
    static mut DAT_1009b70c: u8;
    static mut DAT_1009b714: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b1b0(param_1: *mut i8) -> u32;
    fn FUN_10038710(param_1: *mut i8) -> u32;
    fn FUN_1004c110();
    fn FUN_1004c220() -> u32;
    fn FUN_1004b180(param_1: i8);
    fn FUN_1004c2d0();
    fn FUN_1004c520();
    fn FUN_1005bbb0(param_1: *mut i8) -> *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PostMessageA(a: u32, b: u32, c: u32, d: i32) -> i32;
}

unsafe fn cmp(mut p: *const u8, mut q: *const u8) -> i32 {
    // SAFETY: both pointers address NUL-terminated strings, read as the original does.
    unsafe {
        loop {
            let mut b1 = mech_trace::dr!("1004c6d0", p);
            let mut b8 = b1 < mech_trace::dr!("1004c6d0", q);
            if b1 != mech_trace::dr!("1004c6d0", q) {
                return (1u32.wrapping_sub(b8 as u32)).wrapping_sub((b8 as u32 != 0) as u32) as i32;
            }
            if b1 == 0 {
                break;
            }
            b1 = mech_trace::dr!("1004c6d0", p.add(1));
            b8 = b1 < mech_trace::dr!("1004c6d0", q.add(1));
            if b1 != mech_trace::dr!("1004c6d0", q.add(1)) {
                return (1u32.wrapping_sub(b8 as u32)).wrapping_sub((b8 as u32 != 0) as u32) as i32;
            }
            p = p.add(2);
            q = q.add(2);
            if b1 == 0 {
                break;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c6d0() -> u32 {
    mech_trace::enter!("1004c6d0");
    // SAFETY: the globals and callees are the game's own; DAT_10083508 points
    // at the game's state block, read and written at the original's offsets.
    unsafe {
        let mode = mech_trace::rd!(
            "1004c6d0",
            ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x23bb)
                as *const i32),
            read_unaligned
        );
        if mode == 0 {
            let i_var5 = mech_trace::rd!(
                "1004c6d0",
                ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x1fe)
                    as *const i32),
                read_unaligned
            );
            let u_var3 = mech_trace::gr!("1004c6d0", "10083508", DAT_10083508);
            if i_var5 == 1 {
                if mech_trace::dr!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239c)
                        as *const i8)
                ) == 1
                {
                    let r = mech_trace::cr!(
                        "1004c6d0",
                        "10038710",
                        FUN_10038710(
                            (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508)
                                .wrapping_add(0x202)) as *mut i8
                        )
                    );
                    return mech_trace::rt!("1004c6d0", r & 0xffffff00);
                }
            } else {
                if i_var5 < 2 || 3 < i_var5 {
                    mech_trace::gw!(
                        "1004c6d0",
                        "1009b394",
                        DAT_1009b394,
                        mech_trace::gr!("1004c6d0", "1009b384", DAT_1009b384)
                    );
                    return mech_trace::rt!(
                        "1004c6d0",
                        mech_trace::gr!("1004c6d0", "1009b384", DAT_1009b384) & 0xffffff00
                    );
                }
                if mech_trace::dr!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239c)
                        as *const i8)
                ) == 1
                {
                    FUN_1004c110();
                    let r = mech_trace::cr!("1004c6d0", "1004c220", FUN_1004c220());
                    return mech_trace::rt!("1004c6d0", r & 0xffffff00);
                }
            }
            return mech_trace::rt!("1004c6d0", u_var3 & 0xffffff00);
        }
        let mode = mech_trace::rd!(
            "1004c6d0",
            ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x23bb)
                as *const i32),
            read_unaligned
        );
        if mode != 1 {
            let r = mech_trace::cr!(
                "1004c6d0",
                "1005b1b0",
                FUN_1005b1b0(&raw mut DAT_1009b714 as *mut i8)
            );
            return mech_trace::rt!("1004c6d0", r & 0xffffff00);
        }
        let mut tmp = mech_trace::rd!(
            "1004c6d0",
            ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x12a)
                as *const i32),
            read_unaligned
        );
        tmp = tmp.wrapping_add(mech_trace::gr!("1004c6d0", "100bacc4", DAT_100bacc4));
        mech_trace::wr!(
            "1004c6d0",
            ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x12a) as *mut i32),
            write_unaligned,
            tmp
        );
        FUN_1004b180(1);
        FUN_1004c2d0();
        let u_var2 = mech_trace::gr!("1004c6d0", "10083508", DAT_10083508);
        let sel = mech_trace::rd!(
            "1004c6d0",
            ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x1fe)
                as *const i32),
            read_unaligned
        );
        let mut u_var3 = sel as u32;
        match sel as u32 {
            0 => {
                let r = mech_trace::cr!(
                    "1004c6d0",
                    "10038710",
                    FUN_10038710(
                        (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508).wrapping_add(0x202))
                            as *mut i8
                    )
                );
                return mech_trace::rt!("1004c6d0", r & 0xffffff00);
            }
            1 => {
                if mech_trace::dr!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239c)
                        as *const i8)
                ) == 1
                {
                    FUN_1004c520();
                    let r = mech_trace::cr!(
                        "1004c6d0",
                        "10038710",
                        FUN_10038710(
                            (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508)
                                .wrapping_add(0x202)) as *mut i8
                        )
                    );
                    return mech_trace::rt!("1004c6d0", r & 0xffffff00);
                }
                let p = mech_trace::cr!(
                    "1004c6d0",
                    "1005bbb0",
                    FUN_1005bbb0(
                        (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508).wrapping_add(0x2a2))
                            as *mut i8
                    )
                );
                let i_var5 = cmp(p, &raw const DAT_1009b704);
                if i_var5 != 0 {
                    let r = mech_trace::cr!(
                        "1004c6d0",
                        "10038710",
                        FUN_10038710(
                            (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508)
                                .wrapping_add(0x2a2)) as *mut i8
                        )
                    );
                    return mech_trace::rt!("1004c6d0", r & 0xffffff00);
                }
                let b = mech_trace::cr!(
                    "1004c6d0",
                    "PostMessageA",
                    PostMessageA(
                        mech_trace::gr!("1004c6d0", "100d08e8", DAT_100d08e8),
                        0xc02b,
                        0xc00a,
                        0
                    )
                );
                return mech_trace::rt!("1004c6d0", (((b as u32) >> 8) << 8) | 1);
            }
            2 => {
                if mech_trace::dr!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239c)
                        as *const i8)
                ) == 0
                {
                    let p = mech_trace::cr!(
                        "1004c6d0",
                        "1005bbb0",
                        FUN_1005bbb0(
                            (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508)
                                .wrapping_add(0x2a2)) as *mut i8
                        )
                    );
                    let i_var5 = cmp(p, &raw const DAT_1009b70c);
                    if i_var5 == 0 {
                        let b = mech_trace::cr!(
                            "1004c6d0",
                            "PostMessageA",
                            PostMessageA(
                                mech_trace::gr!("1004c6d0", "100d08e8", DAT_100d08e8),
                                0xc02b,
                                0xc00a,
                                0
                            )
                        );
                        return mech_trace::rt!("1004c6d0", (((b as u32) >> 8) << 8) | 1);
                    }
                } else {
                    FUN_1004c520();
                }
                let r = mech_trace::gr!("1004c6d0", "10083508", DAT_10083508);
                mech_trace::dw!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239d)
                        as *mut u8),
                    1
                );
                return mech_trace::rt!("1004c6d0", r & 0xffffff00);
            }
            3 => {
                if mech_trace::dr!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239c)
                        as *const i8)
                ) == 1
                {
                    FUN_1004c520();
                    let r = mech_trace::cr!(
                        "1004c6d0",
                        "10038710",
                        FUN_10038710(
                            (mech_trace::gr!("1004c6d0", "10083508", DAT_10083508)
                                .wrapping_add(0x202)) as *mut i8
                        )
                    );
                    return mech_trace::rt!("1004c6d0", r & 0xffffff00);
                }
                mech_trace::dw!(
                    "1004c6d0",
                    ((mech_trace::gr!("1004c6d0", "10083508", DAT_10083508) as usize + 0x239d)
                        as *mut u8),
                    1
                );
                u_var3 = u_var2;
            }
            _ => {}
        }
        mech_trace::rt!("1004c6d0", u_var3 & 0xffffff00)
    }
}
