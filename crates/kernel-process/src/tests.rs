// SPDX-License-Identifier: MIT
//! The front's rules on a row made for the test, no game's: the name
//! table's match and refusals, the last error, the path cut, the end.
//! The harvested tests live in the game's rows crate.
use crate::page::Page;
use crate::table::{Calls, Errors, Module, Row};
use crate::{Front, Process, names};

const CALLS: Calls = Calls {
    command_line: "K!GetCommandLineA",
    module_handle: "K!GetModuleHandleA",
    module_file_name: "K!GetModuleFileNameA",
    version: "K!GetVersion",
    current_process: "K!GetCurrentProcess",
    load_library: "K!LoadLibraryA",
    proc_address: "K!GetProcAddress",
    free_library: "K!FreeLibrary",
    last_error: "K!GetLastError",
    set_last_error: "K!SetLastError",
    exit: "K!ExitProcess",
};

const ROW: Row = Row {
    file: "test",
    calls: CALLS,
    command_line: "PROG.EXE -x",
    own: 0x0040_0000,
    path: "C:\\DIR\\PROG.EXE",
    version: 0xc000_0a04,
    current_process: 0xffff_ffff,
    modules: &[
        Module {
            name: "LIB.DLL",
            handle: 0x1000_0000,
            entries: &["Main"],
        },
        Module {
            name: "SYS.DLL",
            handle: 0x2000_0000,
            entries: &[],
        },
    ],
    errors: Errors {
        module_not_found: 126,
        name_not_found: 127,
        bad_handle: 6,
    },
};

fn entries(module: &Module, name: &str) -> u32 {
    u32::from(module.name == "LIB.DLL" && name == "Main") * 7
}

fn front() -> Front<Page> {
    Front::new(&ROW, Page::new(entries))
}

#[test]
fn a_module_name_matches_as_windows_does() {
    assert!(names(b"LIB.DLL", "LIB.DLL"));
    assert!(names(b"lib", "LIB.DLL"));
    assert!(names(b"C:\\X\\Lib.Dll", "LIB.DLL"));
    assert!(!names(b"LIB.EXE", "LIB.DLL"));
    assert!(!names(b"LI", "LIB.DLL"));
}

#[test]
fn a_name_in_the_table_is_found() {
    let front = front();
    let handle = front.load_library(b"lib.dll");
    assert_eq!(handle, 0x1000_0000);
    assert_eq!(front.proc_address(handle, b"Main"), 7);
    assert_eq!(front.free_library(handle), 1);
    assert_eq!(front.process().last_error(), 0);
}

#[test]
fn a_missing_name_is_a_null_and_its_error() {
    let front = front();
    assert_eq!(front.load_library(b"NONE.DLL"), 0);
    assert_eq!(front.last_error(), 126);
    let system = front.module_handle(Some(b"sys.dll"));
    assert_eq!(system, 0x2000_0000);
    assert_eq!(front.proc_address(system, b"IsTNT"), 0);
    assert_eq!(front.last_error(), 127);
    assert_eq!(front.proc_address(0x3000_0000, b"Main"), 0);
    assert_eq!(front.last_error(), 6);
    assert_eq!(front.free_library(0), 0);
}

#[test]
fn the_start_up_answers_are_the_row() {
    let front = front();
    assert_eq!(front.command_line(), "PROG.EXE -x");
    assert_eq!(front.version(), 0xc000_0a04);
    assert_eq!(front.module_handle(None), 0x0040_0000);
    let mut path = [0xff; 8];
    assert_eq!(front.module_file_name(0, &mut path), 7);
    assert_eq!(&path, b"C:\\DIR\\\0");
    assert_eq!(front.module_file_name(0x1000_0000, &mut path), 0);
    assert_eq!(front.last_error(), 6);
}

#[test]
fn the_last_error_set_is_got_and_the_end_kept() {
    let front = front();
    front.set_last_error(0x57);
    assert_eq!(front.last_error(), 0x57);
    front.exit(3);
    assert_eq!(front.process().ended(), Some(3));
}
