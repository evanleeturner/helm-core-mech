// SPDX-License-Identifier: MIT
//! The game tests' shared kit, taken by `mod kit;`: a capture logger and the
//! readers of the rows it keeps, so a test checks a port against the DEBUG
//! rows the same call wrote. The stand-ins beside it are taken by `#[path]`,
//! each only by a test whose port calls it, so no test defines a symbol
//! twice; a stand-in that needs another names, in its doc, the module name
//! it is taken under.
#![allow(dead_code, reason = "each test takes the readers its port needs")]

use std::sync::{Mutex, Once, PoisonError};

/// Every row logged since the last take, as its line's body (`event k=v ...`).
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

fn take() -> Vec<String> {
    std::mem::take(&mut *ROWS.lock().unwrap_or_else(PoisonError::into_inner))
}

/// Runs `call` with the logger set and the rows cleared; returns its result
/// and the rows it wrote, in order. The rows and the stand-ins' records are
/// shared, so each test file holds one `#[test]` and one thread runs it.
pub fn rows<T>(call: impl FnOnce() -> T) -> (T, Vec<String>) {
    static SET: Once = Once::new();
    SET.call_once(|| {
        assert!(
            log::set_logger(&CAPTURE).is_ok(),
            "a logger was already set"
        );
        log::set_max_level(log::LevelFilter::Debug);
    });
    take();
    let result = call();
    (result, take())
}

/// The value of field `key` in `row`.
pub fn value<'r>(row: &'r str, key: &str) -> Option<&'r str> {
    row.split(' ')
        .skip(1)
        .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
}

/// The rows of `event` among `rows`.
pub fn of<'r>(rows: &'r [String], event: &str) -> Vec<&'r str> {
    rows.iter()
        .map(String::as_str)
        .filter(|row| row.split(' ').next() == Some(event))
        .collect()
}

/// The rows of `event` among `rows` whose field `key` is `want`.
pub fn with<'r>(rows: &'r [String], event: &str, key: &str, want: &str) -> Vec<&'r str> {
    of(rows, event)
        .into_iter()
        .filter(|row| value(row, key) == Some(want))
        .collect()
}

/// The one row of `event` among `rows`; panics, showing the rows, unless
/// there is exactly one.
pub fn one<'r>(rows: &'r [String], event: &str) -> &'r str {
    match of(rows, event).as_slice() {
        [row] => row,
        found => panic!("one {event} row, found {}: {rows:?}", found.len()),
    }
}

/// The value of `key` in `row`; panics, showing the row, when it is absent.
pub fn field<'r>(row: &'r str, key: &str) -> &'r str {
    value(row, key).unwrap_or_else(|| panic!("no {key} in {row}"))
}

/// Argument `at` (0 first) of a `fn.in` row: its `a` is comma-separated.
pub fn arg(row: &str, at: usize) -> &str {
    field(row, "a")
        .split(',')
        .nth(at)
        .unwrap_or_else(|| panic!("no argument {at} in {row}"))
}

/// A field's value as a number: the trace writes values in lowercase hex.
pub fn number(row: &str, key: &str) -> usize {
    let text = field(row, key);
    usize::from_str_radix(text, 16).unwrap_or_else(|_| panic!("{key}={text} is not hex in {row}"))
}

/// A number as the trace writes it, so a stand-in's record compares with a row.
pub fn shown(n: impl core::fmt::LowerHex) -> String {
    format!("{n:x}")
}

/// Writes `rows`, one body a line, to `<dir>/<name>.log` when the variable
/// `MECH_GAME_HARVEST` names a folder: the log reader's fixture. Unset, none.
pub fn harvest(name: &str, rows: &[String]) {
    let Some(dir) = std::env::var_os("MECH_GAME_HARVEST") else {
        return;
    };
    let path = std::path::Path::new(&dir).join(format!("{name}.log"));
    let mut text = rows.join("\n");
    text.push('\n');
    if let Err(error) = std::fs::write(&path, text) {
        panic!("the fixture {} was not written: {error}", path.display());
    }
}

/// One stand-in's calls since the last take, each its arguments.
pub struct Calls<T>(Mutex<Vec<T>>);

impl<T> Calls<T> {
    pub const fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }

    pub fn record(&self, call: T) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(call);
    }

    pub fn take(&self) -> Vec<T> {
        std::mem::take(&mut *self.0.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

/// An address as the game's 32-bit argument word, as a port's `i32`
/// parameter takes a pointer.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "the game tests build for i686 alone, where an address is 32 bits"
)]
pub fn word(at: usize) -> i32 {
    at as i32
}
