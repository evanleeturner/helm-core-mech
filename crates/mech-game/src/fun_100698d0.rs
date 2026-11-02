// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a0c: i32;
    static mut DAT_100b3a74: i32;
    static mut DAT_100b3a24: i32;
    static mut DAT_100b3a20: i32;
    static mut DAT_100b3a48: i32;
    static mut DAT_100b3a50: i32;
    static mut DAT_100b3a54: i32;
    static mut DAT_100b3a5c: i32;
    static mut PTR_DAT_100aaf68: *mut i32;
    static mut DAT_100b3a60: i32;
    static mut DAT_100b3a68: i32;
    static mut DAT_100b3a6c: i32;
    static mut DAT_100b3a70: i32;
    static mut DAT_100b3a88: i32;
    static mut DAT_100b3a84: i32;
    static mut DAT_100b3a80: i32;
    static mut DAT_100b3a7c: i32;
    static mut DAT_100b38fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10069a27();
    fn FUN_1006a240();
    fn FUN_10069c65();
    fn FUN_10069b2e();
    fn FUN_10069e95();
    fn FUN_10069cb4();
    fn FUN_1006a09b();
    fn FUN_1006a1f1();
    fn FUN_1006a16d();
    fn FUN_1006a133();
    fn FUN_1006a0f7();
    fn FUN_100868d8();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069f4b(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100698d0() {
    mech_trace::enter!("100698d0");
    // SAFETY: the globals and callees are the game's own; they are accessed
    // in the same order as the original.
    unsafe {
        if mech_trace::gr!("100698d0", "100b3a0c", DAT_100b3a0c) != 0 {
            FUN_10069a27();
        }
        if mech_trace::gr!("100698d0", "100b3a74", DAT_100b3a74) != 0 {
            FUN_1006a240();
        }
        if mech_trace::gr!("100698d0", "100b3a24", DAT_100b3a24) == 0 {
            if mech_trace::gr!("100698d0", "100b3a20", DAT_100b3a20) != 0 {
                FUN_10069c65();
            }
        } else {
            FUN_10069b2e();
        }
        if mech_trace::gr!("100698d0", "100b3a48", DAT_100b3a48) == 0 {
            if mech_trace::gr!("100698d0", "100b3a50", DAT_100b3a50) != 0 {
                FUN_10069e95();
            }
        } else {
            FUN_10069cb4();
        }
        if mech_trace::gr!("100698d0", "100b3a54", DAT_100b3a54) == 0 {
            if mech_trace::gr!("100698d0", "100b3a5c", DAT_100b3a5c) != 0 {
                FUN_1006a09b();
            }
        } else {
            FUN_10069f4b(mech_trace::gr!(
                "100698d0",
                "PTR_DAT_100aaf68",
                PTR_DAT_100aaf68
            ));
        }
        if mech_trace::gr!("100698d0", "100b3a60", DAT_100b3a60) == 0 {
            if mech_trace::gr!("100698d0", "100b3a68", DAT_100b3a68) != 0 {
                FUN_1006a1f1();
            }
        } else {
            FUN_1006a16d();
        }
        if mech_trace::gr!("100698d0", "100b3a6c", DAT_100b3a6c) == 0 {
            if mech_trace::gr!("100698d0", "100b3a70", DAT_100b3a70) != 0 {
                FUN_1006a133();
            }
        } else {
            FUN_1006a0f7();
        }
        if mech_trace::gr!("100698d0", "100b3a88", DAT_100b3a88) != 0 {
            mech_trace::gw!("100698d0", "100b3a84", DAT_100b3a84, 1);
        }
        if mech_trace::gr!("100698d0", "100b3a80", DAT_100b3a80) != 0
            && mech_trace::gr!("100698d0", "100b3a7c", DAT_100b3a7c)
                < mech_trace::gr!("100698d0", "100b38fc", DAT_100b38fc)
        {
            mech_trace::gw!(
                "100698d0",
                "100b3a7c",
                DAT_100b3a7c,
                mech_trace::gr!("100698d0", "100b38fc", DAT_100b38fc).wrapping_add(0xb5)
            );
            FUN_100868d8();
        }
    }
}
