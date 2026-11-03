// SPDX-License-Identifier: MIT
//! Falsifier 1, every call logs: each of the 11 calls once answered and
//! once refused, on the in-memory folder, writes one `imp.call` line, its
//! `c` the row's name and its `e` the error the shell reads (0 on
//! success), and leaves that error as the folder's last. The names are
//! a1's (`records/2026-11-03-lane-a/helm-files-hc9/a1-file-lines.txt`);
//! the bytes are the tests' own fill.
use std::ffi::CStr;

use crate::memory::Memory;
use crate::table::{FIND_BYTES, SHELL};
use crate::test_log::debug_lines;
use crate::{Create, Files, Mode};

/// The test's own bytes, standing for a file's.
const FILL: &[u8] = b"helm hc9 test fill";

/// A folder holding a1's names: the settings it reads, the saves it
/// finds and deletes, and the one it moves.
fn files() -> Files<Memory> {
    let folder = Memory::default()
        .give("MW2DIF.CFG", FILL)
        .give("en01star.bwd", FILL)
        .give("en02star.bwd", FILL)
        .give("ran0wld1.bwd", FILL);
    Files::new(&SHELL, folder)
}

/// An open's access, in the shell's numbers.
const fn access(read: bool, write: bool) -> u32 {
    SHELL.access(Mode {
        read,
        write,
        create: Create::Existing,
    })
}

/// An open the way a1's opens are: the row's sharing and attributes.
fn open(files: &Files<Memory>, name: &CStr, access: u32, create: Create) -> u32 {
    files.open(name, access, SHELL.share, SHELL.disposition(create), SHELL.plain)
}

/// A line's value for `key`.
fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.split(' ')
        .find_map(|word| word.strip_prefix(key)?.strip_prefix('='))
}

/// Runs `call` and checks its lines: one, `imp.call`, `c` the call's
/// name, `e` the error in hex; on a failure, the folder's last error that
/// error. Returns the call's answer.
fn logged<T>(files: &Files<Memory>, name: &str, error: u32, call: impl FnOnce() -> T) -> T {
    let mut answer = None;
    let lines = debug_lines(|| answer = Some(call()));
    assert_eq!(lines.len(), 1, "{lines:?}");
    let line = &lines[0];
    assert!(line.starts_with("imp.call "), "{line}");
    assert_eq!(field(line, "c"), Some(name), "{line}");
    assert_eq!(field(line, "e"), Some(format!("{error:x}").as_str()), "{line}");
    if error != 0 {
        assert_eq!(files.folder().last_error(), error, "{line}");
    }
    answer.expect("invariant: debug_lines runs the call")
}

#[test]
fn open_answers_a_handle_and_logs_it() {
    let files = files();
    let file = logged(&files, SHELL.calls.open, 0, || {
        open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing)
    });
    assert_ne!(file, SHELL.invalid);
}

#[test]
fn open_of_a_missing_file_fails_not_found() {
    let files = files();
    let error = SHELL.errors.not_found;
    let file = logged(&files, SHELL.calls.open, error, || {
        open(&files, c"mw2rmg.cfg", access(true, false), Create::Existing)
    });
    assert_eq!(file, SHELL.invalid);
}

#[test]
fn open_in_a_missing_folder_fails_no_path() {
    let files = files();
    let error = SHELL.errors.no_path;
    let file = logged(&files, SHELL.calls.open, error, || {
        open(&files, c"rmg\\mis7scn1.bwd", access(true, true), Create::Existing)
    });
    assert_eq!(file, SHELL.invalid);
}

#[test]
fn open_with_an_unknown_disposition_fails_bad_argument() {
    let files = files();
    let error = SHELL.errors.bad_argument;
    let file = logged(&files, SHELL.calls.open, error, || {
        files.open(c"userstar.bwd", access(false, true), SHELL.share, 0, SHELL.plain)
    });
    assert_eq!(file, SHELL.invalid);
}

#[test]
fn read_counts_the_bytes_and_logs_them() {
    let files = files();
    let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
    let mut into = vec![0; FILL.len() + 1];
    let mut count = 0;
    let done = logged(&files, SHELL.calls.read, 0, || files.read(file, &mut into, Some(&mut count)));
    assert_eq!((done, count), (1, u32::try_from(FILL.len()).expect("invariant: a short fill")));
    assert_eq!(&into[..FILL.len()], FILL);
}

#[test]
fn read_of_a_closed_handle_fails_bad_handle() {
    let files = files();
    let error = SHELL.errors.bad_handle;
    let mut count = 1;
    let done = logged(&files, SHELL.calls.read, error, || {
        files.read(SHELL.invalid, &mut [0; 8], Some(&mut count))
    });
    assert_eq!((done, count), (0, 0));
}

#[test]
fn write_counts_the_bytes_and_keeps_them() {
    let files = files();
    let file = open(&files, c"userstar.bwd", access(false, true), Create::Always);
    let mut count = 0;
    let done = logged(&files, SHELL.calls.write, 0, || files.write(file, FILL, Some(&mut count)));
    assert_eq!((done, count), (1, u32::try_from(FILL.len()).expect("invariant: a short fill")));
    assert_eq!(files.folder().contents("USERSTAR.BWD").as_deref(), Some(FILL));
}

