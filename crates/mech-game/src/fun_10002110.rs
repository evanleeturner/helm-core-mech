// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10218080: [i8; 256];
    static mut DAT_100adf6c: *mut c_void;
    static mut s_Mecenaries__Legacy_Message_100a4060: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __vsnprintf(format: *mut i8, param_2: u32, param_3: *const i8, param_4: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn OutputDebugStringA(lpOutputString: *const i8);
    fn MessageBoxA(hWnd: *mut c_void, lpText: *const i8, lpCaption: *const i8, uType: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002110(format: *const i8) {
    // SAFETY: the globals and callees are the game's own; the variadic
    // arguments sit on the stack directly above `format`, so the address of
    // the next 4-byte slot is the va_list the original passes.
    unsafe {
        let args = (&raw const format).add(1) as *mut i8;
        __vsnprintf(&raw mut DAT_10218080 as *mut i8, 0x100, format, args);
        OutputDebugStringA(&raw const DAT_10218080 as *const i8);
        MessageBoxA(
            DAT_100adf6c,
            &raw const DAT_10218080 as *const i8,
            &raw const s_Mecenaries__Legacy_Message_100a4060 as *const i8,
            0x40,
        );
    }
}
