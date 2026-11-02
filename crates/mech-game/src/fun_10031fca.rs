// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada90: *mut u32;
    static mut DAT_101686e8: u32;
    static mut DAT_100ada98: u32;
    static mut DAT_100adf74: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086951(param_1: u32) -> *mut c_void;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031fca(param_1: *mut u32) -> u32 {
    let mut param_1 = param_1;
    let mut local_8: u32 = 0;
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a buffer whose second dword gives its byte length, copied as the original does.
    unsafe {
        if !DAT_100ada90.is_null() {
            HeapFree(DAT_100adf74, 1, DAT_100ada90 as *mut c_void);
        }
        DAT_100ada90 = core::ptr::null_mut();
        DAT_101686e8 = 0;
        DAT_100ada98 = 0;
        DAT_100ada90 = FUN_10086951(param_1.add(1).read_unaligned()) as *mut u32;
        if DAT_100ada90.is_null() {
            FUN_100108a0(0x3f, core::ptr::null_mut());
        } else {
            let mut u_var2 = param_1.add(1).read_unaligned();
            let mut puvar3 = DAT_100ada90;
            let mut u_var1 = u_var2 >> 2;
            while u_var1 != 0 {
                puvar3.write_unaligned(param_1.read_unaligned());
                param_1 = param_1.add(1);
                puvar3 = puvar3.add(1);
                u_var1 = u_var1.wrapping_sub(1);
            }
            u_var2 &= 3;
            while u_var2 != 0 {
                (puvar3 as *mut u8).write((param_1 as *const u8).read());
                param_1 = (param_1 as *mut u8).add(1) as *mut u32;
                puvar3 = (puvar3 as *mut u8).add(1) as *mut u32;
                u_var2 = u_var2.wrapping_sub(1);
            }
            DAT_101686e8 = DAT_100ada90.add(1).read_unaligned().wrapping_sub(8) / 0xc;
            local_8 = 1;
        }
    }
    local_8
}
