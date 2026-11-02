// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1016a638: i16;
    static mut DAT_1016a63c: i16;
    static mut DAT_1016a63e: i16;
    static mut DAT_1016a644: i16;
    static mut DAT_1016a646: i16;
    static mut DAT_1016a648: i32;

    static mut DAT_100ad6b8: i32;
    static mut DAT_100ad6bc: i32;
    static mut DAT_100ad6c0: i32;
    static mut DAT_100ad6dc: i32;
    static mut DAT_100ad6c4: [i8; 24];

    static mut _DAT_1016a650: i32;
    static mut _DAT_1016a654: i32;
    static mut _DAT_1016a658: i32;
    static mut _DAT_1016a65c: i32;
    static mut _DAT_1016a660: i32;
    static mut _DAT_1016a664: i32;
    static mut DAT_1016a668: [i8; 4];
    static mut _DAT_1016a66c: i32;
    static mut DAT_1016a670: [i8; 4];
    static mut _DAT_1016a674: i32;
    static mut DAT_1016a678: [i8; 4];
    static mut _DAT_1016a67c: i32;
    static mut DAT_1016a680: [i8; 4];
    static mut _DAT_1016a684: i32;
    static mut DAT_1016a688: [i8; 4];
    static mut _DAT_1016a68c: i32;
    static mut DAT_1016a690: [i8; 4];
    static mut _DAT_1016a694: i32;
    static mut _DAT_1016a698: i32;
    static mut _DAT_1016a69c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10022aaa() -> i32;
    fn FUN_10030398() -> i32;
    fn FUN_1005dbf1() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1002d233() -> *mut c_void {
    // SAFETY: all globals and callees are the game's own memory and code;
    // accesses happen in the same order as the original.
    unsafe {
        _DAT_1016a654 = i32::from(DAT_1016a638).wrapping_mul(0x250);
        _DAT_1016a650 = DAT_100ad6b8;
        _DAT_1016a65c = FUN_10022aaa();
        _DAT_1016a65c = _DAT_1016a65c.wrapping_mul(i32::from(DAT_1016a638));
        _DAT_1016a658 = DAT_100ad6bc;
        _DAT_1016a664 = FUN_10030398();
        _DAT_1016a664 = _DAT_1016a664.wrapping_mul(i32::from(DAT_1016a63c));
        _DAT_1016a660 = DAT_100ad6c0;
        let i_var1 = FUN_1005dbf1();
        _DAT_1016a66c = i_var1.wrapping_mul(i32::from(DAT_1016a644));
        DAT_1016a668[0] = DAT_100ad6c4[0];
        DAT_1016a668[1] = DAT_100ad6c4[1];
        DAT_1016a668[2] = DAT_100ad6c4[2];
        DAT_1016a668[3] = DAT_100ad6c4[3];
        _DAT_1016a674 = i32::from(DAT_1016a644).wrapping_mul(0x2c);
        DAT_1016a670[0] = DAT_100ad6c4[4];
        DAT_1016a670[1] = DAT_100ad6c4[5];
        DAT_1016a670[2] = DAT_100ad6c4[6];
        DAT_1016a670[3] = DAT_100ad6c4[7];
        _DAT_1016a67c = i32::from(DAT_1016a646).wrapping_mul(0x14);
        DAT_1016a678[0] = DAT_100ad6c4[8];
        DAT_1016a678[1] = DAT_100ad6c4[9];
        DAT_1016a678[2] = DAT_100ad6c4[10];
        DAT_1016a678[3] = DAT_100ad6c4[11];
        _DAT_1016a684 = DAT_1016a648;
        DAT_1016a680[0] = DAT_100ad6c4[12];
        DAT_1016a680[1] = DAT_100ad6c4[13];
        DAT_1016a680[2] = DAT_100ad6c4[14];
        DAT_1016a680[3] = DAT_100ad6c4[15];
        _DAT_1016a68c = i32::from(DAT_1016a63e).wrapping_shl(3);
        DAT_1016a688[0] = DAT_100ad6c4[16];
        DAT_1016a688[1] = DAT_100ad6c4[17];
        DAT_1016a688[2] = DAT_100ad6c4[18];
        DAT_1016a688[3] = DAT_100ad6c4[19];
        _DAT_1016a694 = i32::from(DAT_1016a63e).wrapping_shl(2);
        DAT_1016a690[0] = DAT_100ad6c4[20];
        DAT_1016a690[1] = DAT_100ad6c4[21];
        DAT_1016a690[2] = DAT_100ad6c4[22];
        DAT_1016a690[3] = DAT_100ad6c4[23];
        _DAT_1016a69c = i32::from(DAT_1016a63e).wrapping_shl(2);
        _DAT_1016a698 = DAT_100ad6dc;
        &raw mut _DAT_1016a650 as *mut c_void
    }
}
