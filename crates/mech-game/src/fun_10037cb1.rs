// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[repr(C)]
struct TagPoint {
    x: i32,
    y: i32,
}

#[repr(C)]
struct TagMsg {
    hwnd: *mut c_void,
    message: u32,
    w_param: u32,
    l_param: i32,
    time: u32,
    pt: TagPoint,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf28: u32;
    static mut DAT_100adf80: u32;
    static mut DAT_100adf94: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn WaitMessage() -> i32;
    fn PeekMessageA(msg: *mut c_void, hwnd: *mut c_void, min: u32, max: u32, remove: u32) -> i32;
    fn TranslateMessage(msg: *const c_void) -> i32;
    fn DispatchMessageA(msg: *const c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037cb1() {
    // SAFETY: the globals and Win32 calls are the game's own; local_20 is a
    // valid MSG buffer that is only read after PeekMessageA has filled it.
    unsafe {
        let mut local_20: TagMsg = core::mem::zeroed();
        let p = &raw mut local_20;
        if DAT_100adf80 == 0 {
            WaitMessage();
        }
        if DAT_100adf28 == 0 {
            let b_var1 = PeekMessageA(p.cast(), core::ptr::null_mut(), 0, 0, 1);
            if b_var1 != 0 {
                while DAT_100adf94 == 0 && 0x1ff < (*p).message && (*p).message < 0x20a {
                    PeekMessageA(p.cast(), core::ptr::null_mut(), 0, 0, 1);
                }
                if (*p).hwnd.is_null() || (*p).message != 0x12 {
                    TranslateMessage(p.cast());
                    DispatchMessageA(p.cast());
                } else {
                    DAT_100adf28 = 1;
                }
            }
        }
    }
}
