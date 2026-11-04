// SPDX-License-Identifier: MIT
//! The Kernel's time (step 3, rulings 1389 and 1398): the 8 KERNEL32 time
//! names the four files import, written once. Every name that measures an
//! interval reads one game clock, [`clock::Clock`], handed in and moved
//! only by its owner: the frame loop on a page, a back on a host, a test.
//! The date names read a wall clock, handed in the same way. No front
//! reads the OS.
//!
//! Each file the seam serves is a [`table::Row`], kept in its game's rows
//! crate; where the clocks come from is a back behind [`Clocks`]: the
//! page's, [`page::Page`]; a host's, `host::Host`; the original's calls
//! under Wine, `mech-layer`'s `time.rs`. [`Times`] answers in the file's
//! numbers and writes one `imp.call` line a call: `c` the row's name, `a`
//! the call's numbers, `v` its answer, `t` the instant read (the wall's
//! ticks, or the game clock's microseconds), `b` the bias in force in
//! minutes, `w` the 8 date words written, `z` the zone record written,
//! whole, `f` the file time written, `e` the error left.
//!
//! The front answers only what the callers read back (wall 1, `RESULT.md`):
//! a date's 8 words, the counter's two halves, every field of the zone
//! record. The counter's rate is the clock's, 1 MHz: its one reader
//! compares counts with each other alone (wall 5).
//!
//! The crate names no game and nothing of any one target, so it builds
//! for `wasm32-unknown-unknown`; a game's numbers live in its rows crate.
#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod clock;
pub mod date;
#[cfg(all(any(test, feature = "std"), unix, target_pointer_width = "64"))]
pub mod host;
pub mod page;
pub mod table;
#[cfg(test)]
mod tests;

use core::fmt;
use date::Date;
use mech_trace::Args;
use table::{DATE_BYTES, Row, ZONE_BYTES};

/// File-time ticks a minute of bias.
const TICKS_PER_MINUTE: i64 = 600_000_000;

/// Which of the zone's two times is in force.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InForce {
    Unknown,
    Standard,
    Daylight,
}

/// A time zone, as a `TIME_ZONE_INFORMATION` carries it: every field is
/// read by the shell's C library (wall 1). Biases are minutes west of UTC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zone {
    pub bias: i32,
    pub standard_name: [u16; 32],
    pub standard_date: Date,
    pub standard_bias: i32,
    pub daylight_name: [u16; 32],
    pub daylight_date: Date,
    pub daylight_bias: i32,
}

impl Zone {
    /// UTC: no bias, no names, no change.
    pub const UTC: Self = Self {
        bias: 0,
        standard_name: [0; 32],
        standard_date: Date([0; 8]),
        standard_bias: 0,
        daylight_name: [0; 32],
        daylight_date: Date([0; 8]),
        daylight_bias: 0,
    };

    /// The bias in force, in minutes.
    #[must_use]
    pub const fn bias_in(&self, in_force: InForce) -> i32 {
        self.bias
            + match in_force {
                InForce::Unknown => 0,
                InForce::Standard => self.standard_bias,
                InForce::Daylight => self.daylight_bias,
            }
    }

    /// The record at Windows' offsets.
    pub fn write(&self, into: &mut [u8; ZONE_BYTES]) {
        into[0x0..0x4].copy_from_slice(&self.bias.to_le_bytes());
        names(&self.standard_name, &mut into[0x4..0x44]);
        into[0x44..0x54].copy_from_slice(&self.standard_date.to_bytes());
        into[0x54..0x58].copy_from_slice(&self.standard_bias.to_le_bytes());
        names(&self.daylight_name, &mut into[0x58..0x98]);
        into[0x98..0xa8].copy_from_slice(&self.daylight_date.to_bytes());
        into[0xa8..0xac].copy_from_slice(&self.daylight_bias.to_le_bytes());
    }

