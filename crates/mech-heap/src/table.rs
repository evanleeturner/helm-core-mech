// SPDX-License-Identifier: MIT
//! The files' rows: where each file keeps its heap, its `_newmode` and
//! its new-handler caller, and the address and name of each of its memory
//! helpers. A file the helpers serve is a row here and nothing else.

/// One memory helper inside a file: its address, and the name its line
/// carries as `c`, the one `ORDER.tsv` gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Helper {
    pub address: u32,
    pub name: &'static str,
}

/// One file's C library, as its memory helpers see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    /// The file, as its exports folder spells it.
    pub file: &'static str,
    /// The address of the file's heap handle, made once at its start.
    pub heap: u32,
    /// The address of `_newmode`: nonzero, a failed malloc calls the new
    /// handler as operator new does.
    pub new_mode: u32,
    /// The address of `__callnewh`, which calls the new handler under
    /// the C library's own lock.
    pub call_new_handler: u32,
    pub malloc: Helper,
    pub operator_new: Helper,
    pub free: Helper,
    pub operator_delete: Helper,
    /// Absent where the file carries no copy.
    pub calloc: Option<Helper>,
    /// Absent where the file carries no copy.
    pub realloc: Option<Helper>,
}

/// The shell, `mw2shell`: its heap is made once, a growable Windows heap
/// of one 4K page at first, at `10071810`; its handle is `DAT_100d1664`.
pub const SHELL: Row = Row {
    file: "mw2shell",
    heap: 0x100d_1664,
    new_mode: 0x100a_bd54,
    call_new_handler: 0x1007_2480,
    malloc: Helper {
        address: 0x1006_fab0,
        name: "FUN_1006fab0",
    },
    operator_new: Helper {
        address: 0x1006_f200,
        name: "FUN_1006f200",
    },
    free: Helper {
        address: 0x1006_e790,
        name: "FUN_1006e790",
    },
    operator_delete: Helper {
        address: 0x1006_ec30,
        name: "FUN_1006ec30",
    },
    calloc: Some(Helper {
        address: 0x1006_e820,
        name: "FUN_1006e820",
    }),
    realloc: Some(Helper {
        address: 0x1007_0080,
        name: "FUN_10070080",
    }),
};
