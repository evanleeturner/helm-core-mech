// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100b3c9c: u32;
    static mut DAT_100b3cb8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1006ac3b();
    fn FUN_1004652f();
    fn FUN_1003c67c();
    fn FUN_1000278e();
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10047fde(param_1: *mut i8, param_2: *mut core::ffi::c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ae47() {
    // SAFETY: all callees and globals are the game's own; the string global
    // is the config name buffer and the heap handle/pointer are read by value.
    unsafe {
        FUN_1006ac3b();
        FUN_1004652f();
        FUN_1003c67c();
        FUN_1000278e();
        FUN_10047fde(
            (&raw mut DAT_100b3cb8).cast::<i8>(),
            DAT_100b3c9c as *mut core::ffi::c_void,
        );
        HeapFree(DAT_100adf74, 1, DAT_100b3c9c as *mut core::ffi::c_void);
    }
}
