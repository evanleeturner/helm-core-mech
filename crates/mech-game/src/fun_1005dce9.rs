// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3448: *mut core::ffi::c_void;
    static mut DAT_100b348c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _time(param_1: *mut i32) -> i32;
    fn _ctime(param_1: *const i32) -> *mut i8;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_1005dd5a(param_1: *mut i8);
    fn _fclose(param_1: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005dce9() -> bool {
    let mut local_58: [i8; 80] = [0; 80];
    let mut local_8: i32;
    // SAFETY: the globals and callees are the game's own; the buffer and
    // the time value are local and live across the calls.
    unsafe {
        let b_var2 = !DAT_100b3448.is_null();
        if b_var2 {
            let t_var3 = _time(core::ptr::null_mut());
            local_8 = t_var3;
            let pc_var1 = _ctime(&raw const local_8);
            _sprintf(
                local_58.as_mut_ptr(),
                (&raw const DAT_100b348c).cast::<i8>(),
                pc_var1,
            );
            FUN_1005dd5a(local_58.as_mut_ptr());
            _fclose(DAT_100b3448);
        }
        b_var2
    }
}