    /// The record back from Windows' offsets.
    #[must_use]
    pub fn read(from: &[u8; ZONE_BYTES]) -> Self {
        let bias =
            |at: usize| i32::from_le_bytes([from[at], from[at + 1], from[at + 2], from[at + 3]]);
        let name = |at: usize| {
            let mut name = [0; 32];
            for (word, from) in name.iter_mut().zip(from[at..at + 64].chunks_exact(2)) {
                *word = u16::from_le_bytes([from[0], from[1]]);
            }
            name
        };
        let date = |at: usize| {
            let mut bytes = [0; DATE_BYTES];
            bytes.copy_from_slice(&from[at..at + DATE_BYTES]);
            Date::from_bytes(&bytes)
        };
        Self {
            bias: bias(0x0),
            standard_name: name(0x4),
            standard_date: date(0x44),
            standard_bias: bias(0x54),
            daylight_name: name(0x58),
            daylight_date: date(0x98),
            daylight_bias: bias(0xa8),
        }
    }
}

fn names(name: &[u16; 32], into: &mut [u8]) {
    for (word, into) in name.iter().zip(into.chunks_exact_mut(2)) {
        into.copy_from_slice(&word.to_le_bytes());
    }
}

/// Where one file's clocks come from: the seam between the calls and a
/// target.
pub trait Clocks {
    /// The game clock's microseconds; a host's back moves it a frame first.
    fn micros(&self) -> u64;
    /// The wall clock: file-time ticks, UTC.
    fn wall(&self) -> u64;
    /// The zone, and which of its times is in force at wall ticks `at`.
    fn zone(&self, at: u64) -> (Zone, InForce);
    /// Waits `millis`; a back that cannot block returns at once.
    fn sleep(&self, millis: u32);
    /// Keeps `code` as the last error the file reads after a failure.
    fn fail(&self, code: u32);
    /// Writes one line around whatever the target must keep across it.
    fn quiet(&self, line: impl FnOnce()) {
        line();
    }
}

/// Bytes as the line shows them: two hex digits each, whole.
pub struct Hex<'a>(pub &'a [u8]);

impl fmt::Display for Hex<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

/// One file's time calls: its row, and its clocks.
pub struct Times<C> {
    file: &'static Row,
    clocks: C,
}

impl<C: Clocks> Times<C> {
    pub const fn new(file: &'static Row, clocks: C) -> Self {
        Self { file, clocks }
    }

    /// The clocks the calls ask.
    pub const fn clocks(&self) -> &C {
        &self.clocks
    }

