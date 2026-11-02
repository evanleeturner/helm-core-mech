// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c50: u32;
    static mut DAT_100b3c58: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002ba7(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: u32,
        param_9: i32,
        param_10: u16,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10007c2b() {
    mech_trace::enter!("10007c2b");
    // SAFETY: the globals and the callee are the game's own; arguments
    // match the original call.
    unsafe {
        if (mech_trace::gr!("10007c2b", "100b3c58", DAT_100b3c58) & 2) == 0 {
            mech_trace::cr!(
                "10007c2b",
                "10002ba7",
                FUN_10002ba7(
                    0,
                    0,
                    0x4f,
                    core::ptr::null_mut(),
                    100,
                    0x10000,
                    0x40,
                    0x2b11,
                    -1,
                    0x250,
                )
            );
        } else {
            mech_trace::cr!(
                "10007c2b",
                "10002ba7",
                FUN_10002ba7(
                    0,
                    0,
                    0x31,
                    core::ptr::null_mut(),
                    100,
                    mech_trace::gr!("10007c2b", "100b3c50", DAT_100b3c50) as i32,
                    0x40,
                    0x2b11,
                    -1,
                    0x250,
                )
            );
        }
    }
}
