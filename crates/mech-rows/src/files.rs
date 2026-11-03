// SPDX-License-Identifier: MIT
//! mercs's files rows (HC9): the shell's file calls, as `kernel_files`'
//! seam serves them. a1's names behind the in-memory folder's rules: it
//! deletes `USERSTAR.BWD`, then opens `userstar.bwd`; 3 calls name
//! `mek\?????mer.mek`, in a folder never given.
use kernel_files::table::{Calls, Errors, Row};

#[cfg(test)]
mod tests;

/// The shell, `mw2shell`: its C library (`_sopen` `10077d00`, `_read_lk`
/// `100747f0`, `_write_lk` `10072800`, `__lseek_lk` `10074af0`,
/// `__close_lk` `10073520`, `remove` `1006f750`, `rename` `10070c20`,
/// `__findfirst` `1006f300`, `__findnext` `1006f450`, `_findclose`
/// `1006f590`) makes every call (wall 2); the numbers are its imports'.
pub const SHELL: Row = Row {
    file: "mw2shell",
    calls: Calls {
        open: "KERNEL32!CreateFileA",
        read: "KERNEL32!ReadFile",
        write: "KERNEL32!WriteFile",
        seek: "KERNEL32!SetFilePointer",
        kind: "KERNEL32!GetFileType",
        close: "KERNEL32!CloseHandle",
        delete: "KERNEL32!DeleteFileA",
        rename: "KERNEL32!MoveFileA",
        find_first: "KERNEL32!FindFirstFileA",
        find_next: "KERNEL32!FindNextFileA",
        find_close: "KERNEL32!FindClose",
    },
    read: 0x8000_0000,
    write: 0x4000_0000,
    share: 3,
    plain: 0x80,
    creates: [1, 2, 3, 4, 5],
    methods: [0, 1, 2],
    kinds: [1, 2, 3],
    unknown: 0,
    invalid: 0xffff_ffff,
    errors: Errors {
        not_found: 2,
        no_path: 3,
        denied: 5,
        bad_handle: 6,
        no_more: 0x12,
        exists: 0x50,
        busy: 0x20,
        taken: 0xb7,
        bad_seek: 0x83,
        bad_argument: 0x57,
    },
    name_at: 44,
    name_bytes: 260,
};

/// Every file the seam serves, one row each.
pub const ROWS: &[&Row] = &[&SHELL];