    /// `GetTickCount`: the game clock's whole milliseconds, wrapping at
    /// 32 bits. xvt alone imports it; its harvest is parked (1168).
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        reason = "the count wraps at 32 bits, as Windows' does"
    )]
    pub fn tick_count(&self) -> u32 {
        let micros = self.clocks.micros();
        let value = (micros / 1000) as u32;
        self.clocks.quiet(|| {
            log::debug!(
                c = self.file.calls.tick_count, v:% = mech_trace::__shown!(value),
                t:% = mech_trace::__shown!(micros); "imp.call"
            );
        });
        value
    }

    /// `Sleep`: the back waits, or returns at once on the page.
    pub fn sleep(&self, millis: u32) {
        self.clocks.sleep(millis);
        let arguments = [mech_trace::__shown!(millis)];
        self.clocks.quiet(|| {
            log::debug!(c = self.file.calls.sleep, a:% = Args(arguments); "imp.call");
        });
    }

    /// `QueryPerformanceCounter`: the game clock's microseconds into
    /// `into`, both halves; 1.
    #[allow(
        clippy::cast_possible_wrap,
        reason = "the counter is Windows' signed 64-bit count"
    )]
    pub fn counter(&self, into: &mut i64) -> i32 {
        let micros = self.clocks.micros();
        *into = micros as i64;
        self.clocks.quiet(|| {
            log::debug!(
                c = self.file.calls.counter, v:% = mech_trace::__shown!(1),
                t:% = mech_trace::__shown!(micros); "imp.call"
            );
        });
        1
    }

    /// `GetSystemTime`: the wall clock's UTC date into `into`.
    pub fn system_time(&self, into: &mut [u8; DATE_BYTES]) {
        let wall = self.clocks.wall();
        let date = Date::of(wall);
        *into = date.to_bytes();
        self.dated(self.file.calls.system_time, wall, None, &date);
    }

    /// `GetLocalTime`: the wall clock less the bias in force, as a date.
    pub fn local_time(&self, into: &mut [u8; DATE_BYTES]) {
        let wall = self.clocks.wall();
        let (zone, in_force) = self.clocks.zone(wall);
        let bias = zone.bias_in(in_force);
        let date = Date::of(local(wall, bias));
        *into = date.to_bytes();
        self.dated(self.file.calls.local_time, wall, Some(bias), &date);
    }

    /// `GetTimeZoneInformation`: the record into `into`; the row's id for
    /// the time in force.
    pub fn zone(&self, into: &mut [u8; ZONE_BYTES]) -> u32 {
        let wall = self.clocks.wall();
        let (zone, in_force) = self.clocks.zone(wall);
        zone.write(into);
        let value = self.file.ids[in_force as usize];
        let record = &*into;
        self.clocks.quiet(|| {
            log::debug!(
                c = self.file.calls.zone, v:% = mech_trace::__shown!(value),
                t:% = mech_trace::__shown!(wall), z:% = Hex(record); "imp.call"
            );
        });
        value
    }

    /// `FileTimeToLocalFileTime`: `ticks` less now's bias, as Windows
    /// does, into `into`; 1.
    pub fn to_local_file_time(&self, ticks: u64, into: &mut u64) -> i32 {
        let wall = self.clocks.wall();
        let (zone, in_force) = self.clocks.zone(wall);
        let bias = zone.bias_in(in_force);
        let value = local(ticks, bias);
        *into = value;
        let arguments = [mech_trace::__shown!(ticks)];
        self.clocks.quiet(|| {
            log::debug!(
                c = self.file.calls.to_local_file_time, a:% = Args(arguments),
                v:% = mech_trace::__shown!(1), b:% = mech_trace::__shown!(bias),
                f:% = mech_trace::__shown!(value); "imp.call"
            );
        });
        1
    }

    /// `FileTimeToSystemTime`: the date of `ticks` into `into`, and 1; a
    /// time at or past 2^63 is refused, 0 and the row's bad argument
    /// (Windows' documented refusal; its code unmeasured).
    pub fn to_system_time(&self, ticks: u64, into: &mut [u8; DATE_BYTES]) -> i32 {
        let (value, error, date) = if ticks >> 63 == 0 {
            let date = Date::of(ticks);
            *into = date.to_bytes();
            (1, 0, date)
        } else {
            self.clocks.fail(self.file.bad_argument);
            (0, self.file.bad_argument, Date::default())
        };
        let arguments = [mech_trace::__shown!(ticks)];
        let words = date.0;
        self.clocks.quiet(|| {
            log::debug!(
                c = self.file.calls.to_system_time, a:% = Args(arguments),
                v:% = mech_trace::__shown!(value),
                w:% = Args(words.each_ref().map(|word| mech_trace::__shown!(*word))),
                e:% = mech_trace::__shown!(error); "imp.call"
            );
        });
        value
    }

    /// A date call's line.
    fn dated(&self, name: &'static str, wall: u64, bias: Option<i32>, date: &Date) {
        let words = date.0;
        let shown = words.each_ref().map(|word| mech_trace::__shown!(*word));
        self.clocks.quiet(|| match bias {
            Some(bias) => log::debug!(
                c = name, t:% = mech_trace::__shown!(wall), b:% = mech_trace::__shown!(bias),
                w:% = Args(shown); "imp.call"
            ),
            None => log::debug!(
                c = name, t:% = mech_trace::__shown!(wall), w:% = Args(shown); "imp.call"
            ),
        });
    }
}

/// File-time `ticks` less `bias` minutes: UTC is local plus the bias.
fn local(ticks: u64, bias: i32) -> u64 {
    ticks.wrapping_add_signed(-i64::from(bias) * TICKS_PER_MINUTE)
}
