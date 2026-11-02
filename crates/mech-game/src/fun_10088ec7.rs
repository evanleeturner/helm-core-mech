// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c0f64: i32;
    static mut DAT_100c8508: [i8; 1];
    static mut DAT_100b9d4c: i32;
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100c27a4: [i8; 1];
    static mut DAT_100c27ac: [i8; 1];
    static mut DAT_100c28d8: [i8; 1];
    static mut DAT_100c29c8: [i8; 1];
    static mut DAT_100c2b20: [i8; 1];
    static mut DAT_100c2bcc: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005dd5a(code: *mut i8);
    fn FUN_10010a8e();
    fn _exit(code: i32) -> !;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ShowWindow(hwnd: *mut c_void, cmd: i32) -> i32;
    fn MessageBoxA(hwnd: *mut c_void, text: *const i8, caption: *const i8, utype: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10088ec7(code: i32, param_2: u32, param_3: *mut i8) {
    // SAFETY: the globals, strings and callees are the game's own memory and
    // functions; param_3 points at the caller's NUL-terminated string.
    unsafe {
        if DAT_100c0f64 == 0 {
            DAT_100c0f64 = 1;
            let mut i_var1: i32 = 6;
            let mut b_var4: bool = true;
            let mut pc_var2: *const i8 = (&raw mut DAT_100c27a4).cast::<i8>().cast_const();
            let mut pc_var3: *const i8 = param_3.cast_const();
            loop {
                if i_var1 == 0 {
                    break;
                }
                i_var1 = i_var1.wrapping_sub(1);
                b_var4 = *pc_var2 == *pc_var3;
                pc_var2 = pc_var2.add(1);
                pc_var3 = pc_var3.add(1);
                if !b_var4 {
                    break;
                }
            }
            if !b_var4 {
                let buf = (&raw mut DAT_100c8508).cast::<i8>();
                if code == 0x6e {
                    sprintf(
                        buf,
                        (&raw mut DAT_100c27ac).cast::<i8>().cast_const(),
                        param_3,
                        param_3,
                    );
                } else if code == 0x6f {
                    sprintf(
                        buf,
                        (&raw mut DAT_100c28d8).cast::<i8>().cast_const(),
                        param_2,
                        param_3,
                    );
                } else if code == 0x70 {
                    sprintf(
                        buf,
                        (&raw mut DAT_100c29c8).cast::<i8>().cast_const(),
                        param_3,
                        param_3,
                    );
                } else {
                    sprintf(buf, (&raw mut DAT_100c2b20).cast::<i8>().cast_const());
                }
                FUN_1005dd5a(buf);
                if DAT_100b9d4c == 1 {
                    ShowWindow(DAT_100adf6c, 7);
                }
                let r = MessageBoxA(
                    DAT_100adf6c,
                    buf.cast_const(),
                    (&raw mut DAT_100c2bcc).cast::<i8>().cast_const(),
                    0x41,
                );
                if r == 2 {
                    FUN_10010a8e();
                    _exit(code);
                }
                if DAT_100b9d4c == 1 {
                    ShowWindow(DAT_100adf6c, 9);
                }
            }
        }
    }
}
