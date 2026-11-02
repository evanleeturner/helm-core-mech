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
    mech_trace::enter!("1000e790", param_1);
    // SAFETY: the callees and the global are the game's own; the path
    // returned by the builder is passed straight to the Win32 call.
    unsafe {
        let file_path = mech_trace::cr!("1000e790", "1000e6d0", FUN_1000e6d0(param_1));
        let file_attributes = mech_trace::cr!(
            "1000e790",
            "GetFileAttributesA",
            GetFileAttributesA(file_path as *const core::ffi::c_void)
        );
        if file_attributes != 0xffff_ffff {
            return mech_trace::rt!("1000e790", 1);
        }
        mech_trace::gw!("1000e790", "1008e3e0", DAT_1008e3e0, 1);
        let file_path = mech_trace::cr!("1000e790", "1000e6d0", FUN_1000e6d0(param_1));
        let file_attributes = mech_trace::cr!(
            "1000e790",
            "GetFileAttributesA",
            GetFileAttributesA(file_path as *const core::ffi::c_void)
        );
        if file_attributes == 0xffff_ffff {
            return mech_trace::rt!("1000e790", 0);
        }
        mech_trace::gw!("1000e790", "1008e3e0", DAT_1008e3e0, 1);
        mech_trace::rt!("1000e790", 1)
    }
}
