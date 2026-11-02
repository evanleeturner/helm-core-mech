// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00408048: i32;
    static mut DAT_0040804c: *mut c_void;
    static mut DAT_00408050: u32;
    static mut DAT_00408158: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __vsnprintf(buf: *mut i8, size: u32, format: *const i8, args: *mut i8) -> i32;
    fn FUN_00401160(format: *mut i8);
    fn FUN_004011a0(format: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn OutputDebugStringA(s: *const i8);
    fn MessageBoxA(hwnd: *mut c_void, text: *const i8, caption: *const i8, utype: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401200(format: *mut i8) {
    mech_trace::enter!("00401200", format);
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the callees and globals are the game's own; the va_list is the
    // address of the stack slot following `format`, as in the original, and
    // local_100 is a 256-byte buffer matching the size passed.
    unsafe {
        let args = (&raw const format).add(1) as *mut i8;
        mech_trace::cr!(
            "00401200",
            "__vsnprintf",
            __vsnprintf(local_100.as_mut_ptr(), 0x100, format, args)
        );

        match mech_trace::gr!("00401200", "00408048", DAT_00408048) {
            1 => {
                FUN_00401160(local_100.as_mut_ptr());
            }
            2 => {
                OutputDebugStringA(local_100.as_ptr());
            }
            3 => {
                if !mech_trace::gr!("00401200", "0040804c", DAT_0040804c).is_null() {
                    mech_trace::cr!(
                        "00401200",
                        "MessageBoxA",
                        MessageBoxA(
                            mech_trace::gr!("00401200", "0040804c", DAT_0040804c),
                            local_100.as_ptr(),
                            &raw mut DAT_00408158 as *const i8,
                            mech_trace::gr!("00401200", "00408050", DAT_00408050),
                        )
                    );
                }
            }
            4 => {
                FUN_004011a0(local_100.as_mut_ptr());
            }
            _ => {}
        }
    }
}
