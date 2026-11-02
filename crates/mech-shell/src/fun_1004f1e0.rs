// SPDX-License-Identifier: MIT

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009be04: *mut FILE;
    static mut DAT_1009bf5c: [i8; 0];
    static mut s_debug_log_1009be08: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut FILE;
    fn _fputs(_Str: *mut i8, _File: *mut FILE) -> i32;
    fn _fflush(_File: *mut FILE) -> i32;
    fn _fclose(_File: *mut FILE) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004f1e0(str: *mut i8) {
    mech_trace::enter!("1004f1e0", str);
    // SAFETY: the globals and the callees are the game's own; the file
    // handle is used and cleared where the original does.
    unsafe {
        if mech_trace::gr!("1004f1e0", "1009be04", DAT_1009be04).is_null() {
            mech_trace::gw!(
                "1004f1e0",
                "1009be04",
                DAT_1009be04,
                mech_trace::cr!(
                    "1004f1e0",
                    "FID_conflict___wfopen",
                    FID_conflict___wfopen(
                        (&raw mut s_debug_log_1009be08).cast::<i8>(),
                        (&raw mut DAT_1009bf5c).cast::<i8>(),
                    )
                )
            );
        }
        if !mech_trace::gr!("1004f1e0", "1009be04", DAT_1009be04).is_null() {
            mech_trace::cr!(
                "1004f1e0",
                "_fputs",
                _fputs(str, mech_trace::gr!("1004f1e0", "1009be04", DAT_1009be04))
            );
            mech_trace::cr!(
                "1004f1e0",
                "_fflush",
                _fflush(mech_trace::gr!("1004f1e0", "1009be04", DAT_1009be04))
            );
            mech_trace::cr!(
                "1004f1e0",
                "_fclose",
                _fclose(mech_trace::gr!("1004f1e0", "1009be04", DAT_1009be04))
            );
            mech_trace::gw!("1004f1e0", "1009be04", DAT_1009be04, core::ptr::null_mut());
        }
    }
}
