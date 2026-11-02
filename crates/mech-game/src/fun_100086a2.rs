// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215654: u32;
    static mut DAT_10215658: u32;
    static mut DAT_1021565c: u32;
    static mut DAT_10215660: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000895c() -> u8;
    fn FUN_100086f7() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100086a2() {
    // SAFETY: the globals and callees are the game's own memory and code,
    // accessed in the original's order.
    unsafe {
        DAT_10215654 = DAT_1021565c;
        DAT_1021565c = 4;
        if DAT_10215658 < 3 {
            FUN_1000895c();
        } else if DAT_10215658 == 4 && DAT_10215660 < 3 {
            FUN_100086f7();
        }
    }
}
