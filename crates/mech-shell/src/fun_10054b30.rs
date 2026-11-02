// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bd990: u8;
    static mut DAT_100d08e8: *mut c_void;
    static mut DAT_100a03a4: u8;
    static mut DAT_100a0400: u8;
    static mut DAT_100a014c: u8;
    static mut DAT_100a02d8: u8;
    static mut DAT_100a0084: u8;
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

unsafe extern "stdcall" {
    fn MessageBoxA(
        hwnd: *mut c_void,
        text: *const c_void,
        caption: *const c_void,
        utype: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054b30(param_1: i32, param_2: u32, param_3: u32) {
    mech_trace::enter!("10054b30", param_1, param_2, param_3);
    let _ = param_2;
    // SAFETY: the globals, the format strings and the buffer are the game's own
    // memory; sprintf and MessageBoxA are called as the original does.
    unsafe {
        let buf = &raw mut DAT_100bd990 as *mut i8;
        let fmt: *const i8;
        if param_1 == 100 {
            fmt = &raw const DAT_100a014c as *const i8;
        } else if param_1 == 0x65 {
            mech_trace::cr!(
                "10054b30",
                "sprintf",
                sprintf(buf, &raw const DAT_100a02d8 as *const i8, param_3)
            );
            mech_trace::cr!(
                "10054b30",
                "MessageBoxA",
                MessageBoxA(
                    mech_trace::gr!("10054b30", "100d08e8", DAT_100d08e8),
                    &raw const DAT_100bd990 as *const c_void,
                    &raw const DAT_100a0400 as *const c_void,
                    0x40,
                )
            );
            return;
        } else if param_1 == 0x66 {
            fmt = &raw const DAT_100a0084 as *const i8;
        } else {
            fmt = &raw const DAT_100a03a4 as *const i8;
        }
        mech_trace::cr!("10054b30", "sprintf", sprintf(buf, fmt));
        mech_trace::cr!(
            "10054b30",
            "MessageBoxA",
            MessageBoxA(
                mech_trace::gr!("10054b30", "100d08e8", DAT_100d08e8),
                &raw const DAT_100bd990 as *const c_void,
                &raw const DAT_100a0400 as *const c_void,
                0x40,
            )
        );
    }
}
