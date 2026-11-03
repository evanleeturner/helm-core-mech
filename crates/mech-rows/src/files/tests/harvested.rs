// SPDX-License-Identifier: MIT
//! Falsifier 3, the tests harvested: a1's own runs of file calls, played on
//! the in-memory folder in a1's order, with a1's names, counts, positions,
//! answers and errors (`records/2026-11-03-lane-a/helm-files-hc9/
//! a1-file-lines.txt`, each test naming its stretch by that file's line
//! numbers). A number here no a1 line holds is a bug. A file's bytes are the test's own fill, repeated to
//! a1's count; never the disc's.
use std::ffi::{CStr, CString};

use super::{FILL, access, field, found, logged, open};
use crate::files::SHELL;
use crate::test_log::debug_lines;
use kernel_files::memory::MemoryFolder;
use kernel_files::table::FIND_BYTES;
use kernel_files::{Create, Files};

/// The test's own bytes, `count` of them.
fn fill(count: u32) -> Vec<u8> {
    FILL.iter().copied().cycle().take(count as usize).collect()
}

/// A name as the folder's `contents` takes it.
fn text(name: &CStr) -> &str {
    name.to_str().expect("invariant: a1's names are ASCII")
}

/// A write's count, once it answers 1.
fn written(files: &Files<MemoryFolder>, file: u32, from: &[u8]) -> u32 {
    let mut count = 0;
    assert_eq!(files.write(file, from, Some(&mut count)), 1);
    count
}

/// A read's bytes, `room` asked for, once it answers 1.
fn read(files: &Files<MemoryFolder>, file: u32, room: usize) -> Vec<u8> {
    let mut into = vec![0; room];
    let mut count = 0;
    assert_eq!(files.read(file, &mut into, Some(&mut count)), 1);
    into.truncate(count as usize);
    into
}

/// Runs `run` and checks its lines are `calls`, in order, each answered:
/// falsifier 1 over a whole stretch.
fn as_a1(calls: &[&str], run: impl FnOnce()) {
    let lines = debug_lines(run);
    let named: Vec<&str> = lines.iter().filter_map(|line| field(line, "c")).collect();
    assert_eq!(lines.len(), calls.len(), "{lines:#?}");
    assert_eq!(named, calls, "{lines:#?}");
    assert!(
        lines.iter().all(|line| field(line, "e") == Some("0")),
        "{lines:#?}"
    );
}

/// a1's sweep (lines 1 to 86): no merc folder
/// (error 3); `USERSTAR.BWD` deleted by its upper-case name; a find of
/// `*.bwd` walked to its end (error 0x12), each name deleted as it is
/// found, in a1's order; `SHELLMER.CFG` none (error 2); the logos 0 to 2
/// deleted, 3 to 9 none (error 2).
#[test]
fn a1s_sweep_deletes_each_save_its_find_walks() {
    const SAVES: [&CStr; 15] = [
        c"en01star.bwd",
        c"en02star.bwd",
        c"en03star.bwd",
        c"en04star.bwd",
        c"en05star.bwd",
        c"mis5are1.bwd",
        c"mis5are2.bwd",
        c"ran0brf1.bwd",
        c"ran0dbff.bwd",
        c"ran0dbfs.bwd",
        c"ran0scn1.bwd",
        c"randnav1.bwd",
        c"randnav2.bwd",
        c"randwld1.bwd",
        c"rnd0cam1.bwd",
    ];
    const LOGOS: [&CStr; 3] = [c"mw2logo0.cel", c"mw2logo1.cel", c"mw2logo2.cel"];
    let mut folder = MemoryFolder::default()
        .give("userstar.bwd", FILL)
        .give("MW2DIF.CFG", FILL);
    for name in SAVES.iter().chain(&LOGOS) {
        folder = folder.give(text(name), FILL);
    }
    let files = Files::new(&SHELL, folder);
    let (calls, errors) = (SHELL.calls, SHELL.errors);
    let mut data = [0; FIND_BYTES];
    let find = logged(&files, calls.find_first, errors.no_path, || {
        files.find_first(c"mek\\?????mer.mek", &mut data)
    });
    assert_eq!(find, SHELL.invalid);
    assert_eq!(
        logged(&files, calls.delete, 0, || files.delete(c"USERSTAR.BWD")),
        1
    );
    let find = logged(&files, calls.find_first, 0, || {
        files.find_first(c"*.bwd", &mut data)
    });
    assert_ne!(find, SHELL.invalid);
    for (at, &save) in SAVES.iter().enumerate() {
        assert_eq!(found(&data), save);
        assert_eq!(logged(&files, calls.delete, 0, || files.delete(save)), 1);
        let more = at + 1 < SAVES.len();
        let error = if more { 0 } else { errors.no_more };
        let next = logged(&files, calls.find_next, error, || {
            files.find_next(find, &mut data)
        });
        assert_eq!(next, i32::from(more));
    }
    assert_eq!(found(&data), c"");
    assert_eq!(
        logged(&files, calls.find_close, 0, || files.find_close(find)),
        1
    );
    assert_eq!(
        logged(&files, calls.delete, errors.not_found, || files
            .delete(c"SHELLMER.CFG")),
        0
    );
    for logo in 0..10 {
        let name = CString::new(format!("mw2logo{logo}.cel")).expect("invariant: no NUL");
        let (done, error) = if logo < 3 {
            (1, 0)
        } else {
            (0, errors.not_found)
        };
        assert_eq!(
            logged(&files, calls.delete, error, || files.delete(&name)),
            done
        );
    }
    for name in SAVES.iter().chain(&LOGOS).chain(&[c"userstar.bwd"]) {
        assert_eq!(files.folder().contents(text(name)), None, "{name:?}");
    }
    assert_eq!(files.folder().contents("MW2DIF.CFG").as_deref(), Some(FILL));
}

