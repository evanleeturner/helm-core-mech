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
    /// Absent where the file carries no copy: a C program has none.
    pub operator_new: Option<Helper>,
    pub free: Helper,
    /// Absent where the file carries no copy: a C program has none.
    pub operator_delete: Option<Helper>,
    /// Absent where the file carries no copy, or none the rules serve.
    pub calloc: Option<Helper>,
    /// Absent where the file carries no copy, or none the rules serve.
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
    operator_new: Some(Helper {
        address: 0x1006_f200,
        name: "FUN_1006f200",
    }),
    free: Helper {
        address: 0x1006_e790,
        name: "FUN_1006e790",
    },
    operator_delete: Some(Helper {
        address: 0x1006_ec30,
        name: "FUN_1006ec30",
    }),
    calloc: Some(Helper {
        address: 0x1006_e820,
        name: "FUN_1006e820",
    }),
    realloc: Some(Helper {
        address: 0x1007_0080,
        name: "FUN_10070080",
    }),
};

/// The launcher, `mercswin`: its heap is made once, unserialized, of one
/// 4K page at first, at `00402420`; its handle is `DAT_0040a264`. Its new-handler
/// caller, `00403080`, takes no lock.
pub const LAUNCHER: Row = Row {
    file: "mercswin",
    heap: 0x0040_a264,
    new_mode: 0x0040_8650,
    call_new_handler: 0x0040_3080,
    malloc: Helper {
        address: 0x0040_54a0,
        name: "FUN_004054a0",
    },
    operator_new: None,
    free: Helper {
        address: 0x0040_19a0,
        name: "FUN_004019a0",
    },
    operator_delete: None,
    calloc: Some(Helper {
        address: 0x0040_1a30,
        name: "FUN_00401a30",
    }),
    realloc: None,
};

/// Every file the helpers serve, one row each.
pub const FILES: &[&Row] = &[&SHELL, &LAUNCHER];

/// A debug C library's list of live blocks, as its make and free keep it:
/// the list's two ends, and the lock both take around it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct List {
    /// The address of the newest block's header, the list's first end.
    pub first: u32,
    /// The address of the oldest block's header, the list's last end.
    pub last: u32,
    /// The address of `__lock`, which takes the C library's lock `number`.
    pub lock: u32,
    /// The address of the call that gives the lock back.
    pub unlock: u32,
    /// The heap's lock among the C library's own, `_HEAP_LOCK`.
    pub number: u32,
}

/// The game, `mercsw`, on the 1998 debug C library (HC8, wall 3): its heap
/// is made once, a growable Windows heap of one 4K page at first, at
/// `10096290`, called by the file's entry on attach; its handle is
/// `DAT_10218180`. The game calls only malloc and free: it names no
/// operator new or delete, and its calloc and realloc copies stay unread.
pub const MERCSW: Row = Row {
    file: "mercsw",
    heap: 0x1021_8180,
    new_mode: 0x100c_3804,
    call_new_handler: 0x1009_5f20,
    malloc: Helper {
        address: 0x1008_cf60,
        name: "_malloc",
    },
    operator_new: None,
    free: Helper {
        address: 0x1008_d9b0,
        name: "FUN_1008d9b0",
    },
    operator_delete: None,
    calloc: None,
    realloc: None,
};

/// `mercsw`'s list: its ends `DAT_100c8df8` and `DAT_100c8df0`, and lock 9,
/// taken by `__lock` and given back by `FUN_1008f280`.
pub const MERCSW_LIST: List = List {
    first: 0x100c_8df8,
    last: 0x100c_8df0,
    lock: 0x1008_f1d0,
    unlock: 0x1008_f280,
    number: 9,
};

/// Every file on the debug rules, its row and its list.
pub const DEBUG_FILES: &[(&Row, &List)] = &[(&MERCSW, &MERCSW_LIST)];
