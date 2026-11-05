// SPDX-License-Identifier: MIT
//! mercs's registry rows (KR1, ruling 1409): the shell's and the game
//! file's registry calls, as `kernel_registry`'s seam serves them. The
//! census `r0` (KR1, Wine's `+relay` on the original, launcher to
//! mission, the prefix's 3 values at 1) measured 10 calls, all the
//! shell's and every answer 0. In `FUN_10016740`: `RegOpenKeyExA` of the
//! key under `HKEY_LOCAL_MACHINE` (options 0, access 1),
//! `RegQueryValueExA` of each of the 3 settings (reserved null; the type,
//! size and data behind pointers), `RegCloseKey`. In `FUN_10016870`, at
//! the INSTANT ACTION click: `RegCreateKeyExA` of the same key (options
//! 0, access `0xf003f`, class and security null), `RegSetValueExA` of
//! each of the 3 as a `REG_DWORD` of 4 bytes, `RegCloseKey`. The shell's
//! `FUN_1003fbf0` and the game file's `FUN_10038f3a` were not reached.
//!
//! The 5 names and the files importing each: `RegOpenKeyExA`,
//! `RegQueryValueExA` and `RegCloseKey` the shell and the game file;
//! `RegCreateKeyExA` and `RegSetValueExA` the shell alone. The launcher
//! imports none.

use kernel_registry::table::{Calls, Errors, Row};

const CALLS: Calls = Calls {
    open_key: "ADVAPI32!RegOpenKeyExA",
    query_value: "ADVAPI32!RegQueryValueExA",
    close_key: "ADVAPI32!RegCloseKey",
    create_key: "ADVAPI32!RegCreateKeyExA",
    set_value: "ADVAPI32!RegSetValueExA",
};

/// Windows' numbers: `ERROR_FILE_NOT_FOUND`, `ERROR_INVALID_HANDLE`,
/// `ERROR_INVALID_PARAMETER`, `ERROR_MORE_DATA`, `ERROR_CANTWRITE`.
const ERRORS: Errors = Errors {
    not_found: 2,
    invalid_handle: 6,
    invalid_parameter: 87,
    more_data: 234,
    cannot_write: 1013,
};

/// `HKEY_LOCAL_MACHINE`, the root `r0`'s open and create name.
const LOCAL_MACHINE: u32 = 0x8000_0002;
/// The game's key, as both files spell it.
const KEY: &[u8] = b"Software\\Activision\\Mercenaries\\1.0";
/// The 3 settings, each 1 in the prefix the census ran on: a new player
/// starts as an installed game.
const FIRST: &[(&[u8], u32)] = &[(b"QuickTips", 1), (b"ShowDialog", 1), (b"LittleMovies", 1)];

const fn row(file: &'static str) -> Row {
    Row {
        file,
        calls: CALLS,
        root: LOCAL_MACHINE,
        key: KEY,
        first: FIRST,
        folder: "mech-mercs",
        errors: ERRORS,
    }
}

pub const SHELL: Row = row("mw2shell");
pub const GAME: Row = row("mercsw");
pub const ROWS: &[&Row] = &[&SHELL, &GAME];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_log::debug_lines;
    use kernel_registry::page::Page;
    use kernel_registry::{Front, Queried};

    static SHELL_ROW: Row = SHELL;

    /// `r0`'s 10 calls, in their order: the arguments as the shell asked,
    /// Wine's answers. The data is the prefix's 1 and the type and size
    /// the code's 4, both behind pointers the census does not read; the
    /// create's disposition is the store's, the key being there (2). The
    /// handles are the store's own, Wine's being `a8` and `dc`.
    #[test]
    fn the_census_reads_and_writes_replay() {
        let page = Page::new(&SHELL_ROW, b"", |_| {}).expect("an empty save");
        let shell = Front::new(&SHELL_ROW, &page);
        let settings: [&[u8]; 3] = [b"QuickTips", b"ShowDialog", b"LittleMovies"];
        let lines = debug_lines(|| {
            let key = shell
                .open_key(LOCAL_MACHINE, KEY, 0, 1)
                .expect("the key is there");
            for name in settings {
                let mut data = [0; 4];
                assert_eq!(
                    shell.query_value(key, name, 0, Some(4), Some(&mut data)),
                    Queried {
                        code: 0,
                        wrote: Some((4, 4))
                    }
                );
                assert_eq!(u32::from_le_bytes(data), 1);
            }
            assert_eq!(shell.close_key(key), 0);
            let (key, disposition) = shell
                .create_key(LOCAL_MACHINE, KEY, 0, 0xf_003f)
                .expect("the key opens");
            assert_eq!(disposition, 2);
            for name in settings {
                assert_eq!(shell.set_value(key, name, 4, &1_u32.to_le_bytes()), 0);
            }
            assert_eq!(shell.close_key(key), 0);
        });
        assert_eq!(
            lines,
            [
                "imp.call c=ADVAPI32!RegOpenKeyExA a=80000002,0,1 s=Software\\Activision\\Mercenaries\\1.0 o=100 v=0",
                "imp.call c=ADVAPI32!RegQueryValueExA a=100,0,4 s=QuickTips o=4,4,1 v=0",
                "imp.call c=ADVAPI32!RegQueryValueExA a=100,0,4 s=ShowDialog o=4,4,1 v=0",
                "imp.call c=ADVAPI32!RegQueryValueExA a=100,0,4 s=LittleMovies o=4,4,1 v=0",
                "imp.call c=ADVAPI32!RegCloseKey a=100 v=0",
                "imp.call c=ADVAPI32!RegCreateKeyExA a=80000002,0,f003f s=Software\\Activision\\Mercenaries\\1.0 o=104,2 v=0",
                "imp.call c=ADVAPI32!RegSetValueExA a=104,4,4 s=QuickTips o=1 v=0",
                "imp.call c=ADVAPI32!RegSetValueExA a=104,4,4 s=ShowDialog o=1 v=0",
                "imp.call c=ADVAPI32!RegSetValueExA a=104,4,4 s=LittleMovies o=1 v=0",
                "imp.call c=ADVAPI32!RegCloseKey a=104 v=0",
            ]
        );
    }
}
