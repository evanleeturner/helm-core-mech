// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1005b5f0, from `src/mw2shell/1005b5f0.c`
//! (`ReturnRecordCountOfOne_Likely`): `__stdcall`, no arguments, returns 1 as
//! an `unsigned int`.

use log::debug;

/// Returns 1.
#[must_use]
pub fn FUN_1005b5f0() -> u32 {
    let result = 1;
    debug!("FUN_1005b5f0() -> {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::FUN_1005b5f0;
    use crate::test_log::debug_lines;

    /// The original's one row returns 1 on both stack fills
    /// (`harvest/1005b5f0.log:272`), and the call leaves a DEBUG line.
    #[test]
    fn returns_the_originals_result() {
        let lines = debug_lines(|| {
            assert_eq!(FUN_1005b5f0(), 1, "harvest/1005b5f0.log:272");
        });
        assert!(lines.iter().any(|line| line.contains("FUN_1005b5f0")));
    }
}
