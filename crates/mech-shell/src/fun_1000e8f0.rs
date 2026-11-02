// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e3e4: u32;
    static mut DAT_1008e3e8: u32;
    static mut DAT_10098220: u32;
    static mut DAT_100a672c: u32;
    static mut DAT_100d08e8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn LAB_1000e7f0();
    fn FUN_1000f6d0(a: i32, b: u32, c: u32, d: u32, e: u32, f: u32) -> i32;
    fn FUN_10016b50(a: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000f360();
    fn PostMessageA(h: *mut c_void, msg: u32, w: u32, l: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100133d0(a: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e8f0(param_1: u32, msg: u32, w_param: u32) -> u32 {
    // SAFETY: the callees and globals are the game's own memory and code.
    unsafe {
        FUN_1000f360();
        let i_var1 = FUN_1000f6d0(0, param_1, 0, 0, 0x1000, 0);
        if i_var1 == -1 {
            PostMessageA(DAT_100d08e8, msg, w_param, 0);
            return 0;
        }
        if DAT_10098220 != 0 {
            FUN_100133d0(DAT_100a672c);
        }
        DAT_1008e3e4 = msg;
        DAT_1008e3e8 = w_param;
        FUN_10016b50(LAB_1000e7f0 as usize as u32);
        1
    }
}
