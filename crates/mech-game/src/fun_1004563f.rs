// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut c_void;
    static mut DAT_100c9680: i32;
    static mut DAT_100b9d30: *mut c_void;
    static mut DAT_100c9250: [u8; 1];
    static mut DAT_10160bec: i32;
    static mut DAT_100b0474: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SetDIBitsToDevice(
        hdc: *mut c_void,
        x_dest: i32,
        y_dest: i32,
        w: u32,
        h: u32,
        x_src: i32,
        y_src: i32,
        start_scan: u32,
        c_lines: u32,
        bits: *const c_void,
        bmi: *const c_void,
        usage: u32,
    ) -> i32;
    fn GetLastError() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004563f(
    param_1: i32,
    y_dest: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("1004563f", param_1, y_dest, param_3, param_4);
    // SAFETY: the globals and callees are the game's own; the pointers passed
    // are the game's DC, bits buffer and bitmap header.
    unsafe {
        mech_trace::gw!(
            "1004563f",
            "10160bec",
            DAT_10160bec,
            mech_trace::cr!(
                "1004563f",
                "SetDIBitsToDevice",
                SetDIBitsToDevice(
                    mech_trace::gr!("1004563f", "100afbc0", DAT_100afbc0),
                    param_1,
                    y_dest,
                    (param_3 as u32)
                        .wrapping_sub(param_1 as u32)
                        .wrapping_add(1),
                    (param_4 as u32).wrapping_sub(y_dest as u32).wrapping_add(1),
                    param_1,
                    (mech_trace::gr!("1004563f", "100c9680", DAT_100c9680) as u32)
                        .wrapping_sub(param_4 as u32)
                        .wrapping_add(0xffff_ffff) as i32,
                    0,
                    mech_trace::gr!("1004563f", "100c9680", DAT_100c9680) as u32,
                    mech_trace::gr!("1004563f", "100b9d30", DAT_100b9d30) as *const c_void,
                    (&raw mut DAT_100c9250) as *const c_void,
                    1,
                )
            )
        );
        if mech_trace::gr!("1004563f", "10160bec", DAT_10160bec) == 0 {
            FUN_10002168(
                (&raw mut DAT_100b0474).cast::<i8>(),
                mech_trace::cr!("1004563f", "GetLastError", GetLastError()),
            );
        }
        mech_trace::rt!(
            "1004563f",
            if mech_trace::gr!("1004563f", "10160bec", DAT_10160bec)
                == mech_trace::gr!("1004563f", "100c9680", DAT_100c9680)
            {
                0
            } else {
                0xffff_ffff
            }
        )
    }
}
