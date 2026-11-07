// SPDX-License-Identifier: MIT
//! `mech-start <program> <library>`: the program made suspended, its entry parked, the library
//! loaded into it and the entry given back, so the library binds before the program's first
//! instruction. Then the program's own exit code, passed on. See [`windows`].
#![cfg_attr(all(target_arch = "x86", target_os = "windows"), no_std, no_main)]

#[cfg(all(target_arch = "x86", target_os = "windows"))]
mod windows;

/// Nothing to start on any other target; the library's reading is what the host tests.
#[cfg(not(all(target_arch = "x86", target_os = "windows")))]
fn main() {}
