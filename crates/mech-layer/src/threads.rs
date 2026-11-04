// SPDX-License-Identifier: MIT
//! The shell's and the game file's thread calls through the seam (KH1,
//! ruling 1405): `kernel_threads`' front over Windows' own threads. The
//! shell's 9 are exported `HC6_<name>` as the layer's other rows are, so the
//! swap's loader points the shell's own slots at them unchanged. The game
//! file's 11 are exported `GAME_<name>`, a name the loader's `HC6_` match
//! leaves alone (2 of them the shell does not import), for the game file's
//! own table to point its slots at.
//!
//! Each export is a naked entry: before any code of its own it reads the
//! address the call returns to off the stack, then calls the front with it
//! first, so each line's `from` is the caller inside the file (wall 2). The
//! swap is built with no frame pointer, so nothing later in the call could
//! read it.
//!
//! The Windows threads answer from the real imports, through the swap DLL's
//! own import table, so no call loops back through a file's slot. A counter
//! is the caller's own 32 bits, added by one locked instruction as
//! `InterlockedIncrement` adds. The thread's last error is kept around each
//! line, as `imp_call!` keeps it.
#![allow(
    clippy::cast_possible_truncation,
    reason = "32-bit x86 only: an address is the 32 bits the front names a lock and a value by"
)]
use core::ffi::c_void;
use core::ptr::with_exposed_provenance_mut;
use core::sync::atomic::AtomicI32;
use kernel_threads::table::NO_SLOT;
use kernel_threads::{ThreadCalls, Threads};
use mech_rows::threads::{GAME, SHELL};

unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
    fn InitializeCriticalSection(section: *mut c_void);
    fn EnterCriticalSection(section: *mut c_void);
    fn LeaveCriticalSection(section: *mut c_void);
    fn DeleteCriticalSection(section: *mut c_void);
    fn TlsAlloc() -> u32;
    fn TlsFree(slot: u32) -> i32;
    fn TlsGetValue(slot: u32) -> *mut c_void;
    fn TlsSetValue(slot: u32, value: *mut c_void) -> i32;
}

/// Windows' own threads.
pub struct Win32;

impl Threads for Win32 {
    fn id(&self) -> u32 {
        // SAFETY: reads the calling thread's number.
        unsafe { GetCurrentThreadId() }
    }

    fn initialize(&self, lock: u32) {
        // SAFETY: the caller's own 24-byte record at `lock`.
        unsafe { InitializeCriticalSection(with_exposed_provenance_mut(lock as usize)) };
    }

    fn enter(&self, lock: u32) {
        // SAFETY: as `initialize`, a record the caller made.
        unsafe { EnterCriticalSection(with_exposed_provenance_mut(lock as usize)) };
    }

    fn leave(&self, lock: u32) {
        // SAFETY: as `enter`.
        unsafe { LeaveCriticalSection(with_exposed_provenance_mut(lock as usize)) };
    }

    fn delete(&self, lock: u32) {
        // SAFETY: as `enter`.
        unsafe { DeleteCriticalSection(with_exposed_provenance_mut(lock as usize)) };
    }

    fn slot_alloc(&self) -> Option<u32> {
        // SAFETY: takes a slot; touches no memory of the caller's.
        let slot = unsafe { TlsAlloc() };
        (slot != NO_SLOT).then_some(slot)
    }

    fn slot_free(&self, slot: u32) -> bool {
        // SAFETY: frees a slot by its index; touches no memory.
        unsafe { TlsFree(slot) != 0 }
    }

    /// Windows clears the last error on success and sets it for a slot not
    /// taken, so the error read back decides.
    fn slot_get(&self, slot: u32) -> Option<u32> {
        // SAFETY: reads the calling thread's value; Windows sets the error.
        let value = unsafe { TlsGetValue(slot) }.expose_provenance() as u32;
        // SAFETY: reads the thread's own last error.
        (unsafe { crate::GetLastError() } == 0).then_some(value)
    }

    fn slot_set(&self, slot: u32, value: u32) -> bool {
        // SAFETY: stores 32 bits in the calling thread's slot; Windows reads
        // nothing through them.
        unsafe { TlsSetValue(slot, with_exposed_provenance_mut(value as usize)) != 0 }
    }

