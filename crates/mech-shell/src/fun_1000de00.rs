// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut core::ffi::c_void;
    static mut DAT_1008d154: *mut core::ffi::c_void;
    static mut DAT_1008d558: *mut core::ffi::c_void;
    static mut DAT_1008d55c: *mut core::ffi::c_void;
    static mut DAT_1008d560: *mut core::ffi::c_void;
    static mut DAT_1008d564: i32;
    static mut DAT_1008d568: i32;
    static mut DAT_100a7560: i32;
    static mut DAT_100a725c: *mut i32;
    static mut DAT_100d08e8: *mut core::ffi::c_void;
    static mut DAT_100d1544: *mut core::ffi::c_void;
    static mut DAT_100d154c: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SelectObject(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void);
    fn DeleteObject(param_1: *mut core::ffi::c_void);
    fn DeleteDC(param_1: *mut core::ffi::c_void);
    fn CloseHandle(param_1: *mut core::ffi::c_void);
    fn ReleaseDC(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000de00() {
    // SAFETY: the globals and the Win32 callees are the game's own; the
    // handles are used and cleared exactly as the original does.
    unsafe {
        DAT_1008d568 = 0;
        if !DAT_1008d150.is_null() {
            if !DAT_1008d154.is_null() {
                SelectObject(DAT_1008d150, DAT_100d154c);
                DeleteObject(DAT_1008d154);
            }
            if !DAT_1008d55c.is_null() {
                SelectObject(DAT_1008d55c, DAT_100d1544);
                DeleteObject(DAT_1008d560);
                DeleteDC(DAT_1008d55c);
                if !DAT_1008d558.is_null() {
                    CloseHandle(DAT_1008d558);
                }
            }
            ReleaseDC(DAT_100d08e8, DAT_1008d150);
        }
        DAT_1008d150 = core::ptr::null_mut();
        DAT_1008d154 = core::ptr::null_mut();
        DAT_1008d558 = core::ptr::null_mut();
        DAT_1008d55c = core::ptr::null_mut();
        DAT_1008d560 = core::ptr::null_mut();
        DAT_100a7560 = 0;
        *DAT_100a725c = 0;
        DAT_1008d564 = 0;
    }
}
