// SPDX-License-Identifier: MIT
//! The heap stand-ins' state, taken as `#[path = "kit/heap.rs"] mod heap;`
//! beside one of its two fronts: `heap_win.rs`, the `stdcall` `HeapAlloc`
//! and `HeapFree` most ports declare, or `heap_c.rs`, the C `HeapAlloc` one
//! port declares. It holds the two globals the heap ports read, a static
//! arena the blocks come from, and a ledger of each call. Every answer is
//! the test's own: a handle that is a token, never dereferenced; a block
//! filled with `MARK` unless the zeroing flag asks for zeros; null after
//! `fail_next`. A heap call never allocates, so a stand-in's call never
//! reaches the process's own allocator; `heap_win.rs` sends that allocator's
//! calls, on any heap but `HANDLE`, to kernel32 and keeps them out of the
//! ledger.
#![allow(dead_code, reason = "each test takes the stand-ins its port needs")]

use core::ffi::c_void;
use core::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

/// The handle the stand-ins answer as the game's heap: a token.
pub const HANDLE: usize = 0x4ea9_0000;
/// The byte a block not zeroed is filled with, so a port's clear shows.
pub const MARK: u8 = 0xa5;
/// `HEAP_ZERO_MEMORY`.
const ZEROED: u32 = 8;

/// `DAT_100adf74`, the game's heap handle.
#[unsafe(no_mangle)]
pub static mut DAT_100adf74: *mut c_void = core::ptr::without_provenance_mut(HANDLE);
/// `DAT_100ab0c0`, the word `1001c5d1` writes into its block's first field;
/// the test sets it.
#[unsafe(no_mangle)]
pub static mut DAT_100ab0c0: u32 = 0;

const ARENA_BYTES: usize = 0x1_0000;

#[repr(align(16))]
struct Arena([u8; ARENA_BYTES]);

static mut ARENA: Arena = Arena([0; ARENA_BYTES]);
/// The arena's bytes handed out so far; a block is never handed twice.
static USED: AtomicUsize = AtomicUsize::new(0);
static FAIL: AtomicBool = AtomicBool::new(false);

/// One heap call: the heap, the flags, the size or block handed, the answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Call {
    pub heap: usize,
    pub flags: u32,
    pub what: usize,
    pub answer: usize,
}

const NONE: Call = Call {
    heap: 0,
    flags: 0,
    what: 0,
    answer: 0,
};
const KEPT: usize = 64;

/// The calls since the last take, at most `KEPT`, in a fixed array.
pub struct Ledger(Mutex<([Call; KEPT], usize)>);

impl Ledger {
    const fn new() -> Self {
        Self(Mutex::new(([NONE; KEPT], 0)))
    }

    fn record(&self, call: Call) {
        let mut kept = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let at = kept.1;
        assert!(at < KEPT, "more than {KEPT} heap calls between two takes");
        kept.0[at] = call;
        kept.1 = at + 1;
    }

    /// The calls since the last take, oldest first; clears the ledger.
    pub fn take(&self) -> Vec<Call> {
        let mut kept = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let calls = kept.0[..kept.1].to_vec();
        kept.1 = 0;
        calls
    }
}

pub static ALLOCS: Ledger = Ledger::new();
pub static FREES: Ledger = Ledger::new();

/// Makes the next `HeapAlloc` answer null, as a heap out of room does.
pub fn fail_next() {
    FAIL.store(true, Ordering::Relaxed);
}

/// A fresh block of `size` bytes from the arena, zeroed or filled with
/// `MARK`; null when the arena is spent.
fn carve(size: usize, zeroed: bool) -> *mut u8 {
    let rounded = size.next_multiple_of(16);
    let at = USED.fetch_add(rounded, Ordering::Relaxed);
    if at + rounded > ARENA_BYTES {
        return null_mut();
    }
    // SAFETY: [at, at + rounded) lies inside the arena, and no block handed
    // out before it overlaps it.
    unsafe {
        let block = (&raw mut ARENA).cast::<u8>().add(at);
        block.write_bytes(if zeroed { 0 } else { MARK }, rounded);
        block
    }
}

/// A zeroed block for the test's own use (an owner object, a list node);
/// no heap call, so never in the ledger.
pub fn block(size: usize) -> *mut u8 {
    carve(size, true)
}

/// `HeapAlloc`'s stand-in body: records the call, answers a block or null.
pub fn alloc(heap: *mut c_void, flags: u32, size: u32) -> *mut c_void {
    let answer = if FAIL.swap(false, Ordering::Relaxed) {
        null_mut()
    } else {
        carve(size as usize, flags & ZEROED != 0)
    };
    ALLOCS.record(Call {
        heap: heap.addr(),
        flags,
        what: size as usize,
        answer: answer.addr(),
    });
    answer.cast()
}

/// `HeapFree`'s stand-in body: records the call and answers success; the
/// block stays in the arena, untouched.
pub fn free(heap: *mut c_void, flags: u32, block: *mut c_void) -> i32 {
    FREES.record(Call {
        heap: heap.addr(),
        flags,
        what: block.addr(),
        answer: 1,
    });
    1
}