    fn set_last_error(&self, code: u32) {
        // SAFETY: sets the thread's own last error.
        unsafe { crate::SetLastError(code) };
    }

    fn quiet(&self, line: impl FnOnce()) {
        // SAFETY: reads the thread's own last error.
        let error = unsafe { crate::GetLastError() };
        line();
        // SAFETY: sets back the value read above.
        unsafe { crate::SetLastError(error) };
    }
}

/// The shell's thread calls.
static SHELL_THREADS: ThreadCalls<Win32> = ThreadCalls::new(&SHELL, Win32);
/// The game file's thread calls.
static GAME_THREADS: ThreadCalls<Win32> = ThreadCalls::new(&GAME, Win32);

/// One file's 9 lock, slot and number calls, `extern "system"` with the
/// caller first, as a naked entry calls them.
macro_rules! fronts {
    ($file:ident, $calls:ident) => {
        mod $file {
            use super::$calls;

            pub(super) extern "system" fn current_thread_id(from: u32) -> u32 {
                $calls.current_thread_id(from)
            }

            pub(super) extern "system" fn initialize(from: u32, lock: u32) {
                $calls.initialize(from, lock);
            }

            pub(super) extern "system" fn enter(from: u32, lock: u32) {
                $calls.enter(from, lock);
            }

            pub(super) extern "system" fn leave(from: u32, lock: u32) {
                $calls.leave(from, lock);
            }

            pub(super) extern "system" fn delete(from: u32, lock: u32) {
                $calls.delete(from, lock);
            }

            pub(super) extern "system" fn slot_alloc(from: u32) -> u32 {
                $calls.slot_alloc(from)
            }

            pub(super) extern "system" fn slot_free(from: u32, slot: u32) -> u32 {
                $calls.slot_free(from, slot)
            }

            pub(super) extern "system" fn slot_get(from: u32, slot: u32) -> u32 {
                $calls.slot_get(from, slot)
            }

            pub(super) extern "system" fn slot_set(from: u32, slot: u32, value: u32) -> u32 {
                $calls.slot_set(from, slot, value)
            }
        }
    };
}

fronts!(shell, SHELL_THREADS);
fronts!(game, GAME_THREADS);

/// `InterlockedIncrement` for the game file.
unsafe extern "system" fn game_increment(from: u32, counter: *const AtomicI32) -> i32 {
    // SAFETY: the caller's own 32-bit counter, 4-aligned as Windows asks.
    GAME_THREADS.increment(from, unsafe { &*counter })
}

/// `InterlockedDecrement` for the game file.
unsafe extern "system" fn game_decrement(from: u32, counter: *const AtomicI32) -> i32 {
    // SAFETY: as `game_increment`.
    GAME_THREADS.decrement(from, unsafe { &*counter })
}

entry!(HC6_GetCurrentThreadId, shell::current_thread_id, 0);
entry!(HC6_InitializeCriticalSection, shell::initialize, 1);
entry!(HC6_EnterCriticalSection, shell::enter, 1);
entry!(HC6_LeaveCriticalSection, shell::leave, 1);
entry!(HC6_DeleteCriticalSection, shell::delete, 1);
entry!(HC6_TlsAlloc, shell::slot_alloc, 0);
entry!(HC6_TlsFree, shell::slot_free, 1);
entry!(HC6_TlsGetValue, shell::slot_get, 1);
entry!(HC6_TlsSetValue, shell::slot_set, 2);

entry!(GAME_GetCurrentThreadId, game::current_thread_id, 0);
entry!(GAME_InitializeCriticalSection, game::initialize, 1);
entry!(GAME_EnterCriticalSection, game::enter, 1);
entry!(GAME_LeaveCriticalSection, game::leave, 1);
entry!(GAME_DeleteCriticalSection, game::delete, 1);
entry!(GAME_TlsAlloc, game::slot_alloc, 0);
entry!(GAME_TlsFree, game::slot_free, 1);
entry!(GAME_TlsGetValue, game::slot_get, 1);
entry!(GAME_TlsSetValue, game::slot_set, 2);
entry!(GAME_InterlockedIncrement, game_increment, 1);
entry!(GAME_InterlockedDecrement, game_decrement, 1);
