// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e0: *mut core::ffi::c_void;
    static mut DAT_100b38e4: *mut core::ffi::c_void;
    static mut DAT_100b38e8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100654db() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: usize) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065555() -> u32 {
    // SAFETY: the callees and globals are the game's own; the globals hold
    // the allocations made here.
    unsafe {
        FUN_100654db();
        DAT_100b38e0 = _malloc(0x1000);
        if DAT_100b38e0.is_null() {
            FUN_100654db();
            return 3;
        }
        DAT_100b38e4 = _malloc(0x4000);
        if DAT_100b38e4.is_null() {
            FUN_100654db();
            return 3;
        }
        DAT_100b38e8 = _malloc(0x9bd4);
        if DAT_100b38e8.is_null() {
            FUN_100654db();
            return 3;
        }
        0
    }
}
