// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f2fc: *mut c_void;
    static mut DAT_1008f300: *mut c_void;
    static mut DAT_1008f308: *mut c_void;
    static mut DAT_1008f30c: *mut c_void;
    static mut DAT_1008f310: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f7d0(this: *mut c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100175e0(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; each global is
    // read by value and passed as the callee's this pointer.
    unsafe {
        match param_1 {
            0 => {
                FUN_1005f7d0(DAT_1008f2fc, 0);
            }
            1 => {
                FUN_1005f7d0(DAT_1008f308, 0);
            }
            2 => {
                FUN_1005f7d0(DAT_1008f30c, 0);
            }
            3 => {
                FUN_1005f7d0(DAT_1008f300, 0);
            }
            4 => {
                FUN_1005f7d0(DAT_1008f310, 0);
            }
            _ => {}
        }
    }
}
