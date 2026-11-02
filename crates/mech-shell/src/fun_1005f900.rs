// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a71e0: i32;
    static mut DAT_100a71e4: Option<unsafe extern "stdcall" fn()>;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005f900() {
    // SAFETY: the globals are the game's own memory; the callback is the
    // game's stdcall function pointer, called only when non-null.
    unsafe {
        if DAT_100a71e0 != 0 {
            DAT_100a71e0 = 0;
            if let Some(f) = DAT_100a71e4 {
                f();
                return;
            }
        }
    }
}
