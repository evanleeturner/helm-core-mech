// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static LAB_1000e7f0: u8;
    static mut DAT_1008e3e4: u32;
    static mut DAT_1008e3e8: u32;
    static mut DAT_1008e450: u8;
    static mut DAT_1008e448: u8;
    static mut DAT_1008edf4: *mut u8;
    static mut DAT_100a6838: u32;
    static mut DAT_100a6ba4: u8;
    static mut DAT_100b0588: u32;
    static mut DAT_100d08e8: *mut c_void;
    static mut DAT_100d08ec: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PostMessageA(hwnd: *mut c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000f940(entry_name: u32);
    fn FUN_1000e8f0(entry_name: u32, msg: u32, w_param: u32) -> u32;
    fn FUN_100166c0(entry_name: *mut c_void);
}

unsafe fn differs(a: *const u8, b: *const u8) -> bool {
    let mut i: usize = 0;
    loop {
        // SAFETY: both pointers are NUL-terminated strings; reading stops at the first NUL or mismatch.
        let (x, y) = unsafe {
            (
                mech_trace::dr!("1000e970", a.add(i)),
                mech_trace::dr!("1000e970", b.add(i)),
            )
        };
        if x != y {
            return true;
        }
        if x == 0 {
            return false;
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e970(entry_name: *mut u8, msg: u32, w_param: u32) -> u32 {
    mech_trace::enter!("1000e970", entry_name, msg, w_param);
    // SAFETY: globals and callees are the game's own; entry_name is a NUL-terminated string.
    unsafe {
        if mech_trace::gr!("1000e970", "100a6838", DAT_100a6838) == 0 {
            if differs(entry_name, &raw const DAT_1008e448)
                && differs(entry_name, &raw const DAT_1008e450)
                && msg != 0xc004
            {
                mech_trace::cr!(
                    "1000e970",
                    "PostMessageA",
                    PostMessageA(
                        mech_trace::gr!("1000e970", "100d08e8", DAT_100d08e8),
                        msg,
                        w_param,
                        0
                    )
                );
                mech_trace::gw!("1000e970", "1008e3e8", DAT_1008e3e8, 0xc004);
                mech_trace::gw!("1000e970", "1008e3e4", DAT_1008e3e4, 0xc004);
                return mech_trace::rt!("1000e970", 0);
            }
        }
        mech_trace::gw!("1000e970", "100b0588", DAT_100b0588, msg);
        if mech_trace::gr!("1000e970", "1008edf4", DAT_1008edf4) as *const u8
            == &raw const LAB_1000e7f0
        {
            FUN_1000f940(entry_name as u32);
            return mech_trace::rt!("1000e970", 1);
        }
        let result = mech_trace::cr!(
            "1000e970",
            "1000e8f0",
            FUN_1000e8f0(entry_name as u32, msg, w_param)
        );
        if result != 0 {
            FUN_100166c0(mech_trace::gr!("1000e970", "100d08ec", DAT_100d08ec));
            mech_trace::gw!("1000e970", "100a6ba4", DAT_100a6ba4, 1);
            return mech_trace::rt!("1000e970", 1);
        }
        mech_trace::rt!("1000e970", 0)
    }
}
