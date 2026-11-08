// SPDX-License-Identifier: MIT
//! Each `input.key` row checked against the port's own DEBUG rows. The port's
//! source, `src/fun_10070381.rs`, is compiled into this test whole; its four
//! globals and three callees are the test's stand-ins, so the presses are the
//! test's and no game file runs. Each call's `input.key` is checked against the
//! `fn.in` and `mem.wr` rows the same call wrote: `vk` is the entry's argument,
//! `act` the value the queue's write wrote, `none` where no write ran. No
//! expected value is worked out by hand.
#![cfg(target_arch = "x86")]
#![allow(
    unsafe_code,
    reason = "the port's globals and callees are linked by name"
)]

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, PoisonError};

#[path = "../src/fun_10070381.rs"]
mod port;

/// The game's key table, a `u16` code a virtual key.
#[unsafe(no_mangle)]
static mut DAT_100b4438: [u16; 0x100] = [0; 0x100];
/// The queue's read index.
#[unsafe(no_mangle)]
static mut DAT_100c7fd8: u32 = 0;
/// The queue's write index.
#[unsafe(no_mangle)]
static mut DAT_100c7fdc: u32 = 0;
/// The queue, 64 `u16` keys.
#[unsafe(no_mangle)]
static mut DAT_100eedb0: [u16; 0x40] = [0; 0x40];

/// `MapVirtualKeyA`'s answer to the next call.
static MAPPED: AtomicU32 = AtomicU32::new(0);
/// The keys `GetKeyState` answers held: 1 Ctrl, 2 Alt, 4 Shift.
static HELD: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
extern "stdcall" fn MapVirtualKeyA(_code: u32, _kind: u32) -> u32 {
    MAPPED.load(Ordering::Relaxed)
}

/// A held key answers `0xff80`, the input front's answer for one held
/// (`crates/mech-rows/src/input/tests.rs`), which sets the `0x1000` the port
/// tests; a key not held answers 0.
#[unsafe(no_mangle)]
extern "stdcall" fn GetKeyState(key: i32) -> i16 {
    let bit = match key {
        0x11 => 1,
        0x12 => 2,
        0x10 => 4,
        _ => 0,
    };
    if HELD.load(Ordering::Relaxed) & bit == 0 {
        0
    } else {
        -0x80
    }
}

#[unsafe(no_mangle)]
extern "C" fn _tolower(c: i32) -> i32 {
    if (0x41..=0x5a).contains(&c) {
        c + 0x20
    } else {
        c
    }
}

/// Every row logged, as its line's body (`event k=v ...`).
static ROWS: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Capture;

impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        ROWS.lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(mech_logline::body(record));
    }

    fn flush(&self) {}
}

static CAPTURE: Capture = Capture;

/// Calls the port with the virtual key `code`; returns the rows the call wrote.
fn release(code: u32) -> Vec<String> {
    ROWS.lock().unwrap_or_else(PoisonError::into_inner).clear();
    // SAFETY: the port reads and writes the stand-in globals above, sized for
    // a key below 0x100 and the queue's 64 slots; its callees are the
    // stand-ins; one thread runs it.
    unsafe { port::FUN_10070381(code) };
    std::mem::take(&mut *ROWS.lock().unwrap_or_else(PoisonError::into_inner))
}

/// The value of field `key` in the row `row`.
fn value<'r>(row: &'r str, key: &str) -> Option<&'r str> {
    row.split(' ')
        .skip(1)
        .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
}

/// The rows of `event` among `rows`.
fn of<'r>(rows: &'r [String], event: &str) -> Vec<&'r str> {
    rows.iter()
        .map(String::as_str)
        .filter(|row| row.split(' ').next() == Some(event))
        .collect()
}

#[test]
fn each_input_key_row_names_the_key_handed_and_the_key_queued() {
    assert!(
        log::set_logger(&CAPTURE).is_ok(),
        "a logger was already set"
    );
    log::set_max_level(log::LevelFilter::Debug);
    // SAFETY: one thread writes the stand-in table before any call reads it.
    unsafe { DAT_100b4438[0x41] = 0x61 };
    // The test's presses: (virtual key, `MapVirtualKeyA`'s answer, held).
    let presses = [
        (0x41, 0, 0),    // the table's code, nothing held
        (0x42, 0x42, 1), // the table's 0, MapVirtualKeyA's code, Ctrl held
        (0x7f, 0, 0),    // the table's 0 and MapVirtualKeyA's 0
        (0x41, 0, 6),    // the table's code, Alt and Shift held
    ];
    let mut branches = [0; 2];
    let mut harvest = Vec::new();
    for (key, mapped, held) in presses {
        MAPPED.store(mapped, Ordering::Relaxed);
        HELD.store(held, Ordering::Relaxed);
        let rows = release(key);
        harvest.extend(rows.iter().cloned());
        let keys = of(&rows, "input.key");
        let entries = of(&rows, "fn.in");
        let [row] = keys.as_slice() else {
            panic!("one input.key a call: {rows:?}")
        };
        let [entry] = entries.as_slice() else {
            panic!("one fn.in a call: {rows:?}")
        };
        assert_eq!(value(row, "vk"), value(entry, "a"), "{rows:?}");
        assert_eq!(
            rows.last().map(String::as_str),
            Some(*row),
            "the call's last row: {rows:?}"
        );
        match of(&rows, "mem.wr").as_slice() {
            [] => {
                assert_eq!(value(row, "act"), Some("none"), "{rows:?}");
                branches[0] += 1;
            }
            [write] => {
                assert_eq!(value(row, "act"), value(write, "v"), "{rows:?}");
                branches[1] += 1;
            }
            _ => panic!("one queue write at most a call: {rows:?}"),
        }
    }
    assert!(
        branches.iter().all(|&ran| ran > 0),
        "both branches ran: {branches:?}"
    );
    // The reader's fixture (`readers/key_story.py`): every row of the four
    // calls, one body a line, to the file `MECH_GAME_KEY_LOG` names; unset, none.
    if let Some(path) = std::env::var_os("MECH_GAME_KEY_LOG") {
        let mut text = harvest.join("\n");
        text.push('\n');
        if let Err(error) = std::fs::write(&path, text) {
            panic!("the fixture {path:?} was not written: {error}");
        }
    }
}
