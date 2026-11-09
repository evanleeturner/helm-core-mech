// SPDX-License-Identifier: MIT
//! `HeapAlloc` and `HeapFree` as most ports declare them, `stdcall`, over
//! `heap.rs`'s state, which the test takes as `mod heap`. Rust's own
//! allocator links the same two `stdcall` symbols, so these stand-ins catch
//! its calls too: measured, the first run under Wine died on "memory
//! allocation of 16 bytes failed" once the arena was spent. A call on the
//! game's heap, `heap::HANDLE`, is the port's and goes to the stand-in; any
//! other heap is the process's own and goes to kernel32's real function,
//! found in its export table at run time and never in the ledger.

use core::ffi::{CStr, c_char, c_void};
use core::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

type Alloc = unsafe extern "system" fn(*mut c_void, u32, usize) -> *mut c_void;
type Free = unsafe extern "system" fn(*mut c_void, u32, *mut c_void) -> i32;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleA(name: *const c_char) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
}

static REAL_ALLOC: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static REAL_FREE: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

/// kernel32's own export `name`, looked up once into `cache`. A miss
/// aborts: a panic here would allocate, and the allocation would come back.
fn kernel32(cache: &AtomicPtr<c_void>, name: &CStr) -> *mut c_void {
    let mut found = cache.load(Ordering::Relaxed);
    if found.is_null() {
        // SAFETY: both names end in NUL; kernel32 is loaded in every Win32
        // process, so its handle is live.
        found =
            unsafe { GetProcAddress(GetModuleHandleA(c"kernel32.dll".as_ptr()), name.as_ptr()) };
        if found.is_null() {
            std::process::abort();
        }
        cache.store(found, Ordering::Relaxed);
    }
    found
}

#[unsafe(no_mangle)]
extern "stdcall" fn HeapAlloc(heap: *mut c_void, flags: u32, size: u32) -> *mut c_void {
    if heap.addr() == crate::heap::HANDLE {
        return crate::heap::alloc(heap, flags, size);
    }
    // SAFETY: kernel32's HeapAlloc has `Alloc`'s signature; the arguments
    // are the process allocator's own, passed on unchanged.
    unsafe {
        let real = core::mem::transmute::<*mut c_void, Alloc>(kernel32(&REAL_ALLOC, c"HeapAlloc"));
        real(heap, flags, size as usize)
    }
}

#[unsafe(no_mangle)]
extern "stdcall" fn HeapFree(heap: *mut c_void, flags: u32, block: *mut c_void) -> i32 {
    if heap.addr() == crate::heap::HANDLE {
        return crate::heap::free(heap, flags, block);
    }
    // SAFETY: kernel32's HeapFree has `Free`'s signature; the block is one
    // kernel32's HeapAlloc handed the process allocator.
    unsafe {
        let real = core::mem::transmute::<*mut c_void, Free>(kernel32(&REAL_FREE, c"HeapFree"));
        real(heap, flags, block)
    }
}
