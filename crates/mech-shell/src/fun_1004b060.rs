// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1004b060, from `src/mw2shell/1004b060.c` (`FUN_1004b060`): `__stdcall`, no
//! arguments, returns 0 as an `unsigned char`.

use log::debug;

/// Returns 0.
#[must_use]
pub fn FUN_1004b060() -> u8 {
    let result = 0;
    debug!("FUN_1004b060() -> {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::FUN_1004b060;
    use crate::test_log::debug_lines;

    /// The original's one row returns 0 on both stack fills
    /// (`harvest/1004b060.log:272`), and the call leaves a DEBUG line.
    #[test]
    fn returns_the_originals_result() {
        let lines = debug_lines(|| {
            assert_eq!(FUN_1004b060(), 0, "harvest/1004b060.log:272");
        });
        assert!(lines.iter().any(|line| line.contains("FUN_1004b060")));
    }
}
