// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut c_void;
    static mut DAT_100b9d30: *mut c_void;
    static mut DAT_100c9680: u32;
    static mut DAT_10160bec: i32;
    static mut DAT_100c9250: [u8; 0];
    static mut DAT_100b0490: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetSystemMetrics(n_index: i32) -> i32;
    fn GetLastError() -> u32;
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
        bits: *mut c_void,
        info: *mut c_void,
        usage: u32,
    ) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(fmt: *mut i8, ...);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100456d9(
    x_dest: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("100456d9", x_dest, param_2, param_3, param_4);
    let h = (param_4 as u32)
        .wrapping_sub(param_2 as u32)
        .wrapping_add(1);
    let w = (param_3 as u32).wrapping_sub(x_dest as u32).wrapping_add(1);
    // SAFETY: the callees and globals are the game's own memory; the
    // arguments are passed exactly as the original passes them.
    unsafe {
        let i_var1 = mech_trace::cr!("100456d9", "GetSystemMetrics", GetSystemMetrics(0xf));
        mech_trace::gw!(
            "100456d9",
            "10160bec",
            DAT_10160bec,
            mech_trace::cr!(
                "100456d9",
                "SetDIBitsToDevice",
                SetDIBitsToDevice(
                    mech_trace::gr!("100456d9", "100afbc0", DAT_100afbc0),
                    x_dest,
                    (param_2 as u32).wrapping_sub(i_var1 as u32) as i32,
                    w,
                    h,
                    x_dest,
                    param_2,
                    0,
                    mech_trace::gr!("100456d9", "100c9680", DAT_100c9680),
                    mech_trace::gr!("100456d9", "100b9d30", DAT_100b9d30),
                    &raw mut DAT_100c9250 as *mut c_void,
                    1,
                )
            )
        );
        if mech_trace::gr!("100456d9", "10160bec", DAT_10160bec) == 0 {
            FUN_10002168(
                (&raw mut DAT_100b0490).cast::<i8>(),
                mech_trace::cr!("100456d9", "GetLastError", GetLastError()),
            );
        }
        mech_trace::rt!(
            "100456d9",
            if (mech_trace::gr!("100456d9", "10160bec", DAT_10160bec) as u32)
                == mech_trace::gr!("100456d9", "100c9680", DAT_100c9680)
            {
                0
            } else {
                0xffff_ffff
            }
        )
    }
}
