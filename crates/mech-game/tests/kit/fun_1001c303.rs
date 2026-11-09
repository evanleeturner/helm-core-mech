// SPDX-License-Identifier: MIT
//! `FUN_1001c303`'s stand-in, as `fun_1001c5d1.rs` declares it: records its
//! six arguments, the sixth as an address, and answers the address `answer`
//! last set, the test's own (null until set).

use crate::kit::Calls;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Each call's arguments, the sixth as an address.
pub static CALLS: Calls<(i32, i32, i32, i32, i32, usize)> = Calls::new();
static ANSWER: AtomicUsize = AtomicUsize::new(0);

/// Sets the address each later call answers; 0 answers null.
pub fn answer(block: usize) {
    ANSWER.store(block, Ordering::Relaxed);
}

#[unsafe(no_mangle)]
extern "C" fn FUN_1001c303(
    owner: i32,
    key: i32,
    first: i32,
    second: i32,
    third: i32,
    out: *mut u32,
) -> *mut i32 {
    CALLS.record((owner, key, first, second, third, out.addr()));
    core::ptr::with_exposed_provenance_mut(ANSWER.load(Ordering::Relaxed))
}
