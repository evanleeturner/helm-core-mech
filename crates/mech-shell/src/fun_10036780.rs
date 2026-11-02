// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b529c: i32;
    static mut DAT_10093c60: i32;
    static mut DAT_10093c84: i8;
    static mut DAT_100b5298: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(filename: *mut i8, mode: *mut i8) -> *mut core::ffi::c_void;
    fn _fread(
        dst: *mut core::ffi::c_void,
        size: i32,
        count: i32,
        stream: *mut core::ffi::c_void,
    ) -> i32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036780(param_1: *mut i8) {
    mech_trace::enter!("10036780", param_1);
    // SAFETY: the globals and the C runtime functions are the game's own;
    // param_1 is the caller's file name string.
    unsafe {
        mech_trace::gw!(
            "10036780",
            "100b529c",
            DAT_100b529c,
            mech_trace::gr!("10036780", "10093c60", DAT_10093c60)
        );
        let file_stream = mech_trace::cr!(
            "10036780",
            "__wfopen",
            __wfopen(param_1, &raw mut DAT_10093c84)
        );
        if !file_stream.is_null() {
            mech_trace::cr!(
                "10036780",
                "_fread",
                _fread(
                    (&raw mut DAT_100b5298).cast::<core::ffi::c_void>(),
                    1,
                    mech_trace::gr!("10036780", "10093c60", DAT_10093c60),
                    file_stream,
                )
            );
            mech_trace::cr!("10036780", "_fclose", _fclose(file_stream));
        }
    }
}
