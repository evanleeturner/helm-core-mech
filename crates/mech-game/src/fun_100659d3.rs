// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38ec: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10065891(param_1: *mut u8) -> u32;
    fn FUN_1008d9b0(param_1: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10065a34();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100659d3() -> u32 {
    let mut local_18: [u8; 20] = [0; 20];
    local_18[0] = 0x3b;
    // SAFETY: the callees and the global are the game's own; local_18 is a
    // live buffer for the duration of the call.
    unsafe {
        let i_var1 = FUN_10065891(local_18.as_mut_ptr());
        if i_var1 == 0 {
            FUN_10065a34();
            if !DAT_100b38ec.is_null() {
                FUN_1008d9b0(DAT_100b38ec);
                DAT_100b38ec = core::ptr::null_mut();
            }
            0
        } else {
            2
        }
    }
}
