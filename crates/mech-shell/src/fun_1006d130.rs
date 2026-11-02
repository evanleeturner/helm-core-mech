// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1006d130, from `src/mw2shell/1006d130.c` (`FUN_1006d130`):
//! `__stdcall`, no arguments, returns the game's `unsigned int` at
//! `DAT_100a6798`, here [`ShellState::dat_100a6798`].

use crate::ShellState;

/// Returns the word at `DAT_100a6798`.
#[must_use]
pub fn FUN_1006d130(state: &ShellState) -> u32 {
    let value = mech_trace::gr!("1006d130", "100a6798", state.dat_100a6798);
    mech_trace::rt!("1006d130", value)
}

#[cfg(test)]
mod tests {
    use super::FUN_1006d130;
    use crate::ShellState;
    use crate::test_log::debug_lines;

    /// The original's one row (`harvest/1006d130.log:273`, both stack fills)
    /// reads `DAT_100a6798` in the game file as the checker maps it and returns
    /// 0, the word the file holds there. Every field is 0 here, so the row
    /// cannot tell this field from another.
    #[test]
    fn returns_the_word_the_game_file_holds() {
        let state = ShellState {
            dat_100a6798: 0,
            ..ShellState::default()
        };
        let lines = debug_lines(|| {
            assert_eq!(FUN_1006d130(&state), 0, "harvest/1006d130.log:273");
        });
        assert!(lines.iter().any(|line| line.contains("f=1006d130")));
    }
}
