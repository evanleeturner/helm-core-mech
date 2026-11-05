// SPDX-License-Identifier: MIT
//! The drawing stand-in's step 1 (DB1, ruling 1422): in progress; the fronts,
//! the backs and this page's whole doc are the next session's (`NEXT.md`).
#![cfg_attr(not(test), no_std)]

#[cfg(any(test, feature = "desk"))]
extern crate alloc;

#[cfg(any(test, feature = "desk"))]
pub mod easel;
pub mod table;
