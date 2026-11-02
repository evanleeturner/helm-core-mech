// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dest: *mut core::ffi::c_void, src: *const i8, count: u32)
    -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045f0f(param_1: i32, source: *mut i8) {
    // SAFETY: param_1 is the address of the caller's record; the copy and the
    // terminating byte land at offsets 0x10..0x30 of it, as in the original.
    unsafe {
        _strncpy(
            (param_1.wrapping_add(0x10)) as *mut core::ffi::c_void,
            source as *const i8,
            0x20,
        );
        ((param_1.wrapping_add(0x2f)) as *mut i8).write(0);
    }
}
