// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100adf78: u32;
    static mut DAT_100adf7c: u32;
    static mut DAT_100adf80: u32;
    static mut DAT_100b34d4: u32;
    static mut DAT_100b34d8: u32;
    static mut DAT_10114d90: [u8; 16];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005efd3(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_1005ed0c(param_1: u32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ClipCursor(rect: *const c_void) -> i32;
    fn GetAsyncKeyState(key: i32) -> i16;
    fn GetCursorPos(pt: *mut c_void) -> i32;
    fn ScreenToClient(hwnd: *mut c_void, pt: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ed99(param_1: u32, param_2: *mut i32, param_3: *mut u32) -> u32 {
    mech_trace::enter!("1005ed99", param_1, param_2, param_3);
    // SAFETY: globals and callees are the game's own; param_2 and param_3
    // point at caller memory, accessed where the original does.
    unsafe {
        if mech_trace::gr!("1005ed99", "100adf80", DAT_100adf80) == 0 {
            if mech_trace::gr!("1005ed99", "100b34d4", DAT_100b34d4) != 0 {
                mech_trace::cr!("1005ed99", "ClipCursor", ClipCursor(core::ptr::null()));
                mech_trace::gw!("1005ed99", "100b34d4", DAT_100b34d4, 0);
            }
            mech_trace::wr!("1005ed99", param_3, write_unaligned, 0);
        } else {
            if mech_trace::gr!("1005ed99", "100b34d4", DAT_100b34d4) == 0
                || mech_trace::gr!("1005ed99", "100b34d8", DAT_100b34d8) != 0
            {
                FUN_1005efd3(
                    (&raw mut DAT_10114d90).cast::<i32>(),
                    mech_trace::gr!("1005ed99", "100adf78", DAT_100adf78) as i32,
                    mech_trace::gr!("1005ed99", "100adf7c", DAT_100adf7c) as i32,
                );
                mech_trace::cr!(
                    "1005ed99",
                    "ClipCursor",
                    ClipCursor((&raw mut DAT_10114d90).cast::<c_void>())
                );
                mech_trace::cr!("1005ed99", "1005ed0c", FUN_1005ed0c(param_1, 0));
                mech_trace::cr!("1005ed99", "1005ed0c", FUN_1005ed0c(param_1, 1));
                mech_trace::gw!("1005ed99", "100b34d4", DAT_100b34d4, 1);
                mech_trace::gw!("1005ed99", "100b34d8", DAT_100b34d8, 0);
            }
            if !param_3.is_null() {
                let s = mech_trace::cr!("1005ed99", "GetAsyncKeyState", GetAsyncKeyState(1));
                let local_18: u16 = ((s as i32 as u32 & 0x8000) != 0) as u16;
                let s = mech_trace::cr!("1005ed99", "GetAsyncKeyState", GetAsyncKeyState(2));
                let local_8: u16 = ((s as i32 as u32 & 0x8000) != 0) as u16;
                let s = mech_trace::cr!("1005ed99", "GetAsyncKeyState", GetAsyncKeyState(4));
                let local_14: u16 = ((s as i32 as u32 & 0x8000) != 0) as u16;
                let v = ((local_8 as i16 as i32).wrapping_shl(2))
                    | (local_14 as i16 as i32).wrapping_mul(2)
                    | (local_18 as i16 as i32);
                mech_trace::wr!("1005ed99", param_3, write_unaligned, v as u32);
            }
            if !param_2.is_null() {
                let mut pt: [i32; 2] = [0, 0];
                let ptr = pt.as_mut_ptr().cast::<c_void>();
                if mech_trace::cr!("1005ed99", "GetCursorPos", GetCursorPos(ptr)) != 0 {
                    mech_trace::cr!(
                        "1005ed99",
                        "ScreenToClient",
                        ScreenToClient(mech_trace::gr!("1005ed99", "100adf6c", DAT_100adf6c), ptr)
                    );
                    let x = pt[0];
                    let y = pt[1];
                    let ok = if x < 0
                        || (mech_trace::gr!("1005ed99", "100adf78", DAT_100adf78) as i32) <= x
                    {
                        false
                    } else if y < 0
                        || (mech_trace::gr!("1005ed99", "100adf7c", DAT_100adf7c) as i32) <= y
                    {
                        false
                    } else {
                        true
                    };
                    if ok {
                        let h = mech_trace::gr!("1005ed99", "100adf7c", DAT_100adf7c);
                        let w = mech_trace::gr!("1005ed99", "100adf78", DAT_100adf78);
                        let a = (y.wrapping_mul(2) as u32)
                            .wrapping_sub(h)
                            .wrapping_mul(0x10000)
                            / h;
                        mech_trace::wr!("1005ed99", param_2, write_unaligned, a as i32);
                        let b = (x.wrapping_mul(2) as u32)
                            .wrapping_sub(w)
                            .wrapping_mul(0x10000)
                            / w;
                        mech_trace::wr!("1005ed99", param_2.byte_add(4), write_unaligned, b as i32);
                    }
                }
            }
        }
        mech_trace::rt!("1005ed99", 0)
    }
}
