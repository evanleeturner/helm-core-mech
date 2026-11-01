// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1000a160, from `src/mw2shell/1000a160.c`
//! (`RoundToNearest100_Likely`): `__cdecl`, one `int` argument; calls
//! the game's `FUN_1000a140` (the C's `TruncateTo100`) on it plus 0x32,
//! wrapping as the 1996 code does. The C declares the callee `void` and
//! returns nothing, so the port keeps no result. The callee is not ported
//! here: the caller passes it in.

/// Calls `fun_1000a140`, standing for the game's `FUN_1000a140`, on
/// `value + 0x32`, wrapping.
pub fn FUN_1000a160(value: i32, fun_1000a140: impl FnOnce(i32)) {
    fun_1000a140(value.wrapping_add(0x32));
}
