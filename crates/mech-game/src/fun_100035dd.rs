// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c4c: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002ba7(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
        param_5: i32,
        param_6: *mut core::ffi::c_void,
        param_7: u32,
        param_8: u32,
        param_9: i32,
        param_10: u16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100035dd(param_1: u32, param_2: i32, param_3: u32, param_4: u32) {
    // SAFETY: the callee and the global are the game's own; the arguments
    // are passed through as the original does.
    unsafe {
        FUN_10002ba7(
            0,
            0,
            param_1,
            core::ptr::null_mut(),
            param_2,
            DAT_100b3c4c,
            param_3,
            param_4,
            -1,
            0x450,
        );
    }
}
