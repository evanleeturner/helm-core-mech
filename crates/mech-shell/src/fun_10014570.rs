// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6720: *mut u32;
    static mut DAT_100c6100: u32;
    static mut DAT_100a672c: *mut c_void;
    static mut DAT_100a6ba4: u8;
    static mut DAT_10098204: u32;
    static mut DAT_1008ee04: *mut i32;
    static mut DAT_1008ee0c: i32;
    static mut DAT_100d08e8: *mut c_void;
    static mut DAT_100d08ec: *mut c_void;
    static mut DAT_100d08dc: *mut c_void;
    static mut DAT_10098214: i32;
    static mut DAT_100a757c: i32;
    static mut DAT_10083508: i32;
    static mut DAT_1008edf4:
        Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, u32)>;
    static mut DAT_100a6734: *mut c_void;
    static mut _DAT_100b0f44: i32;
    static mut DAT_100b0f50: u8;
    static mut DAT_1008ee18: i32;
    static mut _DAT_1008ee10: i32;
    static mut DAT_1009821c: i32;
    static mut DAT_10098218: i32;
    static mut s_Save_game__Yes_No_1008ee30: u8;
    static mut s_Sell_Out__Yes_No_1008ee44: u8;
    static mut s_mw2help_hlp_1008ee58: u8;
    static mut DAT_1008ee64: u8;
    static mut DAT_1008ee74: u8;
    static mut s_mw2help_hlp_1008ee68: u8;
    static mut DAT_100a6780: u8;
    static mut DAT_1008ee88: u8;
    static mut s_help_hlp_1008ee7c: u8;
    static mut DAT_1008ee98: u8;
    static mut s_help_hlp_1008ee8c: u8;
    static mut s_exittos_1008ef18: u8;
    static mut s_SHELLMER_CFG_1008ef08: u8;
    static mut DAT_1008eeb0: u8;
    static mut s_mland_1008eea0: u8;
    static mut s_mland2_1008eea8: u8;
    static mut s_You_cannot_afford_even_the_cheap_1008eeb4: u8;
    static mut s_mpoorw_1008ef00: u8;
    static mut LAB_100169f0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b760(param_1: u32, param_2: u32, param_3: u32);
    fn FUN_1000f290(param_1: i32);
    fn FUN_1005b1b0(param_1: *const u8, param_2: i32) -> i32;
    fn FUN_10016cd0();
    fn FUN_10054680();
    fn _sprintf(dest: *mut u8, fmt: *const u8, ...) -> i32;
    fn FUN_1005abf0();
    fn FUN_1000f970();
    fn FUN_1003fe90();
    fn FUN_10011170(param_1: u32, param_2: u8, param_3: *const u8);
    fn FUN_1004e0e0(param_1: *mut c_void, param_2: *mut u8);
    fn FUN_10007770(param_1: *mut c_void, param_2: i32);
    fn FUN_1001d650();
    fn FUN_100396a0(param_1: *const u8);
    fn FUN_100361e0(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_1001a980(param_1: *mut u8, param_2: i32, param_3: i32);
    fn FUN_10013490(param_1: *mut c_void, param_2: *mut u8);
    fn FUN_10041780(param_1: *mut c_void);
    fn FUN_10037e50(param_1: *mut u8);
    fn FUN_10038710(param_1: *mut u8);
    fn FUN_1000c250();
    fn FUN_1000c2b0();
    fn FUN_1000c410();
    fn FUN_1007d270(param_1: *mut u8) -> i32;
    fn FUN_10059880(param_1: *mut c_void);
    fn FUN_10051340(param_1: *mut c_void);
    fn FUN_10044090(param_1: *mut c_void);
    fn FUN_1006e050(param_1: *mut c_void);
    fn FUN_1001c350(param_1: *mut c_void);
    fn FUN_10060c50(param_1: *mut c_void);
    fn FUN_10044a60(param_1: *mut c_void);
    fn FUN_10018250(param_1: *mut c_void);
    fn FUN_100367d0(param_1: *mut c_void);
    fn FUN_10042920(param_1: *mut c_void);
    fn FUN_10057a90(param_1: *mut c_void);
    fn FUN_10040bf0(param_1: *mut c_void);
    fn FUN_1001db60(param_1: *mut c_void, param_2: i32);
    fn FUN_1003cc80(param_1: *mut c_void);
    fn FUN_1006c210(param_1: *mut c_void, param_2: i32);
    fn FUN_1001afe0(param_1: *mut c_void);
    fn FUN_1005bd10(param_1: *mut c_void);
    fn FUN_1003a690(param_1: *mut c_void, param_2: i32);
    fn FUN_10033090(param_1: *mut c_void, param_2: i32);
    fn FUN_1003bca0(param_1: *mut c_void);
    fn FUN_10011710(param_1: *mut c_void);
    fn FUN_10016d80(param_1: *mut c_void);
    fn FUN_1000e970(param_1: *const u8, param_2: u32, param_3: u32);
    fn FUN_10043230(param_1: *mut c_void);
    fn FUN_1000fe40();
    fn FUN_10050f10(param_1: *mut c_void);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000f170() -> u32;
    fn FUN_1000f190();
    fn FUN_1000f1b0();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10012830(param_1: *mut c_void);
    fn FUN_10012620(param_1: *mut c_void);
    fn FUN_10010a30(param_1: *mut i32);
    fn FUN_10010a60(param_1: *mut i32);
    fn FUN_100133d0(param_1: *mut c_void);
    fn FUN_100125f0(param_1: *mut c_void);
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100124d0(this: *mut c_void, a: i32, b: i32, c: i32, d: i32);
    fn FUN_10012470(this: *mut c_void, a: i32, b: i32, c: i32, d: i32);
}

