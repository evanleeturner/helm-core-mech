// SPDX-License-Identifier: MIT
//! The Kernel's time (step 3, rulings 1389 and 1398). Every name that
//! measures an interval reads one game clock, [`clock::Clock`], handed in
//! and moved only by its owner: the frame loop on a page, a back on a
//! host, a test. No front reads the OS.
//!
//! The crate names no game and nothing of any one target, so it builds
//! for `wasm32-unknown-unknown`; a game's numbers (its longest frame) live
//! in its rows crate.
#![cfg_attr(not(test), no_std)]

pub mod clock;
