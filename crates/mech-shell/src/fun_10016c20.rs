// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edf4: *const c_void;
    static mut DAT_1008edf8: u32;
    static mut DAT_1008edfc: *const c_void;
    static mut DAT_1008ee00: *mut c_void;
    static mut DAT_100a6730: *mut i32;
    static mut DAT_100a6734: u32;
    static mut DAT_100b0f44: [u8; 0];
    static mut DAT_100b0f4c: u32;
    static mut DAT_100b0f50: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ee30() -> u8;
    fn FUN_10016bd0() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005f850(param_1: *mut c_void) -> bool;
    fn FUN_1005edd0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10016c20() {
    // SAFETY: the globals, callbacks and callees are the game's own memory
    // and code; the callbacks are called with the arguments the original passes.
    unsafe {
        if !DAT_1008edfc.is_null() {
            let f: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_1008edfc);
            f(1);
            return;
        }
        if !DAT_1008edf4.is_null() {
            let f: unsafe extern "C" fn(u32, *mut u8, *mut u8, u32) =
                core::mem::transmute(DAT_1008edf4);
            f(
                DAT_100a6734,
                (&raw mut DAT_100b0f50).cast::<u8>(),
                (&raw mut DAT_100b0f44).cast::<u8>(),
                DAT_100b0f4c,
            );
            FUN_1000ee30();
            return;
        }
        if DAT_1008edf8 == 0 {
            return;
        }
        let check_result = FUN_1005f850(DAT_1008ee00);
        if !check_result {
            FUN_10016bd0();
            return;
        }
        FUN_1005edd0(DAT_100a6730);
    }
}