unsafe extern "stdcall" {
    fn DefWindowProcA(hwnd: *mut c_void, msg: u32, wparam: u32, lparam: u32) -> i32;
    fn PostQuitMessage(code: i32);
    fn BeginPaint(hwnd: *mut c_void, ps: *mut i32) -> *mut c_void;
    fn EndPaint(hwnd: *mut c_void, ps: *mut i32) -> i32;
    fn SetFocus(hwnd: *mut c_void) -> *mut c_void;
    fn ShowCursor(show: i32) -> i32;
    fn PostMessageA(hwnd: *mut c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
    fn EnableMenuItem(menu: *mut c_void, id: u32, flags: u32) -> i32;
    fn GetFileAttributesA(name: *const u8) -> u32;
    fn WinHelpA(hwnd: *mut c_void, file: *const u8, cmd: u32, data: u32) -> i32;
    fn SetMenu(hwnd: *mut c_void, menu: *mut c_void) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
    fn DialogBoxParamA(
        inst: *mut c_void,
        name: u32,
        parent: *mut c_void,
        proc_: *const u8,
        param: i32,
    ) -> i32;
}

unsafe fn mem(off: u32) -> *mut u8 {
    // SAFETY: the game's global holds the address of its state block.
    unsafe {
        (mech_trace::gr!("10014570", "10083508", DAT_10083508) as u32).wrapping_add(off) as usize
            as *mut u8
    }
}

unsafe fn set(off: u32, v: u32) {
    // SAFETY: writes into the game's state block.
    unsafe { mech_trace::wr!("10014570", mem(off).cast::<u32>(), write_unaligned, v) }
}

unsafe fn get(off: u32) -> i32 {
    // SAFETY: reads from the game's state block.
    unsafe { mech_trace::rd!("10014570", mem(off).cast::<i32>(), read_unaligned) }
}

unsafe fn menu_enable(id: u32, st: u32) {
    // SAFETY: the menu handle is the game's own.
    unsafe {
        mech_trace::cr!(
            "10014570",
            "EnableMenuItem",
            EnableMenuItem(
                mech_trace::gr!("10014570", "100d08ec", DAT_100d08ec),
                id,
                st
            )
        );
    }
}

unsafe fn chk(p3: u32) {
    if p3 == 0xc02e {
        // SAFETY: game-owned menu.
        unsafe { menu_enable(0x9c97, 0) }
    }
}

unsafe fn unhide() {
    // SAFETY: game globals and Win32 call.
    unsafe {
        if mech_trace::gr!("10014570", "1008ee0c", DAT_1008ee0c) != 0 {
            loop {
                if mech_trace::cr!("10014570", "ShowCursor", ShowCursor(1)) >= 0 {
                    break;
                }
            }
            mech_trace::gw!("10014570", "1008ee0c", DAT_1008ee0c, 0);
        }
    }
}

unsafe fn notify(msg: u32, wparam: u32) {
    // SAFETY: the callback and window handle are the game's own.
    unsafe {
        match mech_trace::gr!("10014570", "1008edf4", DAT_1008edf4) {
            None => {
                mech_trace::cr!(
                    "10014570",
                    "PostMessageA",
                    PostMessageA(
                        mech_trace::gr!("10014570", "100d08e8", DAT_100d08e8),
                        msg,
                        wparam,
                        0
                    )
                );
            }
            Some(f) => {
                mech_trace::cr!(
                    "10014570",
                    "ind",
                    f(
                        mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                        (&raw mut DAT_100b0f50).cast::<c_void>(),
                        (&raw mut _DAT_100b0f44).cast::<c_void>(),
                        msg,
                    )
                );
            }
        }
    }
}

unsafe fn help(fmt1: *const u8, f1: *const u8, fmt2: *const u8, f2: *const u8) {
    let mut buf = [0u8; 32];
    // SAFETY: the buffer and strings are valid for the calls.
    unsafe {
        mech_trace::cr!("10014570", "_sprintf", _sprintf(buf.as_mut_ptr(), fmt1, f1));
        if mech_trace::cr!(
            "10014570",
            "GetFileAttributesA",
            GetFileAttributesA(buf.as_ptr())
        ) == 0xffff_ffff
        {
            mech_trace::cr!(
                "10014570",
                "_sprintf",
                _sprintf(buf.as_mut_ptr(), fmt2, &raw mut DAT_100a6780, f2)
            );
        }
        if mech_trace::cr!(
            "10014570",
            "WinHelpA",
            WinHelpA(
                mech_trace::gr!("10014570", "100d08e8", DAT_100d08e8),
                buf.as_ptr(),
                3,
                0
            )
        ) != 0
        {
            mech_trace::gw!("10014570", "10098218", DAT_10098218, 1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10014570(
    param_1: *mut c_void,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> i32 {
    mech_trace::enter!("10014570", param_1, param_2, param_3, param_4);
    // SAFETY: all globals, callees and imports are the game's own.
    unsafe {
        if param_2 >= 0x100 && param_2 < 0x109 {
            FUN_1005b760(param_2, param_3, param_4);
            let v = mech_trace::gr!("10014570", "100c6100", DAT_100c6100);
            mech_trace::dw!(
                "10014570",
                mech_trace::gr!("10014570", "100a6720", DAT_100a6720),
                v
            );
            return mech_trace::rt!("10014570", 0);
        }
        if param_2 < 0x10 {
            if param_2 == 0xf {
                let mut ps = [0i32; 16];
                mech_trace::cr!(
                    "10014570",
                    "BeginPaint",
                    BeginPaint(param_1, ps.as_mut_ptr())
                );
                if !mech_trace::gr!("10014570", "100a672c", DAT_100a672c).is_null() {
                    if mech_trace::gr!("10014570", "100a6ba4", DAT_100a6ba4) == 0 {
                        FUN_10012470(
                            mech_trace::gr!("10014570", "100a672c", DAT_100a672c),
                            ps[2],
                            ps[3],
                            ps[4],
                            ps[5],
                        );
                        FUN_10012620(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
                    } else {
                        FUN_100124d0(
                            mech_trace::gr!("10014570", "100a672c", DAT_100a672c),
                            0,
                            0,
                            0x140,
                            200,
                        );
                        FUN_10012830(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
                    }
                }
                mech_trace::cr!("10014570", "EndPaint", EndPaint(param_1, ps.as_mut_ptr()));
                return mech_trace::rt!("10014570", 0);
            }
            if param_2 == 2 {
                PostQuitMessage(0);
                return mech_trace::rt!("10014570", 0);
            }
            return mech_trace::rt!(
                "10014570",
                mech_trace::cr!(
                    "10014570",
                    "DefWindowProcA",
                    DefWindowProcA(param_1, param_2, param_3, param_4)
                )
            );
        }
        if param_2 < 0xa1 {
            if param_2 == 0xa0 {
                unhide();
                return mech_trace::rt!("10014570", 0);
            }
            if param_2 == 0x1c {
                mech_trace::gw!("10014570", "10098204", DAT_10098204, param_3);
                if param_3 == 0 {
                    if mech_trace::cr!("10014570", "1000f170", FUN_1000f170()) != 0 {
                        FUN_1000f190();
                    }
                    if !mech_trace::gr!("10014570", "1008ee04", DAT_1008ee04).is_null() {
                        FUN_10010a60(mech_trace::gr!("10014570", "1008ee04", DAT_1008ee04));
                    }
                    return mech_trace::rt!("10014570", 0);
                }
                if mech_trace::cr!("10014570", "1000f170", FUN_1000f170()) != 0 {
                    FUN_1000f1b0();
                }
                if !mech_trace::gr!("10014570", "1008ee04", DAT_1008ee04).is_null() {
                    FUN_10010a30(mech_trace::gr!("10014570", "1008ee04", DAT_1008ee04));
                }
                mech_trace::cr!(
                    "10014570",
                    "SetFocus",
                    SetFocus(mech_trace::gr!("10014570", "100d08e8", DAT_100d08e8))
                );
                return mech_trace::rt!("10014570", 0);
            }
            return mech_trace::rt!(
                "10014570",
                mech_trace::cr!(
                    "10014570",
                    "DefWindowProcA",
                    DefWindowProcA(param_1, param_2, param_3, param_4)
                )
            );
        }
        if param_2 < 0x201 {
            if param_2 == 0x200 {
                if mech_trace::cr!("10014570", "1000f170", FUN_1000f170()) != 0
                    && mech_trace::gr!("10014570", "1008ee0c", DAT_1008ee0c) == 0
                {
                    loop {
                        if mech_trace::cr!("10014570", "ShowCursor", ShowCursor(0)) < 0 {
                            break;
                        }
                    }
                    mech_trace::gw!("10014570", "1008ee0c", DAT_1008ee0c, 1);
                }
                let edi = ((param_4 >> 16) & 0xffff) as i32;
                if mech_trace::gr!("10014570", "10098214", DAT_10098214) != 0 {
                    if mech_trace::gr!("10014570", "100a757c", DAT_100a757c) != 1 {
                        return mech_trace::rt!("10014570", 0);
                    }
                    if edi > 2 {
                        mech_trace::cr!(
                            "10014570",
                            "SetMenu",
                            SetMenu(param_1, core::ptr::null_mut())
                        );
                        mech_trace::gw!("10014570", "10098214", DAT_10098214, 0);
                        return mech_trace::rt!("10014570", 0);
                    }
                }
                if mech_trace::gr!("10014570", "100a757c", DAT_100a757c) == 1
                    && mech_trace::gr!("10014570", "10098214", DAT_10098214) == 0
                    && mech_trace::cr!("10014570", "1000f170", FUN_1000f170()) == 0
                    && mech_trace::cr!("10014570", "GetSystemMetrics", GetSystemMetrics(0xf)) >= edi
                {
                    mech_trace::cr!(
                        "10014570",
                        "SetMenu",
                        SetMenu(
                            param_1,
                            mech_trace::gr!("10014570", "100d08ec", DAT_100d08ec)
                        )
                    );
                    mech_trace::gw!("10014570", "10098214", DAT_10098214, 1);
                    FUN_10012470(
                        mech_trace::gr!("10014570", "100a672c", DAT_100a672c),
                        0,
                        0,
                        0x280,
                        0x1e0,
                    );
                    FUN_10012620(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
                }
                return mech_trace::rt!("10014570", 0);
            }
            if param_2 == 0x111 {
                if mech_trace::cr!("10014570", "1000f170", FUN_1000f170()) != 0 {
                    FUN_1000f290(0);
                }
                match param_3 & 0xffff {
                    0x9c41 => {
                        if get(0x23c7) != 0xc02b
                            && mech_trace::cr!(
                                "10014570",
                                "1005b1b0",
                                FUN_1005b1b0(&raw const s_Save_game__Yes_No_1008ee30, 1)
                            ) == 0
                        {
                            mech_trace::gw!("10014570", "1008ee18", DAT_1008ee18, 1);
                            notify(0xc02e, 0xc02b);
                            return mech_trace::rt!("10014570", 0);
                        }
                        FUN_1000f360();
                        mech_trace::gw!("10014570", "1008ee10", _DAT_1008ee10, 0);
                        FUN_10016cd0();
                        notify(0xc02b, 0xc02b);
                        return mech_trace::rt!("10014570", 0);
                    }
                    0x9c43 => {
                        if mech_trace::cr!(
                            "10014570",
                            "1005b1b0",
                            FUN_1005b1b0(&raw const s_Sell_Out__Yes_No_1008ee44, 1)
                        ) == 0
                        {
                            FUN_10016cd0();
                            FUN_100133d0(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
                            let mut d = mech_trace::rd!(
                                "10014570",
                                mech_trace::gr!("10014570", "100a672c", DAT_100a672c)
                                    .cast::<u8>()
                                    .add(0x42)
                                    .cast::<*mut u32>(),
                                read_unaligned
                            );
                            let mut n = 0x12c00;
                            while n != 0 {
                                mech_trace::wr!("10014570", d, write_unaligned, 0);
                                d = d.add(1);
                                n -= 1;
                            }
                            notify(0xc002, 0xc002);
                            mech_trace::gw!("10014570", "1009821c", DAT_1009821c, 1);
                            return mech_trace::rt!("10014570", 0);
                        }
                    }
                    0x9c4b => {
                        FUN_10016cd0();
                        menu_enable(0x9c4b, 1);
                        menu_enable(0x9c97, 1);
                        FUN_10054680();
                        mech_trace::gw!("10014570", "1009821c", DAT_1009821c, 1);
                    }
                    0x9c4c => {
                        help(
                            &raw const DAT_1008ee64,
                            &raw const s_mw2help_hlp_1008ee58,
                            &raw const DAT_1008ee74,
                            &raw const s_mw2help_hlp_1008ee68,
                        );
                    }
                    0x9c94 => {
                        FUN_10016cd0();
                        menu_enable(0x9c94, 1);
                        menu_enable(0x9c97, 1);
                        FUN_1005abf0();
                        mech_trace::gw!("10014570", "1009821c", DAT_1009821c, 1);
                    }
                    0x9c95 => {
                        help(
                            &raw const DAT_1008ee88,
                            &raw const s_help_hlp_1008ee7c,
                            &raw const DAT_1008ee98,
                            &raw const s_help_hlp_1008ee8c,
                        );
                    }
                    0x9c96 => {
                        mech_trace::cr!(
                            "10014570",
                            "DialogBoxParamA",
                            DialogBoxParamA(
                                mech_trace::gr!("10014570", "100d08dc", DAT_100d08dc),
                                0x8a,
                                mech_trace::gr!("10014570", "100d08e8", DAT_100d08e8),
                                &raw const LAB_100169f0,
                                0,
                            )
                        );
                    }
                    0x9c97 => {
                        notify(0xc02e, 0xc02e);
                    }
                    0x9c99 => {
                        FUN_10016cd0();
                        menu_enable(0x9c99, 1);
                        menu_enable(0x9c97, 1);
                        FUN_1000f970();
                        mech_trace::gw!("10014570", "1009821c", DAT_1009821c, 1);
                    }
                    0x9c9a => {
                        FUN_10016cd0();
                        menu_enable(0x9c9a, 1);
                        menu_enable(0x9c97, 1);
                        FUN_1003fe90();
                        mech_trace::gw!("10014570", "1009821c", DAT_1009821c, 1);
                    }
                    _ => {}
                }
                return mech_trace::rt!("10014570", 0);
            }
            return mech_trace::rt!(
                "10014570",
                mech_trace::cr!(
                    "10014570",
                    "DefWindowProcA",
                    DefWindowProcA(param_1, param_2, param_3, param_4)
                )
            );
        }
        if param_2 < 0xc003 {
            if param_2 == 0xc002 {
                FUN_10011170(
                    0xc002,
                    mech_trace::gr!("10014570", "100b0f50", DAT_100b0f50),
                    &raw const s_exittos_1008ef18,
                );
                PostQuitMessage(0xc002);
                return mech_trace::rt!("10014570", 0);
            }
            if param_2 != 0x30f {
                if param_2 != 0x311 {
                    return mech_trace::rt!(
                        "10014570",
                        mech_trace::cr!(
                            "10014570",
                            "DefWindowProcA",
                            DefWindowProcA(param_1, param_2, param_3, param_4)
                        )
                    );
                }
                if mech_trace::gr!("10014570", "100d08e8", DAT_100d08e8) == param_1 {
                    return mech_trace::rt!("10014570", 0);
                }
            }
            if mech_trace::gr!("10014570", "100a672c", DAT_100a672c).is_null() {
                return mech_trace::rt!("10014570", 0);
            }
            FUN_100125f0(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
            if mech_trace::gr!("10014570", "100a6ba4", DAT_100a6ba4) == 0 {
                FUN_100124d0(
                    mech_trace::gr!("10014570", "100a672c", DAT_100a672c),
                    0,
                    0,
                    0x280,
                    0x1e0,
                );
                FUN_10012620(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
                return mech_trace::rt!("10014570", 1);
            }
            FUN_100124d0(
                mech_trace::gr!("10014570", "100a672c", DAT_100a672c),
                0,
                0,
                0x140,
                200,
            );
            FUN_10012830(mech_trace::gr!("10014570", "100a672c", DAT_100a672c));
            return mech_trace::rt!("10014570", 1);
        }
        match param_2 {
            0xc00a => {
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1004e0e0(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    mem(0x166),
                );
            }
            0xc010 => {
                chk(param_3);
                FUN_10007770(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    get(0x23c3),
                );
                set(0x23c7, param_2);
                set(0x23c3, 0xc01c);
            }
            0xc011 => {
                set(0x23c7, param_2);
                FUN_10011170(0xc00a, 0, mem(0x166));
                if get(0x23bb) != 2 {
                    FUN_1001d650();
                }
                FUN_100396a0(&raw const s_SHELLMER_CFG_1008ef08);
                PostQuitMessage(0xc001);
            }
            0xc012 => {
                set(0x23c7, param_2);
                set(0x23c3, param_3);
                chk(param_3);
                FUN_100361e0(0, 0, 3, 1, 100);
                FUN_1001a980(mem(0x166), 0, 0);
                FUN_100361e0(1, -1, -1, -1, -1);
                FUN_10013490(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    mem(0x166),
                );
            }
            0xc013 => {
                unhide();
                FUN_1000f360();
                set(0x23cb, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10041780(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc016 => {
                chk(param_3);
                set(0x23c3, param_3);
                set(0x23c7, param_2);
                unhide();
                if param_3 != 0xc02d {
                    let k = get(0x23bb);
                    if ((k == 1 && mech_trace::dr!("10014570", mem(0x23a2)) == 0) || k == 0)
                        && get(0x23c3) != 0xc036
                        && get(0x23c3) != 0xc02e
                    {
                        let sel = mech_trace::dr!("10014570", mem(0x239c));
                        if sel == 0 {
                            let q = (get(0x239e) as u32)
                                .wrapping_shl(4)
                                .wrapping_add(
                                    mech_trace::gr!("10014570", "10083508", DAT_10083508) as u32
                                )
                                .wrapping_add(0x2a2) as usize
                                as *mut u8;
                            FUN_10037e50(q);
                            let q = (get(0x239e) as u32)
                                .wrapping_shl(4)
                                .wrapping_add(
                                    mech_trace::gr!("10014570", "10083508", DAT_10083508) as u32
                                )
                                .wrapping_add(0x2a2) as usize
                                as *mut u8;
                            FUN_10038710(q);
                        } else if sel == 1 {
                            let q = (get(0x239e) as u32)
                                .wrapping_shl(4)
                                .wrapping_add(
                                    mech_trace::gr!("10014570", "10083508", DAT_10083508) as u32
                                )
                                .wrapping_add(0x202) as usize
                                as *mut u8;
                            FUN_10037e50(q);
                            let q = (get(0x239e) as u32)
                                .wrapping_shl(4)
                                .wrapping_add(
                                    mech_trace::gr!("10014570", "10083508", DAT_10083508) as u32
                                )
                                .wrapping_add(0x202) as usize
                                as *mut u8;
                            FUN_10038710(q);
                        }
                        if get(0x23bb) == 0 {
                            FUN_1000c250();
                            FUN_1000c2b0();
                            FUN_1000c410();
                        }
                    }
                    mech_trace::gw!(
                        "10014570",
                        "100b0f44",
                        _DAT_100b0f44,
                        mech_trace::cr!("10014570", "1007d270", FUN_1007d270(mem(0x166)))
                    );
                }
                FUN_10059880(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc018 => {
                set(0x23c3, param_3);
                set(0x23c7, param_2);
                FUN_10051340(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc01a => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10044090(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc01b => {
                set(0x23c3, 0xc01b);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1006e050(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc01c => {
                unhide();
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_100361e0(1, 0, 0, 0, 100);
                mech_trace::gw!("10014570", "100b0f50", DAT_100b0f50, 0);
                FUN_1001c350(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc01d => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10060c50(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc01e => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10044a60(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc01f => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10018250(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc020 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_100367d0(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc021 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10042920(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc022 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10057a90(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc024 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10040bf0(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc025 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1001db60(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    get(0x23c3),
                );
            }
            0xc026 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1003cc80(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc028 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1006c210(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    get(0x23c3),
                );
            }
            0xc029 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1001afe0(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc02b => {
                mech_trace::gw!("10014570", "100b0f50", DAT_100b0f50, 0);
                set(0x23c7, param_2);
                chk(param_3);
                FUN_1005bd10(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
                unhide();
            }
            0xc02e => {
                set(0x23c3, param_3);
                menu_enable(0x9c97, 1);
                FUN_1003a690(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    get(0x23c3),
                );
            }
            0xc02f => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10033090(
                    mech_trace::gr!("10014570", "100a6734", DAT_100a6734),
                    get(0x23cf),
                );
            }
            0xc030 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_1003bca0(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc031 => {
                set(0x23c3, param_3);
                chk(param_3);
                set(0x23c7, param_2);
                FUN_10011710(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc032 => {
                set(0x23c3, param_3);
                set(0x23c7, param_2);
                chk(param_3);
                FUN_10016d80(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc033 => {
                mech_trace::gw!("10014570", "1008ee10", _DAT_1008ee10, 1);
                unhide();
                FUN_1000e970(&raw const DAT_1008eeb0, 0xc016, 0xc016);
            }
            0xc034 => {
                FUN_1000e970(&raw const s_mland_1008eea0, 0xc01c, 0xc01c);
            }
            0xc035 => {
                FUN_1000e970(&raw const s_mland2_1008eea8, 0xc01c, 0xc01c);
            }
            0xc036 => {
                chk(param_3);
                set(0x23c3, param_3);
                set(0x23c7, param_2);
                FUN_10043230(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            0xc037 => {
                mech_trace::cr!(
                    "10014570",
                    "1005b1b0",
                    FUN_1005b1b0(&raw const s_You_cannot_afford_even_the_cheap_1008eeb4, 0)
                );
                FUN_1000fe40();
                FUN_1000e970(&raw const s_mpoorw_1008ef00, 0xc02b, 0xc01c);
            }
            0xc038 => {
                set(0x23c3, param_3);
                set(0x23c7, param_2);
                FUN_10050f10(mech_trace::gr!("10014570", "100a6734", DAT_100a6734));
            }
            _ => {
                return mech_trace::rt!(
                    "10014570",
                    mech_trace::cr!(
                        "10014570",
                        "DefWindowProcA",
                        DefWindowProcA(param_1, param_2, param_3, param_4)
                    )
                );
            }
        }
        mech_trace::rt!("10014570", 0)
    }
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000f360();
}
