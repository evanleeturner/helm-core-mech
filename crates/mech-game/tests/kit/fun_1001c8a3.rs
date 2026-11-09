// SPDX-License-Identifier: MIT
//! `FUN_1001c8a3`'s stand-in, as `fun_1001c989.rs` declares it (returning a
//! `u8`; the port returns nothing): records each block it is handed and
//! answers 0, the test's own.

use crate::kit::Calls;

/// Each call's block, as an address.
pub static CALLS: Calls<usize> = Calls::new();

#[unsafe(no_mangle)]
extern "C" fn FUN_1001c8a3(block: *mut core::ffi::c_void) -> u8 {
    CALLS.record(block.addr());
    0
}
