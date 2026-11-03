// SPDX-License-Identifier: MIT
//! The files' rows: each call's name as the counter labels it, and every
//! number the shell's file calls pass or read back, in the shell's own
//! spelling. A file the seam serves is a row here and nothing else.

/// The find data's size: the buffer a find call fills.
pub const FIND_BYTES: usize = 320;

/// Each call's name, the line's `c`: the import as the counter spells it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calls {
    pub open: &'static str,
    pub read: &'static str,
    pub write: &'static str,
    pub seek: &'static str,
    pub kind: &'static str,
    pub close: &'static str,
    pub delete: &'static str,
    pub rename: &'static str,
    pub find_first: &'static str,
    pub find_next: &'static str,
    pub find_close: &'static str,
}

/// The last error each [`crate::Error`] leaves, as the shell reads it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Errors {
    pub not_found: u32,
    pub no_path: u32,
    pub denied: u32,
    pub bad_handle: u32,
    pub no_more: u32,
    pub exists: u32,
    pub busy: u32,
    pub taken: u32,
    pub bad_seek: u32,
    pub bad_argument: u32,
}

/// One file's file calls, as the seam sees them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    /// The file, as its exports folder spells it.
    pub file: &'static str,
    pub calls: Calls,
    /// The access bit an open to read passes.
    pub read: u32,
    /// The access bit an open to write passes.
    pub write: u32,
    /// The sharing every open of the shell's passes (a1: 48 of 48).
    pub share: u32,
    /// The attributes every open of the shell's passes (a1: 48 of 48).
    pub plain: u32,
    /// The dispositions, in [`crate::Create`]'s order.
    pub creates: [u32; 5],
    /// The seek methods, in [`crate::Origin`]'s order.
    pub methods: [u32; 3],
    /// The file types, in [`crate::Kind`]'s order.
    pub kinds: [u32; 3],
    /// The type a failed ask answers.
    pub unknown: u32,
    /// A failed open's or find's handle, and a failed seek's position.
    pub invalid: u32,
    pub errors: Errors,
    /// Where the found name sits in the find data, and its room, its NUL
    /// included.
    pub name_at: usize,
    pub name_bytes: usize,
}

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
pub const FILES: &[&Row] = &[&SHELL];
