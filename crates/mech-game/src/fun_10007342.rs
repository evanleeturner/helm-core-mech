// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41e0: i32;
    static mut DAT_100a41e4: *mut c_void;
    static mut DAT_100a41e8: u32;
    static mut s_DEBUG_Message_100a42f0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __vsnprintf(param_1: *mut i8, param_2: u32, param_3: *mut i8, param_4: *mut i8) -> i32;
    fn FUN_100071a9(param_1: *mut i8);
    fn FUN_10007218(param_1: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn OutputDebugStringA(param_1: *const i8);
    fn MessageBoxA(
        param_1: *mut c_void,
        param_2: *const i8,
        param_3: *const i8,
        param_4: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007342(format: *mut i8) {
    mech_trace::enter!("10007342", format);
    let mut local_104: [i8; 256] = [0; 256];
    // SAFETY: the callees and globals are the game's own; the va_list is the
    // address just past the format parameter on the caller's stack, and the
    // buffer is a local of the size passed.
    unsafe {
        let args = (&raw const format).cast::<u8>().byte_add(4) as *mut i8;
        mech_trace::cr!(
            "10007342",
            "__vsnprintf",
            __vsnprintf(local_104.as_mut_ptr(), 0x100, format, args)
        );
        match mech_trace::gr!("10007342", "100a41e0", DAT_100a41e0) {
            1 => {
                FUN_100071a9(local_104.as_mut_ptr());
            }
            2 => {
                OutputDebugStringA(local_104.as_ptr());
            }
            3 => {
                if !mech_trace::gr!("10007342", "100a41e4", DAT_100a41e4).is_null() {
                    mech_trace::cr!(
                        "10007342",
                        "MessageBoxA",
                        MessageBoxA(
                            mech_trace::gr!("10007342", "100a41e4", DAT_100a41e4),
                            local_104.as_ptr(),
                            (&raw mut s_DEBUG_Message_100a42f0).cast::<i8>(),
                            mech_trace::gr!("10007342", "100a41e8", DAT_100a41e8),
                        )
                    );
                }
            }
            4 => {
                FUN_10007218(local_104.as_mut_ptr());
            }
            _ => {}
        }
    }
}
