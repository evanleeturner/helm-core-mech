// SPDX-License-Identifier: MIT
//! mercs's rows (ruling 1389): the game's facts the Kernel stand-in reads,
//! a module a step, constants only: [`memory`] for `kernel_memory`'s
//! helpers, [`files`] for `kernel_files`' seam. The harvested logs (c35,
//! c36, c37) and the tests that replay them live here: they test the
//! stand-in on this game. It builds for any target, the browser's included.
#![cfg_attr(not(test), no_std)]

pub mod files;
pub mod memory;
#[cfg(test)]
mod test_log;
