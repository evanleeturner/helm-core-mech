// SPDX-License-Identifier: MIT
//! `HeapAlloc` as `fun_10086951.rs` declares it, `extern "C"`, over
//! `heap.rs`'s state, which the test takes as `mod heap`. The port's
//! declaration is taken as it stands; the other ports declare `stdcall`.

use core::ffi::c_void;

#[unsafe(no_mangle)]
extern "C" fn HeapAlloc(heap: *mut c_void, flags: u32, size: u32) -> *mut c_void {
    crate::heap::alloc(heap, flags, size)
}
