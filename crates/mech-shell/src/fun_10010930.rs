// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10010930, from `src/mw2shell/10010930.c` (`GetGlobalVolumeScale_Likely`):
//! `__stdcall`, no arguments, returns the game's `unsigned int` at
//! `DAT_100a67f4`, here [`ShellState::dat_100a67f4`].

use log::debug;

use crate::ShellState;

/// Returns the word at `DAT_100a67f4`.
#[must_use]
pub fn FUN_10010930(state: &ShellState) -> u32 {
    let value = state.dat_100a67f4;
    debug!("FUN_10010930() -> {value:#x}");
    value
}

#[cfg(test)]
mod tests {
    use super::FUN_10010930;
    use crate::ShellState;
    use crate::test_log::debug_lines;

    /// The original's one row (`harvest/10010930.log:273`, both stack fills)
    /// reads `DAT_100a67f4` in the game file as the checker maps it and returns
    /// 65536, the word the file holds there. Only this field is set, so a port
    /// that read another would return 0.
    #[test]
    fn returns_the_word_the_game_file_holds() {
        let state = ShellState {
            dat_100a67f4: 65536,
            ..ShellState::default()
        };
        let lines = debug_lines(|| {
            assert_eq!(FUN_10010930(&state), 65536, "harvest/10010930.log:273");
        });
        assert!(lines.iter().any(|line| line.contains("FUN_10010930")));
    }
}
