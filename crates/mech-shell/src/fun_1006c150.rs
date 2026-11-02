// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa6b0: i32;
    static mut DAT_100aa6b4: i32;
    static mut DAT_100aa6b8: i32;
    static mut DAT_100aa6bc: i32;
    static mut DAT_100aa6c0: i32;
    static mut DAT_100aa6c4: i32;
    static mut DAT_100aa6c8: i32;
    static mut DAT_100aa6cc: i32;
    static mut DAT_100aa6d0: i32;
    static mut DAT_100aa6d4: i32;
    static mut DAT_100aa6d8: i32;
    static mut DAT_100aa6dc: i32;
    static mut DAT_100aa6e0: i32;
    static mut DAT_100aa6e4: i32;
    static mut DAT_100ab048: i32;
    static mut DAT_100ab04c: i32;
    static mut DAT_100ab050: i32;
    static mut DAT_100ab054: i32;
    static mut DAT_100ab058: i32;
    static mut DAT_100ab05c: i32;
    static mut DAT_100ab060: i32;
    static mut DAT_100ab064: i32;
    static mut DAT_100ab068: i32;
    static mut DAT_100ab06c: i32;
    static mut DAT_100ab070: i32;
    static mut DAT_100ab074: i32;
    static mut DAT_100ab078: u8;
    static mut DAT_100c56b8: u8;
    static mut DAT_100ab07c: i32;
    static mut DAT_100ab080: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006c150() {
    // SAFETY: the globals are the game's own memory; each is written once
    // in the original's order.
    unsafe {
        DAT_100aa6b0 = 0;
        DAT_100aa6b4 = 0;
        DAT_100aa6b8 = 0;
        DAT_100aa6bc = 0;
        DAT_100aa6c0 = 0;
        DAT_100aa6c4 = 0;
        DAT_100aa6c8 = 0;
        DAT_100aa6cc = 0;
        DAT_100aa6d0 = 0;
        DAT_100aa6d4 = 0;
        DAT_100aa6d8 = 0;
        DAT_100aa6e0 = 0;
        DAT_100aa6e4 = 0;
        DAT_100ab048 = 0;
        DAT_100ab04c = 0;
        DAT_100ab050 = 0;
        DAT_100ab054 = 0;
        DAT_100ab058 = 0;
        DAT_100ab05c = 0;
        DAT_100ab060 = 0;
        DAT_100ab064 = 0;
        DAT_100ab068 = 0;
        DAT_100ab06c = 0;
        DAT_100ab070 = 0;
        DAT_100ab074 = 0;
        DAT_100ab078 = 0;
        DAT_100c56b8 = 0;
        DAT_100ab07c = 0;
        DAT_100aa6dc = -1;
        DAT_100ab080 = 0xc004;
    }
}
