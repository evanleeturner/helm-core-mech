// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003eb7c(mem: *mut core::ffi::c_void) {
    // SAFETY: the heap handle global and HeapFree are the game's own; mem is
    // passed through to the allocator unchanged.
    unsafe {
        HeapFree(DAT_100adf74, 1, mem);
    }
}
