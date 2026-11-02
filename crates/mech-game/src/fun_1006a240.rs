// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a9c: i32;
    static mut DAT_100b3aa0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a240() {
    // SAFETY: the globals are the game's own memory, read and written in the original's order.
    unsafe {
        let i_var1 = DAT_100b3aa0;
        DAT_100b3a9c = DAT_100b3a9c.wrapping_sub(1);
        if DAT_100b3a9c == 0 {
            DAT_100b3a9c = 4;
            DAT_100b3aa0 = DAT_100b3aa0.wrapping_sub(1);
            if i_var1 == 0 {
                DAT_100b3aa0 = 3;
            }
        }
    }
}
