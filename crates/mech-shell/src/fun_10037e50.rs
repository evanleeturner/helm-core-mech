// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100943fc: [i8; 0];
    static mut DAT_10094410: [i8; 0];
    static mut DAT_10094400: [i8; 0];
    static mut DAT_10094420: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(param_1: *mut i8) -> *mut i8;
    fn _strstr(_Str: *mut i8, _SubStr: *mut i8) -> *mut i8;
    fn FUN_1006f780(param_1: *const core::ffi::c_void) -> u8;
    fn _atoi(_Str: *mut i8) -> i32;
    fn _sprintf(_Dest: *mut i8, _Format: *mut i8, ...) -> i32;
    fn FUN_10070c20(param_1: *const core::ffi::c_void, param_2: *const core::ffi::c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037e50(param_1: *mut i8) {
    mech_trace::enter!("10037e50", param_1);
    let mut local_40: [i8; 32] = [0; 32];
    let mut local_20: [i8; 32] = [0; 32];
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's string, and the local buffers are large enough for the
    // single digit and the formatted text.
    unsafe {
        let sub_str = &raw mut DAT_100943fc as *mut i8;
        let mut pc_var1 = mech_trace::cr!("10037e50", "1005bbb0", FUN_1005bbb0(param_1));
        pc_var1 = mech_trace::cr!("10037e50", "_strstr", _strstr(pc_var1, sub_str));
        if !pc_var1.is_null() {
            mech_trace::cr!(
                "10037e50",
                "1006f780",
                FUN_1006f780((&raw mut DAT_10094400) as *const core::ffi::c_void)
            );
            local_40[1] = 0;
            local_40[0] = mech_trace::rd!("10037e50", param_1.byte_add(3), read);
            let i_var2 = mech_trace::cr!("10037e50", "_atoi", _atoi(local_40.as_mut_ptr()));
            mech_trace::cr!(
                "10037e50",
                "_sprintf",
                _sprintf(
                    local_20.as_mut_ptr(),
                    &raw mut DAT_10094410 as *mut i8,
                    i_var2,
                )
            );
            mech_trace::cr!(
                "10037e50",
                "10070c20",
                FUN_10070c20(
                    local_20.as_ptr() as *const core::ffi::c_void,
                    (&raw mut DAT_10094420) as *const core::ffi::c_void,
                )
            );
        }
    }
}
