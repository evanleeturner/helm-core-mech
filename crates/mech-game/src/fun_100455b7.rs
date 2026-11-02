// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut core::ffi::c_void;
    static mut DAT_100c967c: i32;
    static mut DAT_100c9680: i32;
    static mut DAT_100b9d30: *mut core::ffi::c_void;
    static mut DAT_100c9250: u8;
    static mut DAT_10160bec: i32;
    static mut DAT_100b045c: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SetDIBitsToDevice(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: u32,
        param_6: i32,
        param_7: i32,
        param_8: u32,
        param_9: u32,
        param_10: *const core::ffi::c_void,
        param_11: *const core::ffi::c_void,
        param_12: u32,
    ) -> i32;
    fn GetLastError() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100455b7() -> u32 {
    mech_trace::enter!("100455b7");
    // SAFETY: the globals and callees are the game's own memory and the
    // system's GDI; they are used exactly as the original does.
    unsafe {
        mech_trace::gw!(
            "100455b7",
            "10160bec",
            DAT_10160bec,
            mech_trace::cr!(
                "100455b7",
                "SetDIBitsToDevice",
                SetDIBitsToDevice(
                    mech_trace::gr!("100455b7", "100afbc0", DAT_100afbc0),
                    0,
                    0,
                    mech_trace::gr!("100455b7", "100c967c", DAT_100c967c) as u32,
                    mech_trace::gr!("100455b7", "100c9680", DAT_100c9680) as u32,
                    0,
                    0,
                    0,
                    mech_trace::gr!("100455b7", "100c9680", DAT_100c9680) as u32,
                    mech_trace::gr!("100455b7", "100b9d30", DAT_100b9d30)
                        as *const core::ffi::c_void,
                    (&raw mut DAT_100c9250) as *const core::ffi::c_void,
                    1,
                )
            )
        );
        if mech_trace::gr!("100455b7", "10160bec", DAT_10160bec) == 0 {
            FUN_10002168(
                (&raw mut DAT_100b045c).cast::<i8>(),
                mech_trace::cr!("100455b7", "GetLastError", GetLastError()),
            );
        }
        mech_trace::rt!(
            "100455b7",
            if mech_trace::gr!("100455b7", "10160bec", DAT_10160bec)
                == mech_trace::gr!("100455b7", "100c9680", DAT_100c9680)
            {
                0
            } else {
                0xffffffff
            }
        )
    }
}
