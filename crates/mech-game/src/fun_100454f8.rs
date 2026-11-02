// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut c_void;
    static mut DAT_100affcc: *mut c_void;
    static mut DAT_100affd0: *mut c_void;
    static mut DAT_100b9a2c: *mut *mut c_void;
    static mut DAT_100b9d30: *mut c_void;
    static mut DAT_10160bec: u32;
    static mut DAT_10160bf0: *mut c_void;
    static mut DAT_100c9250: u8;
    static mut DAT_100afbc8: u8;
    static mut DAT_100b0438: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, ...);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn CreateDIBSection(
        hdc: *mut c_void,
        pbmi: *const c_void,
        usage: u32,
        ppv_bits: *mut *mut c_void,
        h_section: *mut c_void,
        offset: u32,
    ) -> *mut c_void;
    fn SetDIBColorTable(
        hdc: *mut c_void,
        i_start: u32,
        c_entries: u32,
        prgbq: *const c_void,
    ) -> u32;
    fn SelectObject(hdc: *mut c_void, h: *mut c_void) -> *mut c_void;
    fn GetLastError() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100454f8() -> u32 {
    // SAFETY: all globals and imported functions are the game's own; the
    // bits pointer is only dereferenced after CreateDIBSection returned and
    // the original's null checks have been applied.
    unsafe {
        if DAT_100affd0.is_null() {
            DAT_100affd0 = CreateDIBSection(
                DAT_100afbc0,
                (&raw mut DAT_100c9250) as *const c_void,
                1,
                DAT_100b9a2c,
                core::ptr::null_mut(),
                0,
            );
            if DAT_100affd0.is_null() || (*DAT_100b9a2c).is_null() {
                FUN_10002168(&raw mut DAT_100b0438, GetLastError());
                return 0;
            }
            DAT_100b9d30 = *DAT_100b9a2c;
            DAT_10160bf0 = SelectObject(DAT_100affcc, DAT_100affd0);
        } else {
            DAT_10160bec = SetDIBColorTable(
                DAT_100affcc,
                0,
                0x100,
                (&raw mut DAT_100afbc8) as *const c_void,
            );
        }
        1
    }
}
