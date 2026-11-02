// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093c7c: [i8; 0];
    static mut DAT_10098224: *mut c_void;
    static mut s__s_BWD_10093c74: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *mut i8, ...) -> i32;
    fn FID_conflict___wfopen(filename: *mut i8, mode: *mut i8) -> *mut c_void;
    fn FUN_1005bae0(param_1: *mut c_void) -> i32;
    fn _fread(dst: *mut c_void, size: usize, count: usize, file: *mut c_void) -> usize;
    fn _fclose(file: *mut c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, bytes: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100363c0(
    param_1: i32,
    param_2: u32,
    param_3: u32,
) -> *mut c_void {
    let _ = param_2;
    if param_1 != -2 {
        return core::ptr::null_mut();
    }
    let mut local_20 = [0i8; 32];
    // SAFETY: the callees and globals are the game's own; local_20 is a
    // writable buffer passed as the original does, and the file handle is
    // used only between its open and close.
    unsafe {
        _sprintf(
            local_20.as_mut_ptr(),
            &raw mut s__s_BWD_10093c74 as *mut i8,
            param_3,
        );
        let file = FID_conflict___wfopen(local_20.as_mut_ptr(), &raw mut DAT_10093c7c as *mut i8);
        if file.is_null() {
            return core::ptr::null_mut();
        }
        let dw_bytes = FUN_1005bae0(file) as u32;
        let dst = HeapAlloc(DAT_10098224, 1, dw_bytes);
        _fread(dst, dw_bytes as usize, 1, file);
        _fclose(file);
        dst
    }
}
