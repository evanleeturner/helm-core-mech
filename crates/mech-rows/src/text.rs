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
    use kernel_text::{Front, Info, Portable};

    static SERVED: Row = GAME;
    static SHELL_ROW: Row = SHELL;
    static LAUNCHER_ROW: Row = LAUNCHER;

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
    }

    /// `x0`'s code page rows, each file's own: `GetACP` 1252, then
    /// `GetCPInfo(1252)` TRUE. The relay showed TRUE alone; the record is
    /// the one Wine wrote for that page to KX1's probe (`cp-probe.out`):
    /// one byte a character, `?` the default, no lead bytes.
    #[test]
    fn the_census_code_page_rows_replay() {
        let wine = Info {
            max_char_size: 1,
            default: [b'?', 0],
            lead_bytes: [0; 12],
        };
        for row in [&LAUNCHER_ROW, &SHELL_ROW, &SERVED] {
            let front = Front::new(row, Portable::new(|_| {}));
            assert_eq!(front.ansi_code_page(), 1252, "{}", row.file);
            assert_eq!(front.code_page_info(1252), Some(wine), "{}", row.file);
        }
    }

    /// `x0`'s `WideCharToMultiByte` pairs, each file's own: code page 0,
    /// flags 0, the environment block's units counted, sized with no buffer
    /// and then written into one that size, no default given, Wine's answer
    /// the count both times. The relay showed the block's first string
    /// alone, so a neutral ASCII block of each count stands in: the shape
    /// replayed, not the input. Its bytes are falsifier 4's, Wine's own.
    #[test]
    fn the_census_environment_rows_replay() {
        for (row, count) in [
            (&LAUNCHER_ROW, 0x1a84),
            (&SHELL_ROW, 0x1aab),
            (&SERVED, 0x1aab),
        ] {
            let front = Front::new(row, Portable::new(|_| {}));
            let units = block(count);
            let wine = i32::try_from(count).expect("a count");
            assert_eq!(
                front.to_bytes(0, 0, &units, None, None).0,
                wine,
                "{}",
                row.file
            );
            let mut out = vec![0xff; count];
            let written = front.to_bytes(0, 0, &units, Some(&mut out), None).0;
            assert_eq!(written, wine, "{}", row.file);
            assert!(
                out.iter()
                    .zip(&units)
                    .all(|(&byte, &unit)| u16::from(byte) == unit)
            );
        }
    }

    /// A neutral environment block of `count` units: `KEY=VALUE` strings
    /// ended by a NUL each, the block by a second.
    fn block(count: usize) -> Vec<u16> {
        let mut units: Vec<u16> = b"KEY=VALUE\0"
            .iter()
            .cycle()
            .take(count - 2)
            .map(|&byte| u16::from(byte))
            .collect();
        units.extend([0, 0]);
        units
    }

    /// `x1`'s game-file `CompareStringA` lines, logged through the layer's
    /// rows, beyond the census's: the 11 bytes each call compared, Wine's
    /// answer. The four naming the box's own tools stay in the private
    /// record.
    #[test]
    fn the_flown_sort_rows_replay() {
        let game = Front::new(&SERVED, Portable::new(|_| {}));
        assert_eq!(game.compare(0, 1, b"LOGONSERVER", b"MECHWARRIOR"), 1);
        assert_eq!(game.compare(0, 1, b"USERPROFILE", b"MECHWARRIOR"), 3);
        assert_eq!(game.compare(0, 1, b"ProgramData", b"MECHWARRIOR"), 3);
        assert_eq!(game.compare(0, 1, b"WINEDATADIR", b"MECHWARRIOR"), 3);
        assert_eq!(game.compare(0, 1, b"WINEHOMEDIR", b"MECHWARRIOR"), 3);
        assert_eq!(game.compare(0, 1, b"WINEDLLDIR0", b"MECHWARRIOR"), 3);
    }
}
