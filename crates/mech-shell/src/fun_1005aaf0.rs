// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a50a0: [u16; 0];
    static mut DAT_100a509c: [u16; 0];
    static mut DAT_100a67f0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN___wfopen(filename: *mut u16, mode: *mut u16) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005aaf0() {}
