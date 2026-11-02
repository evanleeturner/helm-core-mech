// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9050: i32;
    static mut DAT_100b3a70: i32;
    static mut DAT_100b3bb8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_sprintf_placeholder();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a0f7() {
    let _ = ();
}
