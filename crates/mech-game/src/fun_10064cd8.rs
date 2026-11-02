// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7f9c: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fwrite(
        _str: *mut core::ffi::c_void,
        _size: u32,
        _count: u32,
        _file: *mut core::ffi::c_void,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064cd8(param_1: *mut core::ffi::c_void, param_2: u32) -> u32 {
    // SAFETY: the callee and the global are the game's own; param_1 points at
    // the caller's buffer of param_2 bytes.
    unsafe {
        let s_var1 = _fwrite(param_1, 1, param_2, DAT_100c7f9c);
        if s_var1 < param_2 { 2 } else { 0 }
    }
}
