// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 100406b0, from `src/mw2shell/100406b0.c` (`FUN_100406b0`): `__stdcall`, no
//! arguments, returns 0 as an `unsigned int`.

/// Returns 0.
#[must_use]
pub fn FUN_100406b0() -> u32 {
    let result = 0;
    mech_trace::rt!("100406b0", result)
}

#[cfg(test)]
mod tests {
    use super::FUN_100406b0;
    use crate::test_log::debug_lines;

    /// The original's one row returns 0 on both stack fills
    /// (`harvest/100406b0.log:272`), and the call leaves a DEBUG line.
    #[test]
    fn returns_the_originals_result() {
        let lines = debug_lines(|| {
            assert_eq!(FUN_100406b0(), 0, "harvest/100406b0.log:272");
        });
        assert!(lines.iter().any(|line| line.contains("f=100406b0")));
    }
}
