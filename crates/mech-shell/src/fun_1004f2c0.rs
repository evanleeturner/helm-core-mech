// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009bdf8: i32;
    static mut DAT_1009bdfc: *mut c_void;
    static mut DAT_1009be00: u32;
    static mut DAT_1009bf08: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __vsnprintf(buf: *mut i8, count: u32, fmt: *const i8, args: *mut c_void) -> i32;
    fn FUN_1004f180(format: *mut i8);
    fn FUN_1004f1e0(format: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn OutputDebugStringA(lp_output_string: *const i8);
    fn MessageBoxA(
        h_wnd: *mut c_void,
        lp_text: *const i8,
        lp_caption: *const i8,
        u_type: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004f2c0(format: *mut i8) {
    mech_trace::enter!("1004f2c0", format);
    let mut local_100 = [0i8; 256];
    // SAFETY: on 32-bit cdecl the variadic arguments follow `format` on the
    // stack, so the address just past it is the va_list; the callees and
    // globals are the game's own, and local_100 is a 256-byte buffer.
    unsafe {
        let args = (&raw const format).add(1) as *mut c_void;
        mech_trace::cr!(
            "1004f2c0",
            "__vsnprintf",
            __vsnprintf(local_100.as_mut_ptr(), 0x100, format, args)
        );

        match mech_trace::gr!("1004f2c0", "1009bdf8", DAT_1009bdf8) {
            1 => {
                FUN_1004f180(local_100.as_mut_ptr());
            }
            2 => {
                OutputDebugStringA(local_100.as_ptr());
            }
            3 => {
                if !mech_trace::gr!("1004f2c0", "1009bdfc", DAT_1009bdfc).is_null() {
                    mech_trace::cr!(
                        "1004f2c0",
                        "MessageBoxA",
                        MessageBoxA(
                            mech_trace::gr!("1004f2c0", "1009bdfc", DAT_1009bdfc),
                            local_100.as_ptr(),
                            (&raw const DAT_1009bf08).cast::<i8>(),
                            mech_trace::gr!("1004f2c0", "1009be00", DAT_1009be00),
                        )
                    );
                }
            }
            4 => {
                FUN_1004f1e0(local_100.as_mut_ptr());
            }
            _ => {}
        }
    }
}
