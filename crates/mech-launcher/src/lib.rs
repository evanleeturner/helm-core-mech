// SPDX-License-Identifier: MIT
//! The launcher's functions (`MERCSWIN.EXE`), ported to Rust one module a function.
//!
//! Each port is copied unchanged from the decompile's Rust (RT2, RT3,
//! `rust/` at `14b3e31`): exported as `FUN_<address>` with the
//! original's calling convention, its globals and callees linked by name.
//! It keeps the original's behavior: no fix.
//!
//! The ports' calling conventions exist only on 32-bit x86, so the crate
//! is empty on any other target: build it with
//! `--target i686-pc-windows-gnu`.
#![cfg(target_arch = "x86")]

mod fun_004010d0;
mod fun_004010f0;
mod fun_00401110;
mod fun_00401160;
mod fun_00401200;
mod fun_004014b0;
mod fun_00401550;
mod fun_00401600;
