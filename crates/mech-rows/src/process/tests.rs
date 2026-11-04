// SPDX-License-Identifier: MIT
//! Falsifier 3, the tests harvested: the census `p0`'s answers, Wine's
//! `+relay` on the original from the launcher to the main menu
//! (`records/2026-11-03-lane-a/helm-kernel-process-kp1/p0-lines.txt`, its
//! counts in `s8-census.txt`), played on the rows. The command line is
//! `p0`'s process trace (`wine-p0.err.gz`, `winedbg`'s `CreateProcessA`).
//! A number here no `p0` line holds is a bug; where the row answers
//! otherwise than Wine, the test names the choice.
use crate::process::{LAUNCHER, SHELL};
use crate::test_log::debug_lines;
use kernel_process::page::Page;
use kernel_process::table::{Module, Row};
use kernel_process::{Access, Front};

/// The two entries the launcher asks of the shell, Wine's answers
/// (`0040158f`, `004015a6`), standing for the swap's table.
fn wine_entries(module: &Module, name: &str) -> u32 {
    match (module.name, name) {
        ("MW2SHELL.DLL", "ShellMain") => 0x1001_5aa0,
        ("MW2SHELL.DLL", "ShellWindowProc") => 0x1001_4570,
        _ => 0,
    }
}

/// `p0` asks no pointer's soundness: no map.
fn no_memory(_: u32, _: u32, _: Access) -> bool {
    false
}

fn front(row: &'static Row) -> Front<Page> {
    Front::new(row, Page::new(wine_entries, no_memory))
}

#[test]
fn the_start_up_answers_are_p0s() {
    for row in [&LAUNCHER, &SHELL] {
        let front = front(row);
        // `ret=0040228c`, `ret=100715c3`: `retval=c0000004`.
        assert_eq!(front.version(), 0xc000_0004);
        // `winedbg`'s `CreateProcessA` line: the command line, quoted.
        assert_eq!(
            front.command_line(),
            "\"C:\\PROG~FBU\\ACTI~QHY\\MERC~ZIR\\MERCSWIN.EXE\""
        );
        // `ret=004043c8`, `ret=10076528`: `(00000000,..,00000104)`,
        // `retval=00000034`.
        let mut path = [0xff; 0x104];
        assert_eq!(front.module_file_name(0, &mut path), 0x34);
        assert_eq!(
            &path[..0x35],
            b"C:\\Program Files\\Activision\\Mercenaries\\MERCSWIN.EXE\0"
        );
    }
    // `ret=00402397`: `GetModuleHandleA(00000000)`, `retval=00400000`.
    assert_eq!(front(&LAUNCHER).module_handle(None), 0x0040_0000);
}

#[test]
fn the_shell_loads_and_its_two_entries_are_found() {
    let front = front(&LAUNCHER);
    // `ret=0040155e`: `"MW2SHELL.DLL"`, `retval=10000000`.
    let shell = front.load_library(b"MW2SHELL.DLL");
    assert_eq!(shell, 0x1000_0000);
    // `ret=0040158f`, `ret=004015a6`.
    assert_eq!(front.proc_address(shell, b"ShellMain"), 0x1001_5aa0);
    assert_eq!(front.proc_address(shell, b"ShellWindowProc"), 0x1001_4570);
    assert_eq!(front.last_error(), 0);
}

#[test]
fn a_module_never_loaded_is_a_null() {
    // `ret=0040183d`, the launcher's own code: `"NETMECHW.DLL"`,
    // `retval=00000000`.
    assert_eq!(front(&LAUNCHER).module_handle(Some(b"NETMECHW.DLL")), 0);
}

#[test]
fn the_names_kernel32_lacks_are_a_logged_null() {
    let front = front(&SHELL);
    // `ret=100715f1` `"kernel32.dll"`, `ret=10072e60` `"KERNEL32"`: one
    // module, a handle not 0.
    let kernel = front.module_handle(Some(b"kernel32.dll"));
    assert_ne!(kernel, 0);
    assert_eq!(front.module_handle(Some(b"KERNEL32")), kernel);
    // `ret=10071601`: `"IsTNT"`, `retval=00000000`.
    let lines = debug_lines(|| assert_eq!(front.proc_address(kernel, b"IsTNT"), 0));
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains(" q=IsTNT "), "{}", lines[0]);
    assert_eq!(front.last_error(), 127);
    // `ret=10072e7e` to `ret=10072ef8`: Wine finds the four; the row's
    // choice answers 0, the C library's divide test then returns 0.
    for name in [
        &b"GetProcessAffinityMask"[..],
        b"GetCurrentProcess",
        b"SetThreadAffinityMask",
        b"GetCurrentThread",
    ] {
        assert_eq!(front.proc_address(kernel, name), 0);
    }
}

#[test]
fn the_wide_block_is_freed() {
    // `ret=00404718`, `ret=10076b8c`: each file's own block from
    // `GetEnvironmentStringsW`, `retval=00000001`; the narrow pair 0 calls.
    assert_eq!(front(&LAUNCHER).free_environment_w(0x004f_2980), 1);
    assert_eq!(front(&SHELL).free_environment_w(0x0050_1628), 1);
}

#[test]
fn the_shells_filter_replaces_none() {
    // `ret=10078188`: `(10078190)`, `retval=00000000`.
    assert_eq!(front(&SHELL).set_filter(0x1007_8190), 0);
}

#[test]
fn each_last_error_the_shell_read_is_kept() {
    // `ret=1006f32a` 3, `ret=1006f473` 0x12, `ret=1006f76a` 2,
    // `ret=10074468` 0xb7, 2, 0 and 0x57: set by other names' calls.
    let front = front(&SHELL);
    for code in [3, 0x12, 2, 0xb7, 0, 0x57] {
        front.set_last_error(code);
        assert_eq!(front.last_error(), code);
    }
}
