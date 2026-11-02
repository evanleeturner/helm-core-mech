// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1004f240, from `src/mw2shell/1004f240.c` (`FUN_1004f240`):
//! `__cdecl`, one `int` argument; stores it at `DAT_1009bdf8`, here
//! [`ShellState::dat_1009bdf8`], and returns 0 when it lies in 0 to 4, else
//! returns 2 and stores nothing.

use crate::ShellState;

/// Stores `param_1` at `DAT_1009bdf8` and returns 0 for 0 to 4; returns 2
/// otherwise. The C's test, `-1 < param_1 && param_1 < 5`, is the range.
pub fn FUN_1004f240(state: &mut ShellState, param_1: i32) -> i32 {
    mech_trace::enter!("1004f240", param_1);
    let result = if (0..5).contains(&param_1) {
        mech_trace::gw!("1004f240", "1009bdf8", state.dat_1009bdf8, param_1);
        0
    } else {
        2
    };
    mech_trace::rt!("1004f240", result)
}

#[cfg(test)]
mod tests {
    use super::FUN_1004f240;
    use crate::ShellState;
    use crate::test_log::debug_lines;

    /// Each edge input's pattern, the original's return, and its line in
    /// `harvest/1004f240.log`; both stack fills agree on all 31 rows. The
    /// checker logs each argument as its 32-bit pattern.
    #[expect(
        clippy::unreadable_literal,
        reason = "each pattern is written as its log line prints it, so a search finds the line"
    )]
    const HARVEST: [(u32, i32, u32); 31] = [
        (0, 0, 273),
        (1, 0, 279),
        (2, 0, 285),
        (3, 0, 291),
        (127, 2, 297),
        (128, 2, 303),
        (255, 2, 309),
        (256, 2, 315),
        (32767, 2, 321),
        (32768, 2, 327),
        (65535, 2, 333),
        (65536, 2, 339),
        (8388607, 2, 345),
        (8388608, 2, 351),
        (16777215, 2, 357),
        (16777216, 2, 363),
        (1073741823, 2, 369),
        (1073741824, 2, 375),
        (2147483647, 2, 381),
        (2147483648, 2, 387),
        (2147483649, 2, 393),
        (3221225472, 2, 399),
        (4278190080, 2, 405),
        (4286578688, 2, 411),
        (4294901760, 2, 417),
        (4294934528, 2, 423),
        (4294967040, 2, 429),
        (4294967168, 2, 435),
        (4294967293, 2, 441),
        (4294967294, 2, 447),
        (4294967295, 2, 453),
    ];

    #[test]
    fn returns_the_originals_result_on_each_harvested_input() {
        for (pattern, want, line) in HARVEST {
            let param_1 = i32::from_ne_bytes(pattern.to_ne_bytes());
            let got = FUN_1004f240(&mut ShellState::default(), param_1);
            assert_eq!(got, want, "harvest/1004f240.log:{line}, param_1 {param_1}");
        }
    }

    #[test]
    fn logs_the_write_only_when_it_stores() {
        let stored = debug_lines(|| {
            FUN_1004f240(&mut ShellState::default(), 3);
        });
        let refused = debug_lines(|| {
            FUN_1004f240(&mut ShellState::default(), 127);
        });
        assert!(stored.iter().any(|line| line.contains("g=1009bdf8")));
        assert!(!refused.iter().any(|line| line.contains("g=1009bdf8")));
    }
}
