// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb4: u32;
    static mut DAT_100aefb8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003c260() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c67c() {
    // SAFETY: the globals and the callee are the game's own memory and code,
    // read, written and called in the original's order.
    unsafe {
        if DAT_100aefb8 != 0 {
            if DAT_100aefb4 == 0 {
                FUN_1003c260();
            }
            DAT_100aefb4 = 1;
            DAT_100aefb8 = 0;
        }
    }
}
