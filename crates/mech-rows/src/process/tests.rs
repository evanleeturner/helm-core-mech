// SPDX-License-Identifier: MIT
//! Falsifier 3, the tests harvested: the census `p0`'s answers, Wine's
//! `+relay` on the original from the launcher to the main menu
//! (`records/2026-11-03-lane-a/helm-kernel-process-kp1/p0-lines.txt`, its
//! counts in `s8-census.txt`), played on the rows. The command line is
//! `p0`'s process trace (`wine-p0.err.gz`, `winedbg`'s `CreateProcessA`).
//! A number here no `p0` line holds is a bug; where the row answers
//! otherwise than Wine, the test names the choice. The tests after
//! `p0`'s take `p10`'s logbook (`logbook-p10.log`, the swap's `imp.call`
//! lines from the launcher to the game's end, counted in `s23-count.txt`
//! and split by file in `s23-split.txt`) the same way.
use crate::process::{GAME, LAUNCHER, SHELL};
use crate::test_log::debug_lines;
use kernel_process::page::Page;
use kernel_process::table::{Module, Row};
use kernel_process::{Access, Front};

/// `mech_layer::process` hands all 3 files one NUL-ended command line, one wide
/// block, and the row's narrow block by its address: a row whose text differs, or
/// a block with no 2 NULs at its end, would hand a caller the wrong bytes.
#[test]
fn the_three_rows_share_the_text_the_layer_keeps_one_copy_of() {
    for row in [&SHELL, &GAME] {
        assert_eq!(row.command_line, LAUNCHER.command_line, "{}", row.file);
        assert_eq!(row.environment, LAUNCHER.environment, "{}", row.file);
    }
    assert!(LAUNCHER.environment.ends_with(b"\0\0"));
    assert!(!LAUNCHER.command_line.contains('\0'));
}

/// The four entries the launcher asks, Wine's answers standing for the
/// swap's table: the shell's (`p0`, `0040158f`, `004015a6`), the game
/// file's (`p10`, `from=4014db`, `from=4014f2`).
fn wine_entries(module: &Module, name: &str) -> u32 {
    match (module.name, name) {
        ("MW2SHELL.DLL", "ShellMain") => 0x1001_5aa0,
        ("MW2SHELL.DLL", "ShellWindowProc") => 0x1001_4570,
        ("MERCSW.DLL", "SimMain") => 0x1003_6b60,
        ("MERCSW.DLL", "SimWindowProc") => 0x1003_7859,
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

#[test]
fn the_shell_is_freed_and_the_game_loads_its_two_entries_found() {
    let front = front(&LAUNCHER);
    // `from=4015f0`: `a=10000000 v=1 e=0`, the shell freed.
    let shell = front.load_library(b"MW2SHELL.DLL");
    assert_eq!(front.free_library(shell), 1);
    // Wine's error after the load, `e=57`, is set inside it and the
    // launcher never reads it: the row keeps the program's own.
    front.set_last_error(0x57);
    // `from=4014be`: `m=MERCSW.DLL v=10000000`. The row's handle is
    // `0x1100_0000`, a choice: the launcher compares it with 0 alone.
    let lines = debug_lines(|| assert_eq!(front.load_library(b"MERCSW.DLL"), 0x1100_0000));
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains(" m=MERCSW.DLL "), "{}", lines[0]);
    // `from=4014db` `q=SimMain v=10036b60`, `from=4014f2`
    // `q=SimWindowProc v=10037859`.
    let game = front.load_library(b"MERCSW.DLL");
    assert_eq!(front.proc_address(game, b"SimMain"), 0x1003_6b60);
    assert_eq!(front.proc_address(game, b"SimWindowProc"), 0x1003_7859);
    assert_eq!(front.last_error(), 0x57);
}

#[test]
fn the_shells_filter_is_put_back() {
    let front = front(&SHELL);
    // `p0`'s `ret=10078188` sets it; `p10`'s `from=100781fc`, the shell's
    // end before the game loads: `a=0 v=10078190`.
    assert_eq!(front.set_filter(0x1007_8190), 0);
    assert_eq!(front.set_filter(0), 0x1007_8190);
}

/// The one block the game file's C library asked of the map in `p10`.
fn p10_block(address: u32, size: u32, _: Access) -> bool {
    address >= 0x0171_1720 && address + size <= 0x0171_1740
}

#[test]
fn the_game_files_pointer_test_finds_its_block_sound() {
    // `from=1008e2ae` and `from=1008e2ce`, `__CrtIsValidPointer`, twice
    // each: `a=1711720,20 v=0`.
    let front = Front::new(&GAME, Page::new(wine_entries, p10_block));
    for _ in 0..2 {
        assert_eq!(front.bad_read(0x0171_1720, 0x20), 0);
        assert_eq!(front.bad_write(0x0171_1720, 0x20), 0);
    }
}

#[test]
fn each_last_error_saved_is_put_back() {
    // `__getptd`'s pair in the game file, `from=1009381f` `v=0` then
    // `from=100938b2` `a=0`, 3941 each; the shell's rebuilt callers' 31
    // and 20, `v=2`, 3 and 0x12, `a=2`, 3 and 0x12.
    for row in [&GAME, &SHELL] {
        let front = front(row);
        for code in [0, 2, 3, 0x12] {
            front.set_last_error(code);
            let saved = front.last_error();
            front.set_last_error(saved);
            assert_eq!(front.last_error(), code);
        }
    }
}