/// a1's saves (lines 122 to 132 and their six kin): made anew read-write,
/// four seeks to where it stands, the body, the 8-byte head at 4, a seek
/// to 2, closed. Each name's body count is a1's.
#[test]
fn a1s_saves_write_the_body_then_its_head_at_4() {
    const SAVES: [(&CStr, u32); 7] = [
        (c"ran0wld1.bwd", 0x2ec),
        (c"randnav1.bwd", 0x84),
        (c"en01star.bwd", 0xfc),
        (c"ran0brf1.bwd", 0x300),
        (c"ran0dbff.bwd", 0xa8),
        (c"ran0dbfs.bwd", 0xbc),
        (c"rnd0cam1.bwd", 0x364),
    ];
    let c = SHELL.calls;
    let [start, here, _] = SHELL.methods;
    let calls = [
        c.open, c.kind, c.seek, c.seek, c.seek, c.seek, c.write, c.seek, c.write, c.seek, c.close,
    ];
    let head = fill(8);
    for (name, body) in SAVES {
        let files = Files::new(&SHELL, MemoryFolder::default());
        as_a1(&calls, || {
            let file = open(&files, name, access(true, true), Create::Always);
            assert_eq!(files.kind(file), SHELL.kinds[0]);
            for _ in 0..4 {
                assert_eq!(files.seek(file, 0, here), 0);
            }
            assert_eq!(written(&files, file, &fill(body)), body);
            assert_eq!(files.seek(file, 4, start), 4);
            assert_eq!(written(&files, file, &head), 8);
            assert_eq!(files.seek(file, 2, start), 2);
            assert_eq!(files.close(file), 1);
        });
        let mut saved = fill(body);
        saved[4..12].copy_from_slice(&head);
        assert_eq!(files.folder().contents(text(name)), Some(saved), "{name:?}");
    }
}

/// a1's copies (lines 111 to 121 and 157 to 167): the source opened read-write,
/// the copy made write-only, both set to 0, the source read 0x8000 then
/// 0x7000 (0 left), closed, the copy written whole and closed.
#[test]
fn a1s_copies_read_to_the_end_then_write_it_whole() {
    const COPIES: [(&CStr, &CStr, u32); 2] = [
        (c"rmg\\drpshp3.bwd", c"mis7are1.bwd", 0x1f4),
        (c"rmg\\mis7scn1.bwd", c"ran0scn1.bwd", 0xb54),
    ];
    let c = SHELL.calls;
    let start = SHELL.methods[0];
    let calls = [
        c.open, c.kind, c.open, c.kind, c.seek, c.seek, c.read, c.read, c.close, c.write, c.close,
    ];
    for (from, to, count) in COPIES {
        let files = Files::new(&SHELL, MemoryFolder::default().give(text(from), &fill(count)));
        as_a1(&calls, || {
            let source = open(&files, from, access(true, true), Create::Existing);
            assert_eq!(files.kind(source), SHELL.kinds[0]);
            let copy = open(&files, to, access(false, true), Create::Always);
            assert_eq!(files.kind(copy), SHELL.kinds[0]);
            assert_eq!(files.seek(source, 0, start), 0);
            assert_eq!(files.seek(copy, 0, start), 0);
            let bytes = read(&files, source, 0x8000);
            assert!(read(&files, source, 0x7000).is_empty());
            assert_eq!(files.close(source), 1);
            assert_eq!(written(&files, copy, &bytes), count);
            assert_eq!(files.close(copy), 1);
        });
        assert_eq!(files.folder().contents(text(to)), Some(fill(count)));
        assert_eq!(files.folder().contents(text(from)), Some(fill(count)));
    }
}