#[test]
fn write_on_a_handle_opened_to_read_fails_denied() {
    let files = files();
    let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
    let error = SHELL.errors.denied;
    let done = logged(&files, SHELL.calls.write, error, || files.write(file, FILL, None));
    assert_eq!(done, 0);
    assert_eq!(files.folder().contents("MW2DIF.CFG").as_deref(), Some(FILL));
}

#[test]
fn seek_answers_the_position() {
    let files = files();
    let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
    let end = logged(&files, SHELL.calls.seek, 0, || files.seek(file, 0, SHELL.methods[2]));
    assert_eq!(end, u32::try_from(FILL.len()).expect("invariant: a short fill"));
}

#[test]
fn seek_before_the_start_fails_bad_seek() {
    let files = files();
    let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
    let error = SHELL.errors.bad_seek;
    let at = logged(&files, SHELL.calls.seek, error, || files.seek(file, -1, SHELL.methods[1]));
    assert_eq!(at, SHELL.invalid);
}

#[test]
fn kind_of_an_open_file_is_disk() {
    let files = files();
    let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
    let kind = logged(&files, SHELL.calls.kind, 0, || files.kind(file));
    assert_eq!(kind, SHELL.kinds[0]);
}

#[test]
fn kind_of_a_closed_handle_fails_bad_handle() {
    let files = files();
    let error = SHELL.errors.bad_handle;
    let kind = logged(&files, SHELL.calls.kind, error, || files.kind(SHELL.invalid));
    assert_eq!(kind, SHELL.unknown);
}

#[test]
fn close_answers_once() {
    let files = files();
    let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
    assert_eq!(logged(&files, SHELL.calls.close, 0, || files.close(file)), 1);
    let error = SHELL.errors.bad_handle;
    assert_eq!(logged(&files, SHELL.calls.close, error, || files.close(file)), 0);
}

#[test]
fn delete_takes_the_file_away() {
    let files = files();
    assert_eq!(logged(&files, SHELL.calls.delete, 0, || files.delete(c"en01star.bwd")), 1);
    assert_eq!(files.folder().contents("en01star.bwd"), None);
}

#[test]
fn delete_of_an_open_file_fails_busy() {
    let files = files();
    open(&files, c"en01star.bwd", access(true, true), Create::Existing);
    let error = SHELL.errors.busy;
    assert_eq!(logged(&files, SHELL.calls.delete, error, || files.delete(c"en01star.bwd")), 0);
    assert_eq!(files.folder().contents("en01star.bwd").as_deref(), Some(FILL));
}

#[test]
fn move_carries_the_bytes() {
    let files = files();
    let moved = logged(&files, SHELL.calls.rename, 0, || {
        files.rename(c"ran0wld1.bwd", c"randwld1.bwd")
    });
    assert_eq!(moved, 1);
    assert_eq!(files.folder().contents("ran0wld1.bwd"), None);
    assert_eq!(files.folder().contents("randwld1.bwd").as_deref(), Some(FILL));
}

#[test]
fn move_onto_a_taken_name_fails_taken() {
    let files = files();
    let error = SHELL.errors.taken;
    let moved = logged(&files, SHELL.calls.rename, error, || {
        files.rename(c"ran0wld1.bwd", c"en01star.bwd")
    });
    assert_eq!(moved, 0);
}

/// The name a find left in the shell's find data.
fn found(data: &[u8]) -> &CStr {
    CStr::from_bytes_until_nul(&data[SHELL.name_at..]).expect("invariant: the front ends the name")
}

#[test]
fn find_walks_the_names_in_order_then_closes() {
    let files = files();
    let mut data = [u8::MAX; FIND_BYTES];
    let find = logged(&files, SHELL.calls.find_first, 0, || files.find_first(c"*.bwd", &mut data));
    assert_ne!(find, SHELL.invalid);
    assert_eq!(found(&data), c"en01star.bwd");
    assert!(data[..SHELL.name_at].iter().all(|&byte| byte == 0));
    let next = logged(&files, SHELL.calls.find_next, 0, || files.find_next(find, &mut data));
    assert_eq!((next, found(&data)), (1, c"en02star.bwd"));
    let next = logged(&files, SHELL.calls.find_next, 0, || files.find_next(find, &mut data));
    assert_eq!((next, found(&data)), (1, c"ran0wld1.bwd"));
    let error = SHELL.errors.no_more;
    let next = logged(&files, SHELL.calls.find_next, error, || files.find_next(find, &mut data));
    assert_eq!((next, found(&data)), (0, c""));
    assert_eq!(logged(&files, SHELL.calls.find_close, 0, || files.find_close(find)), 1);
    let error = SHELL.errors.bad_handle;
    assert_eq!(logged(&files, SHELL.calls.find_close, error, || files.find_close(find)), 0);
}

#[test]
fn find_in_a_missing_folder_fails_no_path() {
    let files = files();
    let mut data = [0; FIND_BYTES];
    let error = SHELL.errors.no_path;
    let find = logged(&files, SHELL.calls.find_first, error, || {
        files.find_first(c"mek\\?????mer.mek", &mut data)
    });
    assert_eq!(find, SHELL.invalid);
}
