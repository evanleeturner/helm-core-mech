// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10051650, from `src/mw2shell/10051650.c` (`FUN_10051650`): `__stdcall`, no
//! arguments, returns 1 as an `unsigned int`.

use log::debug;

/// Returns 1.
#[must_use]
pub fn FUN_10051650() -> u32 {
    let result = 1;
    debug!("FUN_10051650() -> {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::FUN_10051650;
    use crate::test_log::debug_lines;

    /// The original's one row returns 1 on both stack fills
    /// (`harvest/10051650.log:272`), and the call leaves a DEBUG line.
    #[test]
    fn returns_the_originals_result() {
        let lines = debug_lines(|| {
            assert_eq!(FUN_10051650(), 1, "harvest/10051650.log:272");
        });
        assert!(lines.iter().any(|line| line.contains("FUN_10051650")));
    }
}
