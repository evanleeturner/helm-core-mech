// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3448: *mut u8;
    static mut DAT_100b344c: [i8; 0];
    static mut s_mw2_log_100b3450: [i8; 0];
    static mut s__Date__1995_06_25_15_47_38___100b3458: [i8; 0];
    static mut s_MW2_EXE__s__s_100b3478: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *const i8, mode: *const i8) -> *mut u8;
    fn _time(timer: *mut i32) -> i32;
    fn _ctime(timer: *const i32) -> *mut i8;
    fn _sprintf(buffer: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005dd5a(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005dc60() -> u8 {
    let mut local_58: i32;
    let mut local_54: [i8; 80] = [0; 80];
    // SAFETY: the CRT functions and globals are the game's own; the local
    // buffer is large enough for the formatted line as in the original.
    unsafe {
        let p_fvar1 = _fopen(
            (&raw mut s_mw2_log_100b3450).cast::<i8>(),
            (&raw mut DAT_100b344c).cast::<i8>(),
        );
        if !p_fvar1.is_null() {
            DAT_100b3448 = p_fvar1;
            local_58 = _time(core::ptr::null_mut());
            let pc_var2 = _ctime(&raw const local_58);
            _sprintf(
                local_54.as_mut_ptr(),
                (&raw mut s_MW2_EXE__s__s_100b3478).cast::<i8>(),
                (&raw mut s__Date__1995_06_25_15_47_38___100b3458).cast::<i8>(),
                pc_var2,
            );
            FUN_1005dd5a(local_54.as_mut_ptr());
        }
        (!p_fvar1.is_null()) as u8
    }
}
