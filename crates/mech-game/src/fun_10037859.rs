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
    // SAFETY: globals, callees and the pointers they hold are the game's own;
    // l_param points at the caller's structure where the original writes it.
    unsafe {
        if msg > 0xff && msg < 0x109 {
            FUN_10070276(msg, w_param, l_param);
            return 0;
        }
        if msg < 0x10 {
            if msg == 0xf {
                if DAT_100adf84 != 0 && DAT_100b9d4c == 2 {
                    let tbl = DAT_100b9a1c.wrapping_add(0x18) as *const u32;
                    let f: unsafe extern "C" fn() =
                        core::mem::transmute(tbl.read_unaligned() as usize);
                    f();
                    ValidateRect(wnd, core::ptr::null());
                    return 0;
                }
            } else if msg == 2 {
                PostQuitMessage(0);
                return 0;
            }
        } else if msg < 0x1d {
            if msg == 0x1c {
                if DAT_100adf28 == 0 {
                    DAT_100adf80 = w_param;
                    if w_param == 1 {
                        FUN_10070665();
                        if DAT_100b9d58 < 0x281 && DAT_100b9d5c < 0x1e1 {
                            if DAT_100b9d54 == 1 {
                                FUN_10081fdc();
                                DAT_100b9d54 = 0;
                                ShowWindow(DAT_100adf6c, 9);
                            } else if !DAT_100b9a18.is_null() && DAT_100b9a18.read_unaligned() == 1
                            {
                                ShowWindow(DAT_100adf6c, 4);
                            }
                        } else if DAT_100b9d4c == 1 {
                            ShowWindow(DAT_100adf6c, 9);
                        }
                    } else if DAT_100b9d58 < 0x281 && DAT_100b9d5c < 0x1e1 {
                        if DAT_100b9a18.is_null()
                            || DAT_100b9a18.read_unaligned() != 0
                            || DAT_100b9d54 != 0
                        {
                            if !DAT_100b9a18.is_null() && DAT_100b9a18.read_unaligned() == 1 {
                                ShowWindow(DAT_100adf6c, 0);
                            }
                        } else {
                            ShowWindow(DAT_100adf6c, 6);
                            DAT_100b9d54 = 1;
                            FUN_10081fdc();
                        }
                    } else {
                        ShowWindow(DAT_100adf6c, 6);
                    }
                    if !DAT_100b9a18.is_null() {
                        let fp = DAT_100b9a18.add(5).read_unaligned() as u32;
                        let f: unsafe extern "C" fn(i32, i32, *mut u8, i32) =
                            core::mem::transmute(fp as usize);
                        f(0, 0x100, &raw mut DAT_100b9a30 as *mut u8, 1);
                    }
                    return 0;
                }
            } else if msg == 0x10 {
                DAT_100adf2c = DAT_100adf2c.wrapping_add(2);
                DAT_100adf28 = 1;
                return 0;
            }
        } else if msg < 0xa1 {
            if msg == 0xa0 {
                if DAT_100adf94 == 0 && DAT_100b9d4c == 2 {
                    loop {
                        let r = ShowCursor(1);
                        if r >= 0 {
                            break;
                        }
                    }
                    DAT_100adf94 = 1;
                }
            } else if msg == 0x46 && DAT_100adf90 != 0 {
                let t = GetMessageTime() as u32;
                if DAT_1016593c < t && (DAT_1016584c < DAT_1016593c || t < DAT_1016584c) {
                    DAT_100adf90 = 0;
                } else {
                    let p = l_param as *mut u8;
                    p.byte_add(8).cast::<u32>().write_unaligned(DAT_100c9240);
                    p.byte_add(0xc).cast::<u32>().write_unaligned(DAT_100c9244);
                    p.byte_add(0x10).cast::<u32>().write_unaligned(DAT_100c9248);
                    p.byte_add(0x14).cast::<u32>().write_unaligned(DAT_100c924c);
                }
                return 0;
            }
        } else {
            if msg == 0x200 {
                if DAT_100adf94 != 0 && (DAT_100adf88 == 0 || FUN_100498c5(4) == 1) {
                    loop {
                        let r = ShowCursor(0);
                        if r < 0 {
                            break;
                        }
                    }
                    DAT_100adf94 = 0;
                }
                return 0;
            }
            if msg == 0x30f {
                if !DAT_100b9a18.is_null() {
                    let fp = DAT_100b9a18.add(5).read_unaligned() as u32;
                    let f: unsafe extern "C" fn(i32, i32, *mut u8, i32) =
                        core::mem::transmute(fp as usize);
                    f(0, 0x100, &raw mut DAT_100b9a30 as *mut u8, 1);
                    return 1;
                }
                return 0;
            }
        }
        DefWindowProcA(wnd, msg, w_param, l_param as i32)
    }
}
