// SPDX-License-Identifier: MIT
//! `FUN_1001c299`'s stand-in, as `fun_1001c303.rs` declares it: records the
//! owner and block it is handed, and does nothing else.

use crate::kit::Calls;

/// Each call's (owner, block).
pub static CALLS: Calls<(i32, i32)> = Calls::new();

#[unsafe(no_mangle)]
extern "C" fn FUN_1001c299(owner: i32, block: i32) {
    CALLS.record((owner, block));
}
