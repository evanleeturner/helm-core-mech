// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9424: *mut core::ffi::c_void;
    static mut DAT_100aafc4: *mut core::ffi::c_void;
    static mut DAT_100c823c: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1007dd4c();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007e602() {
    // SAFETY: the globals and the callee are the game's own memory; the
    // function's address is stored as the original does.
    unsafe {
        DAT_100b9424 = DAT_100aafc4;
        DAT_100aafc4 = FUN_1007dd4c as *const () as *mut core::ffi::c_void;
        DAT_100c823c = 0;
    }
}
