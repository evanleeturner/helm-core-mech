// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046f04(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004633a() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004652f() {
    // SAFETY: the global and the callees are the game's own; they are
    // called and written in the original's order.
    unsafe {
        if DAT_100b04c4 != 0 {
            FUN_10046f04(0x100b04b8);
            FUN_1004633a();
            DAT_100b04c4 = 0;
        }
    }
}
