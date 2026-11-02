// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7f9c: *mut c_void;
}

unsafe extern "C" {
    fn _fclose(param_1: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065a34() {
    mech_trace::enter!("10065a34");
    // SAFETY: the callee and the global are the game's own; the stored
    // stream pointer is passed to fclose as the original does.
    unsafe {
        mech_trace::cr!(
            "10065a34",
            "_fclose",
            _fclose(mech_trace::gr!("10065a34", "100c7f9c", DAT_100c7f9c))
        );
    }
}
