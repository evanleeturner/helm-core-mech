// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e3e0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000e6d0(param_1: u32) -> *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetFileAttributesA(param_1: *const core::ffi::c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e790(param_1: u32) -> u32 {
    // SAFETY: the callees and the global are the game's own; the path
    // returned by the builder is passed straight to the Win32 call.
    unsafe {
        let file_path = FUN_1000e6d0(param_1);
        let file_attributes = GetFileAttributesA(file_path as *const core::ffi::c_void);
        if file_attributes != 0xffff_ffff {
            return 1;
        }
        DAT_1008e3e0 = 1;
        let file_path = FUN_1000e6d0(param_1);
        let file_attributes = GetFileAttributesA(file_path as *const core::ffi::c_void);
        if file_attributes == 0xffff_ffff {
            return 0;
        }
        DAT_1008e3e0 = 1;
        1
    }
}
