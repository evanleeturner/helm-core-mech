// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static s_mw2rmg_cfg_1008d078: [i8; 0];
    static DAT_1008d074: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(filename: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fread(
        ptr: *mut core::ffi::c_void,
        size: u32,
        nmemb: u32,
        stream: *mut core::ffi::c_void,
    ) -> u32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cf60(param_1: *mut core::ffi::c_void) -> i32 {
    mech_trace::enter!("1000cf60", param_1);
    // SAFETY: the callees and the string data are the game's own; param_1 is
    // the caller's buffer, filled by fread exactly as the original does.
    unsafe {
        let cfg_file = mech_trace::cr!(
            "1000cf60",
            "__wfopen",
            __wfopen(
                (&raw const s_mw2rmg_cfg_1008d078).cast::<i8>(),
                (&raw const DAT_1008d074).cast::<i8>(),
            )
        );
        if !cfg_file.is_null() {
            mech_trace::cr!("1000cf60", "_fread", _fread(param_1, 0xf3, 1, cfg_file));
            mech_trace::cr!("1000cf60", "_fclose", _fclose(cfg_file));
            return mech_trace::rt!("1000cf60", 1);
        }
        mech_trace::rt!("1000cf60", 0)
    }
}
