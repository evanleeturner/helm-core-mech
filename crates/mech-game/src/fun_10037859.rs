// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf28: u32;
    static mut DAT_100adf2c: u32;
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100adf80: u32;
    static mut DAT_100adf84: u32;
    static mut DAT_100adf88: u32;
    static mut DAT_100adf90: u32;
    static mut DAT_100adf94: u32;
    static mut DAT_100b9a18: *mut i32;
    static mut DAT_100b9a1c: u32;
    static mut DAT_100b9a30: [u8; 1];
    static mut DAT_100b9d4c: u32;
    static mut DAT_100b9d54: u32;
    static mut DAT_100b9d58: u32;
    static mut DAT_100b9d5c: u32;
    static mut DAT_100c9240: u32;
    static mut DAT_100c9244: u32;
    static mut DAT_100c9248: u32;
    static mut DAT_100c924c: u32;
    static mut DAT_1016584c: u32;
    static mut DAT_1016593c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10070276(wnd: u32, msg: u32, w_param: u32) -> u8;
    fn FUN_100498c5(wnd: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10070665() -> u8;
    fn FUN_10081fdc() -> u8;
    fn ShowWindow(h: *mut c_void, n: i32) -> i32;
    fn ValidateRect(h: *mut c_void, r: *const c_void) -> i32;
    fn ShowCursor(b: i32) -> i32;
    fn GetMessageTime() -> i32;
    fn PostQuitMessage(code: i32);
    fn DefWindowProcA(h: *mut c_void, msg: u32, w: u32, l: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037859(
    wnd: *mut c_void,
    msg: u32,
    w_param: u32,
    l_param: u32,
) -> i32 {
    mech_trace::enter!("10037859", wnd, msg, w_param, l_param);
    // SAFETY: globals, callees and the pointers they hold are the game's own;
    // l_param points at the caller's structure where the original writes it.
    unsafe {
        if msg > 0xff && msg < 0x109 {
            mech_trace::cr!("10037859", "10070276", FUN_10070276(msg, w_param, l_param));
            return mech_trace::rt!("10037859", 0);
        }
        if msg < 0x10 {
            if msg == 0xf {
                if mech_trace::gr!("10037859", "100adf84", DAT_100adf84) != 0
                    && mech_trace::gr!("10037859", "100b9d4c", DAT_100b9d4c) == 2
                {
                    let tbl = mech_trace::gr!("10037859", "100b9a1c", DAT_100b9a1c)
                        .wrapping_add(0x18) as *const u32;
                    let f: unsafe extern "C" fn() =
                        core::mem::transmute(
                            mech_trace::rd!("10037859", tbl, read_unaligned) as usize
                        );
                    mech_trace::cr!("10037859", "ind", f());
                    mech_trace::cr!(
                        "10037859",
                        "ValidateRect",
                        ValidateRect(wnd, core::ptr::null())
                    );
                    return mech_trace::rt!("10037859", 0);
                }
            } else if msg == 2 {
                PostQuitMessage(0);
                return mech_trace::rt!("10037859", 0);
            }
        } else if msg < 0x1d {
            if msg == 0x1c {
                if mech_trace::gr!("10037859", "100adf28", DAT_100adf28) == 0 {
                    mech_trace::gw!("10037859", "100adf80", DAT_100adf80, w_param);
                    if w_param == 1 {
                        mech_trace::cr!("10037859", "10070665", FUN_10070665());
                        if mech_trace::gr!("10037859", "100b9d58", DAT_100b9d58) < 0x281
                            && mech_trace::gr!("10037859", "100b9d5c", DAT_100b9d5c) < 0x1e1
                        {
                            if mech_trace::gr!("10037859", "100b9d54", DAT_100b9d54) == 1 {
                                mech_trace::cr!("10037859", "10081fdc", FUN_10081fdc());
                                mech_trace::gw!("10037859", "100b9d54", DAT_100b9d54, 0);
                                mech_trace::cr!(
                                    "10037859",
                                    "ShowWindow",
                                    ShowWindow(
                                        mech_trace::gr!("10037859", "100adf6c", DAT_100adf6c),
                                        9
                                    )
                                );
                            } else if !mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18)
                                .is_null()
                                && mech_trace::rd!(
                                    "10037859",
                                    mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18),
                                    read_unaligned
                                ) == 1
                            {
                                mech_trace::cr!(
                                    "10037859",
                                    "ShowWindow",
                                    ShowWindow(
                                        mech_trace::gr!("10037859", "100adf6c", DAT_100adf6c),
                                        4
                                    )
                                );
                            }
                        } else if mech_trace::gr!("10037859", "100b9d4c", DAT_100b9d4c) == 1 {
                            mech_trace::cr!(
                                "10037859",
                                "ShowWindow",
                                ShowWindow(
                                    mech_trace::gr!("10037859", "100adf6c", DAT_100adf6c),
                                    9
                                )
                            );
                        }
                    } else if mech_trace::gr!("10037859", "100b9d58", DAT_100b9d58) < 0x281
                        && mech_trace::gr!("10037859", "100b9d5c", DAT_100b9d5c) < 0x1e1
                    {
                        if mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18).is_null()
                            || mech_trace::rd!(
                                "10037859",
                                mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18),
                                read_unaligned
                            ) != 0
                            || mech_trace::gr!("10037859", "100b9d54", DAT_100b9d54) != 0
                        {
                            if !mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18).is_null()
                                && mech_trace::rd!(
                                    "10037859",
                                    mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18),
                                    read_unaligned
                                ) == 1
                            {
                                mech_trace::cr!(
                                    "10037859",
                                    "ShowWindow",
                                    ShowWindow(
                                        mech_trace::gr!("10037859", "100adf6c", DAT_100adf6c),
                                        0
                                    )
                                );
                            }
                        } else {
                            mech_trace::cr!(
                                "10037859",
                                "ShowWindow",
                                ShowWindow(
                                    mech_trace::gr!("10037859", "100adf6c", DAT_100adf6c),
                                    6
                                )
                            );
                            mech_trace::gw!("10037859", "100b9d54", DAT_100b9d54, 1);
                            mech_trace::cr!("10037859", "10081fdc", FUN_10081fdc());
                        }
                    } else {
                        mech_trace::cr!(
                            "10037859",
                            "ShowWindow",
                            ShowWindow(mech_trace::gr!("10037859", "100adf6c", DAT_100adf6c), 6)
                        );
                    }
                    if !mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18).is_null() {
                        let fp = mech_trace::rd!(
                            "10037859",
                            mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18).add(5),
                            read_unaligned
                        ) as u32;
                        let f: unsafe extern "C" fn(i32, i32, *mut u8, i32) =
                            core::mem::transmute(fp as usize);
                        mech_trace::cr!(
                            "10037859",
                            "ind",
                            f(0, 0x100, &raw mut DAT_100b9a30 as *mut u8, 1)
                        );
                    }
                    return mech_trace::rt!("10037859", 0);
                }
            } else if msg == 0x10 {
                mech_trace::gw!(
                    "10037859",
                    "100adf2c",
                    DAT_100adf2c,
                    mech_trace::gr!("10037859", "100adf2c", DAT_100adf2c).wrapping_add(2)
                );
                mech_trace::gw!("10037859", "100adf28", DAT_100adf28, 1);
                return mech_trace::rt!("10037859", 0);
            }
        } else if msg < 0xa1 {
            if msg == 0xa0 {
                if mech_trace::gr!("10037859", "100adf94", DAT_100adf94) == 0
                    && mech_trace::gr!("10037859", "100b9d4c", DAT_100b9d4c) == 2
                {
                    loop {
                        let r = mech_trace::cr!("10037859", "ShowCursor", ShowCursor(1));
                        if r >= 0 {
                            break;
                        }
                    }
                    mech_trace::gw!("10037859", "100adf94", DAT_100adf94, 1);
                }
            } else if msg == 0x46 && mech_trace::gr!("10037859", "100adf90", DAT_100adf90) != 0 {
                let t = mech_trace::cr!("10037859", "GetMessageTime", GetMessageTime()) as u32;
                if mech_trace::gr!("10037859", "1016593c", DAT_1016593c) < t
                    && (mech_trace::gr!("10037859", "1016584c", DAT_1016584c)
                        < mech_trace::gr!("10037859", "1016593c", DAT_1016593c)
                        || t < mech_trace::gr!("10037859", "1016584c", DAT_1016584c))
                {
                    mech_trace::gw!("10037859", "100adf90", DAT_100adf90, 0);
                } else {
                    let p = l_param as *mut u8;
                    mech_trace::wr!(
                        "10037859",
                        p.byte_add(8).cast::<u32>(),
                        write_unaligned,
                        mech_trace::gr!("10037859", "100c9240", DAT_100c9240)
                    );
                    mech_trace::wr!(
                        "10037859",
                        p.byte_add(0xc).cast::<u32>(),
                        write_unaligned,
                        mech_trace::gr!("10037859", "100c9244", DAT_100c9244)
                    );
                    mech_trace::wr!(
                        "10037859",
                        p.byte_add(0x10).cast::<u32>(),
                        write_unaligned,
                        mech_trace::gr!("10037859", "100c9248", DAT_100c9248)
                    );
                    mech_trace::wr!(
                        "10037859",
                        p.byte_add(0x14).cast::<u32>(),
                        write_unaligned,
                        mech_trace::gr!("10037859", "100c924c", DAT_100c924c)
                    );
                }
                return mech_trace::rt!("10037859", 0);
            }
        } else {
            if msg == 0x200 {
                if mech_trace::gr!("10037859", "100adf94", DAT_100adf94) != 0
                    && (mech_trace::gr!("10037859", "100adf88", DAT_100adf88) == 0
                        || mech_trace::cr!("10037859", "100498c5", FUN_100498c5(4)) == 1)
                {
                    loop {
                        let r = mech_trace::cr!("10037859", "ShowCursor", ShowCursor(0));
                        if r < 0 {
                            break;
                        }
                    }
                    mech_trace::gw!("10037859", "100adf94", DAT_100adf94, 0);
                }
                return mech_trace::rt!("10037859", 0);
            }
            if msg == 0x30f {
                if !mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18).is_null() {
                    let fp = mech_trace::rd!(
                        "10037859",
                        mech_trace::gr!("10037859", "100b9a18", DAT_100b9a18).add(5),
                        read_unaligned
                    ) as u32;
                    let f: unsafe extern "C" fn(i32, i32, *mut u8, i32) =
                        core::mem::transmute(fp as usize);
                    mech_trace::cr!(
                        "10037859",
                        "ind",
                        f(0, 0x100, &raw mut DAT_100b9a30 as *mut u8, 1)
                    );
                    return mech_trace::rt!("10037859", 1);
                }
                return mech_trace::rt!("10037859", 0);
            }
        }
        mech_trace::rt!(
            "10037859",
            mech_trace::cr!(
                "10037859",
                "DefWindowProcA",
                DefWindowProcA(wnd, msg, w_param, l_param as i32)
            )
        )
    }
}
