// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10167658: i32;
    static mut DAT_100adcec: i32;
    static mut DAT_100adcf0: *mut c_void;
    static mut DAT_1016765c: *mut c_void;
    static mut DAT_10167e70: i32;
    static mut DAT_1016828c: i32;
    static mut DAT_10168288: *mut c_void;
    static mut DAT_10167660: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086951(size: u32) -> *mut c_void;
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
    fn FUN_1008699d(ptr: *mut c_void, value: i32, size: u32);
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10035b4e(this: *mut c_void);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100359f0(this: *mut c_void, edx: i32) -> u64;
    fn FUN_10035aa3(this: *mut c_void, edx: i32) -> u64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035e40(param_1: i32, param_2: i32) -> u64 {
    mech_trace::enter!("10035e40", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; the allocated
    // block is used as the original does.
    unsafe {
        mech_trace::gw!("10035e40", "10167658", DAT_10167658, param_2);
        mech_trace::gw!(
            "10035e40",
            "100adcec",
            DAT_100adcec,
            param_1.wrapping_shl(10)
        );
        let size = param_2
            .wrapping_mul(0x40)
            .wrapping_add(param_1.wrapping_mul(0x400)) as u32;
        mech_trace::gw!(
            "10035e40",
            "100adcf0",
            DAT_100adcf0,
            mech_trace::cr!("10035e40", "10086951", FUN_10086951(size))
        );
        if mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0).is_null() {
            FUN_100108a0(0x18, core::ptr::null_mut());
        }
        FUN_1008699d(
            mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0),
            0,
            size,
        );
        mech_trace::gw!(
            "10035e40",
            "1016765c",
            DAT_1016765c,
            mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0)
        );
        mech_trace::gw!(
            "10035e40",
            "10167e70",
            DAT_10167e70,
            mech_trace::gr!("10035e40", "100adcec", DAT_100adcec).wrapping_add(mech_trace::gr!(
                "10035e40",
                "100adcf0",
                DAT_100adcf0
            )
                as i32)
        );
        mech_trace::gw!(
            "10035e40",
            "1016828c",
            DAT_1016828c,
            mech_trace::gr!("10035e40", "10167658", DAT_10167658)
                .wrapping_mul(0x20)
                .wrapping_add(mech_trace::gr!("10035e40", "10167e70", DAT_10167e70))
        );
        mech_trace::gw!(
            "10035e40",
            "10168288",
            DAT_10168288,
            mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0)
        );
        mech_trace::gw!(
            "10035e40",
            "10167660",
            DAT_10167660,
            (mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0) as i32)
                .wrapping_add(
                    mech_trace::gr!("10035e40", "100adcec", DAT_100adcec).wrapping_mul(0xc)
                )
                .wrapping_sub(0x240) as *mut c_void
        );
        FUN_10035b4e(mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0));
        let u_var2 = mech_trace::cr!(
            "10035e40",
            "100359f0",
            FUN_100359f0(mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0), 0)
        );
        mech_trace::rt!(
            "10035e40",
            mech_trace::cr!(
                "10035e40",
                "10035aa3",
                FUN_10035aa3(
                    mech_trace::gr!("10035e40", "100adcf0", DAT_100adcf0),
                    (u_var2 >> 0x20) as i32
                )
            )
        )
    }
}
