// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static s_MW2SND_CFG_100a50d0: [i8; 0];
    static DAT_100a50cc: [i8; 0];
    static mut DAT_100a67f0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(
        filename: *const core::ffi::c_void,
        mode: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn FUN_1006ece0(
        ptr: *mut core::ffi::c_void,
        size: i32,
        count: i32,
        stream: *mut core::ffi::c_void,
    ) -> i32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005abb0() {
    mech_trace::enter!("1005abb0");
    // SAFETY: the callees and globals are the game's own; the buffer is the
    // game's 0x54-byte config area, filled only if the file opened.
    unsafe {
        let file = mech_trace::cr!(
            "1005abb0",
            "__wfopen",
            __wfopen(
                (&raw const s_MW2SND_CFG_100a50d0).cast(),
                (&raw const DAT_100a50cc).cast(),
            )
        );
        if !file.is_null() {
            mech_trace::cr!(
                "1005abb0",
                "1006ece0",
                FUN_1006ece0((&raw mut DAT_100a67f0).cast(), 0x54, 1, file)
            );
            mech_trace::cr!("1005abb0", "_fclose", _fclose(file));
        }
    }
}
