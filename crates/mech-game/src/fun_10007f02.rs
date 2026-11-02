// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5058: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007440(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007f02(param_1: i32) {
    if -1 < param_1 && param_1 < 3 {
        // SAFETY: the global and the callee are the game's own; the offset
        // stays within the three-entry table the original indexes.
        unsafe {
            FUN_10007440(
                (&raw mut DAT_100a5058)
                    .cast::<u8>()
                    .byte_add(param_1.wrapping_mul(0xc) as usize)
                    .cast::<core::ffi::c_void>(),
                core::ptr::null_mut(),
                -1,
            );
        }
    }
}
