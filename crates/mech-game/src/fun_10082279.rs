// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100adf88: u32;
    static mut DAT_100adf90: u32;
    static mut DAT_100b9a18: *mut i32;
    static mut DAT_100b9d4c: u32;
    static mut DAT_100c9240: u32;
    static mut DAT_100c9244: u32;
    static mut DAT_100c9248: u32;
    static mut DAT_100c924c: u32;
    static mut DAT_100c967c: i32;
    static mut DAT_100c9680: i32;
    static mut DAT_1016584c: u32;
    static mut DAT_1016593c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SetWindowLongA(hwnd: *mut c_void, index: i32, value: i32) -> i32;
    fn timeGetTime() -> u32;
    fn SetWindowPos(
        hwnd: *mut c_void,
        insert_after: *mut c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: u32,
    ) -> i32;
    fn ShowCursor(show: i32) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100498c5(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10082279(param_1: i32) {
    if param_1 != 0 {
        let p = param_1 as *const u8;
        // SAFETY: param_1 points at the caller's structure; the globals and
        // callees are the game's own memory and functions.
        unsafe {
            let mode = p.byte_add(4).cast::<i32>().read_unaligned();
            let style = p.byte_add(8).cast::<u32>().read_unaligned();
            if mode == 2 {
                SetWindowLongA(DAT_100adf6c, -0x10, (style | 0x10000000) as i32);
            } else {
                SetWindowLongA(
                    DAT_100adf6c,
                    -0x10,
                    ((style & 0xfff7ffff) | 0x10000000) as i32,
                );
            }
            if p.byte_add(4).cast::<i32>().read_unaligned() == 2 {
                if !DAT_100b9a18.is_null() && *DAT_100b9a18 == 0 {
                    DAT_1016584c = timeGetTime();
                    DAT_1016593c = DAT_1016584c.wrapping_add(3000);
                    DAT_100adf90 = 1;
                }
                SetWindowPos(
                    DAT_100adf6c,
                    0xfffffffeu32 as usize as *mut c_void,
                    DAT_100c9240 as i32,
                    DAT_100c9244 as i32,
                    DAT_100c9248 as i32,
                    DAT_100c924c as i32,
                    0x10,
                );
                if DAT_100adf88 != 0 && FUN_100498c5(4) == 0 {
                    loop {
                        let r = ShowCursor(1);
                        if !(r < 0) {
                            break;
                        }
                    }
                }
            } else if DAT_100adf88 != 0 {
                loop {
                    let r = ShowCursor(0);
                    if !(-1 < r) {
                        break;
                    }
                }
            }
            let mut i_var1 = GetSystemMetrics(0);
            if DAT_100c967c < i_var1 || {
                i_var1 = GetSystemMetrics(1);
                DAT_100c9680 < i_var1
            } {
                DAT_100b9d4c = p.byte_add(4).cast::<u32>().read_unaligned();
            } else {
                DAT_100b9d4c = 1;
            }
        }
    }
}
