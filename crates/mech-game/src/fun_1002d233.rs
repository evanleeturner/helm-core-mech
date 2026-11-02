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
    mech_trace::enter!("1002d233");
    // SAFETY: all globals and callees are the game's own memory and code;
    // accesses happen in the same order as the original.
    unsafe {
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a654",
            _DAT_1016a654,
            i32::from(mech_trace::gr!("1002d233", "1016a638", DAT_1016a638)).wrapping_mul(0x250)
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a650",
            _DAT_1016a650,
            mech_trace::gr!("1002d233", "100ad6b8", DAT_100ad6b8)
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a65c",
            _DAT_1016a65c,
            mech_trace::cr!("1002d233", "10022aaa", FUN_10022aaa())
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a65c",
            _DAT_1016a65c,
            mech_trace::gr!("1002d233", "_DAT_1016a65c", _DAT_1016a65c).wrapping_mul(i32::from(
                mech_trace::gr!("1002d233", "1016a638", DAT_1016a638)
            ))
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a658",
            _DAT_1016a658,
            mech_trace::gr!("1002d233", "100ad6bc", DAT_100ad6bc)
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a664",
            _DAT_1016a664,
            mech_trace::cr!("1002d233", "10030398", FUN_10030398())
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a664",
            _DAT_1016a664,
            mech_trace::gr!("1002d233", "_DAT_1016a664", _DAT_1016a664).wrapping_mul(i32::from(
                mech_trace::gr!("1002d233", "1016a63c", DAT_1016a63c)
            ))
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a660",
            _DAT_1016a660,
            mech_trace::gr!("1002d233", "100ad6c0", DAT_100ad6c0)
        );
        let i_var1 = mech_trace::cr!("1002d233", "1005dbf1", FUN_1005dbf1());
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a66c",
            _DAT_1016a66c,
            i_var1.wrapping_mul(i32::from(mech_trace::gr!(
                "1002d233",
                "1016a644",
                DAT_1016a644
            )))
        );
        mech_trace::gw!(
            "1002d233",
            "1016a668",
            DAT_1016a668[0],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[0])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a668",
            DAT_1016a668[1],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[1])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a668",
            DAT_1016a668[2],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[2])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a668",
            DAT_1016a668[3],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[3])
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a674",
            _DAT_1016a674,
            i32::from(mech_trace::gr!("1002d233", "1016a644", DAT_1016a644)).wrapping_mul(0x2c)
        );
        mech_trace::gw!(
            "1002d233",
            "1016a670",
            DAT_1016a670[0],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[4])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a670",
            DAT_1016a670[1],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[5])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a670",
            DAT_1016a670[2],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[6])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a670",
            DAT_1016a670[3],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[7])
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a67c",
            _DAT_1016a67c,
            i32::from(mech_trace::gr!("1002d233", "1016a646", DAT_1016a646)).wrapping_mul(0x14)
        );
        mech_trace::gw!(
            "1002d233",
            "1016a678",
            DAT_1016a678[0],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[8])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a678",
            DAT_1016a678[1],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[9])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a678",
            DAT_1016a678[2],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[10])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a678",
            DAT_1016a678[3],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[11])
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a684",
            _DAT_1016a684,
            mech_trace::gr!("1002d233", "1016a648", DAT_1016a648)
        );
        mech_trace::gw!(
            "1002d233",
            "1016a680",
            DAT_1016a680[0],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[12])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a680",
            DAT_1016a680[1],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[13])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a680",
            DAT_1016a680[2],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[14])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a680",
            DAT_1016a680[3],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[15])
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a68c",
            _DAT_1016a68c,
            i32::from(mech_trace::gr!("1002d233", "1016a63e", DAT_1016a63e)).wrapping_shl(3)
        );
        mech_trace::gw!(
            "1002d233",
            "1016a688",
            DAT_1016a688[0],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[16])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a688",
            DAT_1016a688[1],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[17])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a688",
            DAT_1016a688[2],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[18])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a688",
            DAT_1016a688[3],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[19])
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a694",
            _DAT_1016a694,
            i32::from(mech_trace::gr!("1002d233", "1016a63e", DAT_1016a63e)).wrapping_shl(2)
        );
        mech_trace::gw!(
            "1002d233",
            "1016a690",
            DAT_1016a690[0],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[20])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a690",
            DAT_1016a690[1],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[21])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a690",
            DAT_1016a690[2],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[22])
        );
        mech_trace::gw!(
            "1002d233",
            "1016a690",
            DAT_1016a690[3],
            mech_trace::gr!("1002d233", "100ad6c4", DAT_100ad6c4[23])
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a69c",
            _DAT_1016a69c,
            i32::from(mech_trace::gr!("1002d233", "1016a63e", DAT_1016a63e)).wrapping_shl(2)
        );
        mech_trace::gw!(
            "1002d233",
            "_DAT_1016a698",
            _DAT_1016a698,
            mech_trace::gr!("1002d233", "100ad6dc", DAT_100ad6dc)
        );
        mech_trace::rt!("1002d233", &raw mut _DAT_1016a650 as *mut c_void)
    }
}
