// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab618: i32;
    static mut DAT_100ab61c: i32;
    static mut DAT_100ab620: i32;
    static mut DAT_100ab624: i32;
    static mut DAT_100ab628: i32;
    static mut DAT_100ab62c: i32;
    static mut DAT_100ab630: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006e020() {
    // SAFETY: the globals are the game's own memory; each is written by value.
    unsafe {
        DAT_100ab618 = 0;
        DAT_100ab61c = 0;
        DAT_100ab620 = 0;
        DAT_100ab624 = 0;
        DAT_100ab628 = 0;
        DAT_100ab62c = 0;
        DAT_100ab630 = 0;
    }
}
