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
    mech_trace::enter!("10064cd8", param_1, param_2);
    // SAFETY: the callee and the global are the game's own; param_1 points at
    // the caller's buffer of param_2 bytes.
    unsafe {
        let s_var1 = mech_trace::cr!(
            "10064cd8",
            "_fwrite",
            _fwrite(
                param_1,
                1,
                param_2,
                mech_trace::gr!("10064cd8", "100c7f9c", DAT_100c7f9c)
            )
        );
        mech_trace::rt!("10064cd8", if s_var1 < param_2 { 2 } else { 0 })
    }
}
