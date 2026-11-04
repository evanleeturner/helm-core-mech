// SPDX-License-Identifier: MIT
//! mercs's thread rows (KH1, ruling 1405): its files' thread calls, as
//! `kernel_threads`' seam serves them. Wall 2 read every caller in the
//! exports: the C library's multithread runtime alone, its locks and its
//! per-thread data, none rebuilt; 0 in the game's own code. Wall 3 read no
//! caller reading a lock's fields. The launcher, `mercswin`, imports none
//! of the 11: it has no row.
//!
//! Each name's callers, by their names in the exports:
//!
//! | Name | `mw2shell` | `mercsw` |
//! | --- | --- | --- |
//! | `GetCurrentThreadId` | `__mtinit`, `__getptd` | `__mtinit`, `__getptd` |
//! | `InitializeCriticalSection` | `__mtinitlocks`, `__lock`, `FUN_10073800`, `__alloc_osfhnd`, `__lock_fhandle` | `__mtinitlocks`, `__lock`, `__getstream`, `__alloc_osfhnd`, `__lock_fhandle` |
//! | `EnterCriticalSection` | `__lock`, `__lock_file`, `FID_conflict:__lock_file2`, `FUN_10073800`, `__alloc_osfhnd`, `__lock_fhandle` | `__lock`, `__lock_file`, `FID_conflict:__lock_file2`, `__getstream`, `__alloc_osfhnd`, `__lock_fhandle` |
//! | `LeaveCriticalSection` | `FUN_100725d0`, `__unlock_file`, `FID_conflict:__lock_file2`, `__alloc_osfhnd`, `__unlock_fhandle` | `FUN_1008f280`, `__unlock_file`, `FID_conflict:__lock_file2`, `__alloc_osfhnd`, `__unlock_fhandle` |
//! | `DeleteCriticalSection` | `__mtdeletelocks`, `__ioterm`, `FUN_10072c40` | `__mtdeletelocks`, `__ioterm`, `__fcloseall` |
//! | `TlsAlloc` | `__mtinit` | `__mtinit` |
//! | `TlsFree` | `__mtterm` | `__mtterm` |
//! | `TlsGetValue` | `__getptd`, `__freeptd` | `__getptd`, `__freeptd` |
//! | `TlsSetValue` | `__mtinit`, `__getptd`, `__freeptd` | `__mtinit`, `__getptd`, `__freeptd` |
//! | `InterlockedIncrement` | not imported | `FUN_1008f400`, the debug heap's |
//! | `InterlockedDecrement` | not imported | `FUN_1008f400`, the debug heap's |
use kernel_threads::table::{Calls, Row};

/// The shell, `mw2shell`: 9 of the 11, no counter.
pub const SHELL: Row = Row {
    file: "mw2shell",
    calls: Calls {
        current_thread_id: "KERNEL32!GetCurrentThreadId",
        initialize: "KERNEL32!InitializeCriticalSection",
        enter: "KERNEL32!EnterCriticalSection",
        leave: "KERNEL32!LeaveCriticalSection",
        delete: "KERNEL32!DeleteCriticalSection",
        slot_alloc: "KERNEL32!TlsAlloc",
        slot_free: "KERNEL32!TlsFree",
        slot_get: "KERNEL32!TlsGetValue",
        slot_set: "KERNEL32!TlsSetValue",
        increment: None,
        decrement: None,
    },
    bad_argument: 0x57,
};

/// The game file, `mercsw`: all 11.
pub const GAME: Row = Row {
    file: "mercsw",
    calls: Calls {
        current_thread_id: "KERNEL32!GetCurrentThreadId",
        initialize: "KERNEL32!InitializeCriticalSection",
        enter: "KERNEL32!EnterCriticalSection",
        leave: "KERNEL32!LeaveCriticalSection",
        delete: "KERNEL32!DeleteCriticalSection",
        slot_alloc: "KERNEL32!TlsAlloc",
        slot_free: "KERNEL32!TlsFree",
        slot_get: "KERNEL32!TlsGetValue",
        slot_set: "KERNEL32!TlsSetValue",
        increment: Some("KERNEL32!InterlockedIncrement"),
        decrement: Some("KERNEL32!InterlockedDecrement"),
    },
    bad_argument: 0x57,
};
