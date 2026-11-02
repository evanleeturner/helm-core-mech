// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(hHeap: *mut core::ffi::c_void, dwFlags: u32, lpMem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c8a3(mem: *mut core::ffi::c_void) {
    if !mem.is_null() {
        // SAFETY: the heap handle is the game's own global; mem is a block
        // the caller allocated from that heap.
        unsafe {
            HeapFree(DAT_100adf74, 1, mem);
        }
    }
}
