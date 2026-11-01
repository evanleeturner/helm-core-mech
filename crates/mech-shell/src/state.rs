// SPDX-License-Identifier: MIT
//! The game's globals the ports touch, held as one owned value.

/// The `MW2SHELL.DLL` globals the ports read or write, one field a global,
/// named `dat_<address>` for its address in the game file and typed as the
/// decompile declares it.
///
/// `Default` sets every field to 0, which is not the value the game file
/// loads with: a caller that needs the game's values sets them.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ShellState {
    /// Written by [`FUN_10016b50`](crate::FUN_10016b50).
    pub dat_1008edf4: u32,
    /// Written by [`FUN_10016c90`](crate::FUN_10016c90).
    pub dat_1008edfc: u32,
    /// Written by [`FUN_1002f31a`](crate::FUN_1002f31a), its first argument.
    pub dat_10090410: u32,
    /// Written by [`FUN_1002f31a`](crate::FUN_1002f31a), its second argument.
    pub dat_10090414: u32,
    /// Written by [`FUN_1004f240`](crate::FUN_1004f240) when its argument is 0 to 4.
    pub dat_1009bdf8: i32,
    /// Set to 0 by [`FUN_10059870`](crate::FUN_10059870).
    pub dat_100a4b10: i32,
    /// Set to 0 by [`FUN_10059870`](crate::FUN_10059870).
    pub dat_100a4b14: i32,
    /// Written by [`FUN_10059e60`](crate::FUN_10059e60), its first argument.
    pub dat_100a4bb8: i32,
    /// Written by [`FUN_10059e60`](crate::FUN_10059e60), its second argument.
    pub dat_100a4bbc: i32,
    /// Read by [`FUN_1006d130`](crate::FUN_1006d130).
    pub dat_100a6798: u32,
    /// Read by [`FUN_10010930`](crate::FUN_10010930).
    pub dat_100a67f4: u32,
    /// Read by [`FUN_10010920`](crate::FUN_10010920).
    pub dat_100a67fc: u32,
}
