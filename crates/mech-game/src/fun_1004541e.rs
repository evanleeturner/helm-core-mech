// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100affd8: i32;
    static mut DAT_100afbc0: i32;
    static mut DAT_100afbc4: i32;
    static mut DAT_10160bf4: i32;
    static mut DAT_100b9d30: *mut core::ffi::c_void;
    static mut DAT_100adf74: i32;
    static mut DAT_100adf6c: i32;
    static mut DAT_100affc8: i32;
    static mut DAT_100affcc: i32;
    static mut DAT_100affd0: i32;
    static mut DAT_100b9a2c: *mut i32;
    static mut DAT_100affd4: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SelectObject(param_1: i32, param_2: i32) -> i32;
    fn DeleteObject(param_1: i32) -> i32;
    fn HeapFree(param_1: i32, param_2: u32, param_3: *mut core::ffi::c_void) -> i32;
    fn ReleaseDC(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004541e() -> i32 {
    // SAFETY: the globals and the imported Win32 functions are the game's
    // own; they are read and written in the original's order.
    unsafe {
        DAT_100affd8 = 0;
        if DAT_100afbc0 != 0 {
            if DAT_100afbc4 != 0 {
                SelectObject(DAT_100afbc0, DAT_10160bf4);
                DeleteObject(DAT_100afbc4);
            }
            if !DAT_100b9d30.is_null() {
                HeapFree(DAT_100adf74, 1, DAT_100b9d30);
            }
            ReleaseDC(DAT_100adf6c, DAT_100afbc0);
        }
        DAT_100afbc0 = 0;
        DAT_100afbc4 = 0;
        DAT_100affc8 = 0;
        DAT_100affcc = 0;
        DAT_100affd0 = 0;
        DAT_100b9d30 = core::ptr::null_mut();
        *DAT_100b9a2c = 0;
        DAT_100affd4 = 0;
        0
    }
}
