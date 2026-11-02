// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b5c: *mut c_void;
    static mut DAT_100a7b64: u32;
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100adf80: u32;
    static mut DAT_100b9a1c: u32;
    static mut DAT_101f52e0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003e1b7();
    fn FUN_10035e10();
    fn FUN_10081cb8();
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000ed10() {
    // SAFETY: the callees and globals are the game's own memory; the
    // function pointer table at DAT_100b9a1c + 0x18 is read and called as the original does.
    unsafe {
        FUN_1003e1b7();
        FUN_10035e10();
        if !DAT_100a7b5c.is_null() && DAT_101f52e0 != 0 {
            FUN_10058e74(&raw mut DAT_101f52e0 as *mut i32, 0);
            if DAT_100adf80 != 0 {
                let f = ((DAT_100b9a1c.wrapping_add(0x18)) as *const u32).read_unaligned();
                let f: unsafe extern "C" fn() = core::mem::transmute(f as usize);
                f();
            }
        }
        if !DAT_100a7b5c.is_null() {
            HeapFree(DAT_100adf74, 1, DAT_100a7b5c);
        }
        DAT_100a7b64 = 0;
        FUN_10081cb8();
    }
}
