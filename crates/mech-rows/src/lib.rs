// SPDX-License-Identifier: MIT
//! mercs's rows (ruling 1389): the game's facts the Kernel stand-in reads,
//! a module a step, constants only: [`memory`] for `kernel_memory`'s
//! helpers, [`files`] for `kernel_files`' seam, [`disc`] for
//! `logical_disc`, [`time`] for `kernel_time`'s, [`threads`] for
//! `kernel_threads`', [`process`] for `kernel_process`'s, [`text`] for
//! `kernel_text`'s, [`registry`] for `kernel_registry`'s, [`window`] for `window`'s. The harvested
//! logs (c35, c36, c37) and the tests that replay them live here: they
//! test the stand-in on this game. It builds for any target, the
//! browser's included.
#![cfg_attr(not(test), no_std)]

pub mod disc;
pub mod files;
pub mod memory;
pub mod process;
pub mod registry;
#[cfg(test)]
mod test_log;
pub mod text;
pub mod threads;
pub mod time;
pub mod window;
