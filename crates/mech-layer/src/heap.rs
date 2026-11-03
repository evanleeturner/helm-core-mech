// SPDX-License-Identifier: MIT
//! The C library's memory helpers on the Windows heap (HC7, ruling 1386):
//! `kernel_memory`'s rules over a file's own heap, the shell's six exported
//! `HC5_<address>` and swapped as the ports are.
//!
//! Each call reads the file's heap handle and `_newmode` at its row's
//! addresses, so a block the unswapped C made before the swap and one made
//! after it live in one heap and free alike. The new handler is called
//! through the C library's own `__callnewh`, its lock and all, never
//! rewritten. The thread's last error is kept around each line, as
//! `imp_call!` keeps it; the heap calls themselves set it as they did.
use core::ffi::c_void;
use core::ptr::with_exposed_provenance;
use kernel_memory::table::Row;
use kernel_memory::{Heap, Memory};
use mech_rows::memory::SHELL;

unsafe extern "system" {
    fn HeapAlloc(heap: *mut c_void, flags: u32, bytes: usize) -> *mut c_void;
    fn HeapReAlloc(heap: *mut c_void, flags: u32, block: *mut c_void, bytes: usize) -> *mut c_void;
    fn HeapFree(heap: *mut c_void, flags: u32, block: *mut c_void) -> i32;
}

/// `HEAP_ZERO_MEMORY`: calloc's flag.
const ZEROED: u32 = 8;

/// One file's Windows heap, found through its row.
pub struct Win32(&'static Row);

impl Win32 {
    /// The file's heap handle, read at its row's address each call.
    fn handle(&self) -> *mut c_void {
        // SAFETY: the row's address is the file's heap handle, written once
        // by its C library's start, before any screen runs.
        unsafe { with_exposed_provenance::<*mut c_void>(self.0.heap as usize).read() }
    }
}

impl Memory for Win32 {
    fn heap(&self) -> usize {
        self.handle().addr()
    }

    fn alloc(&self, bytes: u32, zeroed: bool) -> *mut u8 {
        // SAFETY: the handle is the file's own heap.
        unsafe {
            HeapAlloc(
                self.handle(),
                if zeroed { ZEROED } else { 0 },
                bytes as usize,
            )
            .cast()
        }
    }

    unsafe fn resize(&self, block: *mut u8, bytes: u32) -> *mut u8 {
        // SAFETY: the caller's promise: the block is this heap's.
        unsafe { HeapReAlloc(self.handle(), 0, block.cast(), bytes as usize).cast() }
    }

    unsafe fn release(&self, block: *mut u8) {
        // SAFETY: the caller's promise: the block is this heap's. The
        // original drops the answer too.
        unsafe { HeapFree(self.handle(), 0, block.cast()) };
    }

    fn new_mode(&self) -> bool {
        // SAFETY: the row's address is the file's `_newmode`, an int.
        unsafe { with_exposed_provenance::<i32>(self.0.new_mode as usize).read() != 0 }
    }

    fn new_handler(&self, bytes: u32) -> bool {
        // SAFETY: the row's address is the file's `__callnewh`, a cdecl
        // function of the size, answering nonzero to try again.
        let call: unsafe extern "C" fn(u32) -> i32 =
            unsafe { core::mem::transmute(self.0.call_new_handler as usize) };
        // SAFETY: as above.
        unsafe { call(bytes) != 0 }
    }

    fn quiet(&self, line: impl FnOnce()) {
        // SAFETY: reads the thread's own last error.
        let error = unsafe { crate::GetLastError() };
        line();
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(error) };
    }
}

/// The shell's helpers.
static SHELL_HEAP: Heap<Win32> = Heap::new(&SHELL, Win32(&SHELL));

/// The shell's malloc.
#[must_use]
#[unsafe(export_name = "HC5_1006fab0")]
pub extern "C" fn malloc(bytes: u32) -> *mut u8 {
    SHELL_HEAP.malloc(bytes)
}

/// The shell's operator new.
#[must_use]
#[unsafe(export_name = "HC5_1006f200")]
pub extern "C" fn operator_new(bytes: u32) -> *mut u8 {
    SHELL_HEAP.operator_new(bytes)
}

/// The shell's free.
///
/// # Safety
///
/// `block` is null, or the shell's heap's and not yet freed.
#[unsafe(export_name = "HC5_1006e790")]
pub unsafe extern "C" fn free(block: *mut u8) {
    // SAFETY: the caller's promise.
    unsafe { SHELL_HEAP.free(block) };
}

/// The shell's operator delete.
///
/// # Safety
///
/// As [`free`].
#[unsafe(export_name = "HC5_1006ec30")]
pub unsafe extern "C" fn operator_delete(block: *mut u8) {
    // SAFETY: the caller's promise.
    unsafe { SHELL_HEAP.operator_delete(block) };
}

/// The shell's calloc.
#[must_use]
#[unsafe(export_name = "HC5_1006e820")]
pub extern "C" fn calloc(count: u32, size: u32) -> *mut u8 {
    SHELL_HEAP.calloc(count, size)
}

/// The shell's realloc.
///
/// # Safety
///
/// As [`free`].
#[unsafe(export_name = "HC5_10070080")]
pub unsafe extern "C" fn realloc(block: *mut u8, bytes: u32) -> *mut u8 {
    // SAFETY: the caller's promise.
    unsafe { SHELL_HEAP.realloc(block, bytes) }
}
