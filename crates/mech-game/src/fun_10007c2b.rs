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
    // SAFETY: the globals and the callee are the game's own; arguments
    // match the original call.
    unsafe {
        if (DAT_100b3c58 & 2) == 0 {
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
            );
        } else {
            FUN_10002ba7(
                0,
                0,
                0x31,
                core::ptr::null_mut(),
                100,
                DAT_100b3c50 as i32,
                0x40,
                0x2b11,
                -1,
                0x250,
            );
        }
    }
}
