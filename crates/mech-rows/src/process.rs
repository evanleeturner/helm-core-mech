// SPDX-License-Identifier: MIT
//! mercs's process rows (KP1, ruling 1407): the launcher's, the shell's and
//! the game file's process calls, as `kernel_process`'s seam serves them.
//! The census `p0` (KP1, Wine's `+relay` on the original to the main menu)
//! measured the launcher's and the shell's answers; the game file never
//! loaded, so its row is the launcher's, unmeasured. A relay line shows a
//! pointer, never what it points at: the start-up record and the
//! environment block stay unmeasured, and `GetCurrentProcess` has no line
//! at all, Wine's own calls included. The tests replaying `p0` live in
//! `tests`.
//!
//! The 29 names and the files importing each (`mercswin` the launcher,
//! `mw2shell` the shell, `mercsw` the game file):
//!
//! | Name | Files |
//! | --- | --- |
//! | `CreateProcessA` | mw2shell |
//! | `DebugBreak` | mercsw |
//! | `ExitProcess` | all three |
//! | `FreeEnvironmentStringsA` | all three |
//! | `FreeEnvironmentStringsW` | all three |
//! | `FreeLibrary` | mercswin |
//! | `GetCommandLineA` | all three |
//! | `GetCurrentProcess` | all three |
//! | `GetEnvironmentStrings` | all three |
//! | `GetEnvironmentStringsW` | all three |
//! | `GetLastError` | all three |
//! | `GetModuleFileNameA` | all three |
//! | `GetModuleHandleA` | all three |
//! | `GetProcAddress` | all three |
//! | `GetStartupInfoA` | all three |
//! | `GetVersion` | all three |
//! | `IsBadCodePtr` | mw2shell |
//! | `IsBadReadPtr` | mw2shell, mercsw |
//! | `IsBadWritePtr` | mw2shell, mercsw |
//! | `LoadLibraryA` | all three |
//! | `LoadModule` | mw2shell, mercsw |
//! | `OutputDebugStringA` | all three |
//! | `RaiseException` | mw2shell, mercsw |
//! | `RtlUnwind` | all three |
//! | `SetEnvironmentVariableA` | mw2shell, mercsw |
//! | `SetLastError` | mw2shell, mercsw |
//! | `SetUnhandledExceptionFilter` | mw2shell |
//! | `TerminateProcess` | all three |
//! | `UnhandledExceptionFilter` | mercswin |
use kernel_process::table::{Calls, Errors, Module, Row, Startup};

#[cfg(test)]
mod tests;

/// Each call's name, the layer's spelling.
const CALLS: Calls = Calls {
    command_line: "KERNEL32!GetCommandLineA",
    module_handle: "KERNEL32!GetModuleHandleA",
    module_file_name: "KERNEL32!GetModuleFileNameA",
    version: "KERNEL32!GetVersion",
    current_process: "KERNEL32!GetCurrentProcess",
    load_library: "KERNEL32!LoadLibraryA",
    proc_address: "KERNEL32!GetProcAddress",
    free_library: "KERNEL32!FreeLibrary",
    last_error: "KERNEL32!GetLastError",
    set_last_error: "KERNEL32!SetLastError",
    exit: "KERNEL32!ExitProcess",
    environment: "KERNEL32!GetEnvironmentStrings",
    environment_w: "KERNEL32!GetEnvironmentStringsW",
    free_environment: "KERNEL32!FreeEnvironmentStringsA",
    free_environment_w: "KERNEL32!FreeEnvironmentStringsW",
    startup_info: "KERNEL32!GetStartupInfoA",
    set_environment: "KERNEL32!SetEnvironmentVariableA",
    terminate: "KERNEL32!TerminateProcess",
    load_module: "KERNEL32!LoadModule",
    create_process: "KERNEL32!CreateProcessA",
    debug_string: "KERNEL32!OutputDebugStringA",
    debug_break: "KERNEL32!DebugBreak",
    raise: "KERNEL32!RaiseException",
    set_filter: "KERNEL32!SetUnhandledExceptionFilter",
    unhandled_filter: "KERNEL32!UnhandledExceptionFilter",
    bad_read: "KERNEL32!IsBadReadPtr",
    bad_write: "KERNEL32!IsBadWritePtr",
    bad_code: "KERNEL32!IsBadCodePtr",
    unwind: "KERNEL32!RtlUnwind",
};

/// The program's name table. The handles are the stand-in's own: Wine
/// loads both DLLs at `0x1000_0000`, never together, and the launcher
/// compares a handle with 0 alone. `KERNEL32.DLL` holds no entry, so
/// `IsTNT` and the FDIV test's four names answer a logged 0. Wine finds
/// the four (`p0`); the C library's test returns 0 at any null (wall 4), a
/// choice.
const MODULES: &[Module] = &[
    Module {
        name: "MW2SHELL.DLL",
        handle: 0x1000_0000,
        entries: &["ShellMain", "ShellWindowProc"],
    },
    Module {
        name: "MERCSW.DLL",
        handle: 0x1100_0000,
        entries: &["SimMain", "SimWindowProc"],
    },
    Module {
        name: "USER32.DLL",
        handle: 0x1200_0000,
        entries: &["MessageBoxA"],
    },
    Module {
        name: "KERNEL32.DLL",
        handle: 0x1300_0000,
        entries: &[],
    },
];

/// The launcher, `mercswin`. The command line is the rig's, quoted, as
/// `winedbg` starts the program (`p0`'s process trace); the path is the
/// image's long name, `p0`'s count 52 for both files. `own` the program's
/// handle, which a DLL's `GetModuleHandleA(NULL)` answers too (`p0`).
/// `version` is Wine's for its `win95` setting (`p0`): the top bit set, so
/// `1000e6cd` asks `VirtualProtect` for flag 4 (clear, `0x40`);
/// `kernel_memory` takes no flag. The current process is Wine's pseudo
/// handle (Windows 95's own is `0x7FFF_FFFF`: a choice). No variable a
/// reader reads, the start-up record all 0 (wall 4). The errors are
/// Windows' numbers.
pub const LAUNCHER: Row = Row {
    file: "mercswin",
    calls: CALLS,
    command_line: "\"C:\\PROG~FBU\\ACTI~QHY\\MERC~ZIR\\MERCSWIN.EXE\"",
    own: 0x0040_0000,
    path: "C:\\Program Files\\Activision\\Mercenaries\\MERCSWIN.EXE",
    version: 0xC000_0004,
    current_process: 0xFFFF_FFFF,
    environment: b"\0\0",
    startup: Startup {
        flags: 0,
        show_window: 0,
        reserved_count: 0,
        reserved: 0,
    },
    modules: MODULES,
    errors: Errors {
        module_not_found: 126,
        name_not_found: 127,
        bad_handle: 6,
        file_not_found: 2,
    },
};

/// The shell, `mw2shell`: the launcher's answers.
pub const SHELL: Row = Row {
    file: "mw2shell",
    ..LAUNCHER
};

/// The game file, `mercsw`: the launcher's answers.
pub const GAME: Row = Row {
    file: "mercsw",
    ..LAUNCHER
};

/// Every file the process seam serves.
pub const ROWS: &[&Row] = &[&LAUNCHER, &SHELL, &GAME];
