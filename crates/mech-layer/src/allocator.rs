// SPDX-License-Identifier: MIT
//! The swap's allocator, Windows' own heap: `draw`'s easel holds buffers and
//! tables, and the swap had none. It holds no state, so it works before any
//! first row, on any thread, and a call it makes changes no error a file reads
//! on success. An alignment over 8, `HeapAlloc`'s edge on this target, answers
//! null: nothing here asks for more, and a null is a named failure, where a
//! misaligned answer would be a silent one.
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr::null_mut;

unsafe extern "system" {
    fn GetProcessHeap() -> *mut c_void;
    fn HeapAlloc(heap: *mut c_void, flags: u32, bytes: usize) -> *mut u8;
    fn HeapFree(heap: *mut c_void, flags: u32, at: *mut u8) -> i32;
}

/// `HeapAlloc`'s edge on this target.
const EDGE: usize = 8;
/// `HEAP_ZERO_MEMORY`.
const ZEROED: u32 = 8;

struct Heap;

// SAFETY: `HeapAlloc` answers fresh memory on an 8-byte edge or null; any larger
// alignment asked answers null before the call.
unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() > EDGE {
            return null_mut();
        }
        // SAFETY: the process's own heap, always there.
        unsafe { HeapAlloc(GetProcessHeap(), 0, layout.size()) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.align() > EDGE {
            return null_mut();
        }
        // SAFETY: the process's own heap, always there.
        unsafe { HeapAlloc(GetProcessHeap(), ZEROED, layout.size()) }
    }

    unsafe fn dealloc(&self, at: *mut u8, _layout: Layout) {
        // SAFETY: `at` came from `alloc` on this heap, by the trait's contract.
        unsafe { HeapFree(GetProcessHeap(), 0, at) };
    }
}

#[global_allocator]
static HEAP: Heap = Heap;