/// a1's records (lines 241 to 372): `userstar.bwd` made and written 0x98
/// three times; then 15 rounds, each reading `MW2DIF.CFG`'s 0x17 bytes
/// whole (0x1000 asked) and making the next of the five stars anew at
/// 0xfc. A make empties: each file ends one write long, the settings
/// as given.
#[test]
fn a1s_records_make_each_file_anew_and_leave_the_settings_be() {
    const STARS: [&CStr; 5] = [
        c"en01star.bwd",
        c"en02star.bwd",
        c"en03star.bwd",
        c"en04star.bwd",
        c"en05star.bwd",
    ];
    let c = SHELL.calls;
    let settings = fill(0x17);
    let files = Files::new(&SHELL, MemoryFolder::default().give("MW2DIF.CFG", &settings));
    let mut calls = [c.open, c.kind, c.write, c.close].repeat(3);
    for _ in 0..15 {
        calls.extend([
            c.open, c.kind, c.read, c.close, c.open, c.kind, c.write, c.close,
        ]);
    }
    as_a1(&calls, || {
        for _ in 0..3 {
            let file = open(&files, c"userstar.bwd", access(false, true), Create::Always);
            assert_eq!(files.kind(file), SHELL.kinds[0]);
            assert_eq!(written(&files, file, &fill(0x98)), 0x98);
            assert_eq!(files.close(file), 1);
        }
        for &star in STARS.iter().cycle().take(15) {
            let file = open(&files, c"MW2DIF.CFG", access(true, false), Create::Existing);
            assert_eq!(files.kind(file), SHELL.kinds[0]);
            assert_eq!(read(&files, file, 0x1000), settings);
            assert_eq!(files.close(file), 1);
            let file = open(&files, star, access(false, true), Create::Always);
            assert_eq!(files.kind(file), SHELL.kinds[0]);
            assert_eq!(written(&files, file, &fill(0xfc)), 0xfc);
            assert_eq!(files.close(file), 1);
        }
    });
    assert_eq!(files.folder().contents("userstar.bwd"), Some(fill(0x98)));
    for star in STARS {
        assert_eq!(files.folder().contents(text(star)), Some(fill(0xfc)));
    }
    assert_eq!(files.folder().contents("MW2DIF.CFG"), Some(settings));
}

/// a1's log and logos (lines 108 to 110 and 214 to 233): `rmglog.txt` made
/// and set to 0, held open while the two logos are made, each written
/// 0x400 four times and then 2; the log written 0x1df last, and closed.
/// Writes in a row follow on.
#[test]
fn a1s_log_stays_open_while_the_logos_are_written() {
    const LOGOS: [&CStr; 2] = [c"MW2LOGO0.cel", c"MW2LOGO1.cel"];
    let c = SHELL.calls;
    let files = Files::new(&SHELL, MemoryFolder::default());
    let mut calls = vec![c.open, c.kind, c.seek];
    for _ in LOGOS {
        calls.extend([
            c.open, c.kind, c.write, c.write, c.write, c.write, c.write, c.close,
        ]);
    }
    calls.extend([c.write, c.close]);
    as_a1(&calls, || {
        let log = open(&files, c"rmglog.txt", access(false, true), Create::Always);
        assert_eq!(files.kind(log), SHELL.kinds[0]);
        assert_eq!(files.seek(log, 0, SHELL.methods[0]), 0);
        for logo in LOGOS {
            let file = open(&files, logo, access(false, true), Create::Always);
            assert_eq!(files.kind(file), SHELL.kinds[0]);
            for _ in 0..4 {
                assert_eq!(written(&files, file, &fill(0x400)), 0x400);
            }
            assert_eq!(written(&files, file, &fill(2)), 2);
            assert_eq!(files.close(file), 1);
        }
        assert_eq!(written(&files, log, &fill(0x1df)), 0x1df);
        assert_eq!(files.close(log), 1);
    });
    let mut logo = fill(0x400).repeat(4);
    logo.extend(fill(2));
    for name in LOGOS {
        assert_eq!(files.folder().contents(text(name)), Some(logo.clone()));
    }
    assert_eq!(files.folder().contents("rmglog.txt"), Some(fill(0x1df)));
}

/// a1's move (lines 122 to 132, 234 and 240): `ran0wld1.bwd` made and
/// written 0x2ec; the delete of `randwld1.bwd` refused (the sweep took it,
/// error 2); the move onto that name.
#[test]
fn a1s_world_moves_onto_the_name_its_sweep_freed() {
    let files = Files::new(&SHELL, MemoryFolder::default());
    let file = open(&files, c"ran0wld1.bwd", access(true, true), Create::Always);
    assert_eq!(written(&files, file, &fill(0x2ec)), 0x2ec);
    assert_eq!(files.close(file), 1);
    let error = SHELL.errors.not_found;
    assert_eq!(
        logged(&files, SHELL.calls.delete, error, || files
            .delete(c"randwld1.bwd")),
        0
    );
    let moved = logged(&files, SHELL.calls.rename, 0, || {
        files.rename(c"ran0wld1.bwd", c"randwld1.bwd")
    });
    assert_eq!(moved, 1);
    assert_eq!(files.folder().contents("ran0wld1.bwd"), None);
    assert_eq!(files.folder().contents("randwld1.bwd"), Some(fill(0x2ec)));
}
