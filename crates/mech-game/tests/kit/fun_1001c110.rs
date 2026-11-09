// SPDX-License-Identifier: MIT
//! `FUN_1001c110`'s stand-in, as `fun_1001c989.rs` declares it: records each
//! object it is handed and answers 0, the test's own.

use crate::kit::Calls;

/// Each call's object.
pub static CALLS: Calls<i32> = Calls::new();

#[unsafe(no_mangle)]
extern "C" fn FUN_1001c110(object: i32) -> u8 {
    CALLS.record(object);
    0
}
