// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut c_void;
    static mut DAT_100b9d30: *const c_void;
    static mut DAT_100c9250: u8;
    static mut DAT_100c967c: u32;
    static mut DAT_100c9680: u32;
    static mut DAT_10160bec: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn StretchDIBits(
        hdc: *mut c_void,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
        bits: *const c_void,
        bmi: *const c_void,
        usage: u32,
        rop: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045777(xSrc: i32, param_2: i32, param_3: i32, param_4: i32) -> u32 {
    mech_trace::enter!("10045777", xSrc, param_2, param_3, param_4);
    // SAFETY: the globals and StretchDIBits are the game's own; the header
    // address is passed through as the original does.
    unsafe {
        let h = mech_trace::gr!("10045777", "100c9680", DAT_100c9680) as i32;
        let r = mech_trace::cr!(
            "10045777",
            "StretchDIBits",
            StretchDIBits(
                mech_trace::gr!("10045777", "100afbc0", DAT_100afbc0),
                0,
                0,
                mech_trace::gr!("10045777", "100c967c", DAT_100c967c) as i32,
                h,
                xSrc,
                h.wrapping_sub(param_4).wrapping_sub(1),
                param_3.wrapping_sub(xSrc).wrapping_add(1),
                param_4.wrapping_sub(param_2).wrapping_add(1),
                mech_trace::gr!("10045777", "100b9d30", DAT_100b9d30),
                (&raw mut DAT_100c9250) as *const c_void,
                1,
                0xcc0020,
            )
        );
        mech_trace::gw!("10045777", "10160bec", DAT_10160bec, r as u32);
        mech_trace::rt!(
            "10045777",
            if mech_trace::gr!("10045777", "10160bec", DAT_10160bec)
                == mech_trace::gr!("10045777", "100c9680", DAT_100c9680)
            {
                0
            } else {
                0xffffffff
            }
        )
    }
}
