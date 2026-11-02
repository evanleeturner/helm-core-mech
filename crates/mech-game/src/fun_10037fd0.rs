// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1021a5cc: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037fd0() {
    // SAFETY: the global holds the game's own function pointer (an import
    // slot for joyGetNumDevs); it is called with no arguments as the original does.
    unsafe {
        let f: unsafe extern "stdcall" fn() = core::mem::transmute(DAT_1021a5cc);
        f();
    }
}
