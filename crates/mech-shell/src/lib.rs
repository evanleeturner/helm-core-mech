// SPDX-License-Identifier: MIT
//! The shell library's functions, ported to Rust one module a function.
//!
//! Each port is taken from the decompile's Rust (RT1, `rust/mw2shell/src/`
//! at `02b7520`), which was judged against `MW2SHELL.DLL` under Wine, and
//! keeps the original's behavior: no fix. A port keeps its export's name,
//! `FUN_<address>`, until a name is approved for it.
//!
//! Call: the 20 `FUN_<address>` functions re-exported below; the game's
//! globals any of them reads or writes are the fields of [`ShellState`],
//! passed by reference.
#![expect(
    non_snake_case,
    reason = "each port keeps its export's name, FUN_<address>, until a name is approved"
)]

mod fun_10008680;
mod fun_1000a160;
mod fun_1000a6e0;
mod fun_10010920;
mod fun_10010930;
mod fun_10016b50;
mod fun_10016c90;
mod fun_1002f31a;
mod fun_10035d90;
mod fun_100406b0;
mod fun_1004b060;
mod fun_1004f240;
mod fun_10051650;
mod fun_10055340;
mod fun_10055ae0;
mod fun_10059870;
mod fun_10059e60;
mod fun_1005b5f0;
mod fun_100640f0;
mod fun_1006d130;
mod state;

pub use fun_1000a6e0::FUN_1000a6e0;
pub use fun_1000a160::FUN_1000a160;
pub use fun_1002f31a::FUN_1002f31a;
pub use fun_1004b060::FUN_1004b060;
pub use fun_1004f240::FUN_1004f240;
pub use fun_1005b5f0::FUN_1005b5f0;
pub use fun_1006d130::FUN_1006d130;
pub use fun_10016b50::FUN_10016b50;
pub use fun_10016c90::FUN_10016c90;
pub use fun_10035d90::FUN_10035d90;
pub use fun_10055ae0::FUN_10055ae0;
pub use fun_10059e60::FUN_10059e60;
pub use fun_100406b0::FUN_100406b0;
pub use fun_100640f0::FUN_100640f0;
pub use fun_10008680::FUN_10008680;
pub use fun_10010920::FUN_10010920;
pub use fun_10010930::FUN_10010930;
pub use fun_10051650::FUN_10051650;
pub use fun_10055340::FUN_10055340;
pub use fun_10059870::FUN_10059870;
pub use state::ShellState;
