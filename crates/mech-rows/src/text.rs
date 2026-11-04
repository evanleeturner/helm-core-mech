// SPDX-License-Identifier: MIT
//! mercs's text rows (KX1, ruling 1408): the launcher's, the shell's and
//! the game file's text calls, as `kernel_text`'s seam serves them. The
//! census `x0` (KX1, Wine's `+relay` on the original to the main menu, the
//! game file loaded) measured: `GetACP` 1252 in all three files,
//! `GetCPInfo(1252)` TRUE, `WideCharToMultiByte` code page 0 flags 0, and
//! `CompareStringA` locale 0 flags 0 and 1, all 28 calls in each file's C
//! library start-up. `GetOEMCP`, the locale's number and its texts were
//! never asked: 437 and `0x409` stand unmeasured, and no text is held.
//!
//! The 13 names and the files importing each: `GetACP`, `GetCPInfo`,
//! `GetOEMCP`, `GetStringTypeA`, `GetStringTypeW`, `LCMapStringA`,
//! `LCMapStringW`, `MultiByteToWideChar` and `WideCharToMultiByte` all
//! three; `CompareStringA`, `CompareStringW`, `GetLocaleInfoA` and
//! `GetLocaleInfoW` the shell and the game file.

use kernel_text::table::{Calls, Errors, Row};

const CALLS: Calls = Calls {
    ansi_code_page: "KERNEL32!GetACP",
    oem_code_page: "KERNEL32!GetOEMCP",
    code_page_info: "KERNEL32!GetCPInfo",
    to_wide: "KERNEL32!MultiByteToWideChar",
    to_bytes: "KERNEL32!WideCharToMultiByte",
    string_type: "KERNEL32!GetStringTypeA",
    string_type_w: "KERNEL32!GetStringTypeW",
    map_string: "KERNEL32!LCMapStringA",
    map_string_w: "KERNEL32!LCMapStringW",
    compare: "KERNEL32!CompareStringA",
    compare_w: "KERNEL32!CompareStringW",
    locale_info: "KERNEL32!GetLocaleInfoA",
    locale_info_w: "KERNEL32!GetLocaleInfoW",
};

/// Windows' numbers: `ERROR_INVALID_PARAMETER`,
/// `ERROR_INSUFFICIENT_BUFFER`, `ERROR_INVALID_FLAGS`.
const ERRORS: Errors = Errors {
    invalid_parameter: 87,
    insufficient_buffer: 122,
    invalid_flags: 1004,
};

const fn row(file: &'static str) -> Row {
    Row {
        file,
        calls: CALLS,
        ansi: 1252,
        oem: 437,
        locale: 0x409,
        locale_info: &[],
        errors: ERRORS,
    }
}

pub const LAUNCHER: Row = row("mercswin");
pub const SHELL: Row = row("mw2shell");
pub const GAME: Row = row("mercsw");
pub const ROWS: &[&Row] = &[&LAUNCHER, &SHELL, &GAME];

#[cfg(test)]
mod tests {
    use super::*;
    use kernel_text::{Front, Portable};

    static SERVED: Row = GAME;
    static SHELL_ROW: Row = SHELL;

    /// `x0`'s neutral `CompareStringA` rows (the others carry the box's
    /// environment): the arguments as asked, Wine's answer.
    #[test]
    fn the_census_sort_rows_replay() {
        let game = Front::new(&SERVED, Portable::new(|_| {}));
        let shell = Front::new(&SHELL_ROW, Portable::new(|_| {}));
        assert_eq!(game.compare(0, 0, b"\0", b"\0"), 2);
        assert_eq!(game.compare(0, 1, b"SESSIONNAME", b"MECHWARRIOR"), 3);
        assert_eq!(game.compare(0, 1, b"SystemDrive", b"MECHWARRIOR"), 3);
        assert_eq!(game.compare(0, 1, b"VKD3D_DEBUG", b"MECHWARRIOR"), 3);
        assert_eq!(shell.compare(0, 0, b"\0", b"\0"), 2);
        assert_eq!(shell.compare(0, 1, b"OS", b"TZ"), 1);
        assert_eq!(shell.ansi_code_page(), 1252);
        assert!(shell.code_page_info(1252).is_some());
    }
}
