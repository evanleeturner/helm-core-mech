// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100701d0(param_1: *mut core::ffi::c_void) -> i32;
    fn _fputc(_Ch: i32, _File: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011670(param_1: *mut core::ffi::c_void) {
    mech_trace::enter!("10011670", param_1);
    // SAFETY: the callees are the game's own; param_1 is the caller's FILE
    // handle, passed through unchanged.
    unsafe {
        let i_var1 = mech_trace::cr!("10011670", "100701d0", FUN_100701d0(param_1));
        let mut u_var2: u32 = (i_var1.wrapping_neg() as u32) & 3;
        if u_var2 != 0 {
            loop {
                u_var2 = u_var2.wrapping_sub(1);
                mech_trace::cr!("10011670", "_fputc", _fputc(0, param_1));
                if !(0 < (u_var2 as i32)) {
                    break;
                }
            }
        }
    }
}
