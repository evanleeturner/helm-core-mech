// SPDX-License-Identifier: MIT
//! A host's clocks, Linux's and macOS's, through `std` and the C library:
//! the game clock moved a frame at each read from the host's own instant,
//! the wall clock `SystemTime`, `Sleep` a real wait. `std` has no time zone
//! (wall 3), so the zone is the C library's own, `localtime_r` after one
//! `tzset`: the offset in force and the daylight flag at the instant asked,
//! and the year's two changes found by probing it a day, then a second, at
//! a time. Each change is written as a zone record writes its dates: month,
//! week of the month (5 the last), weekday, and the clock before the change
//! plus 1 s. The names are the C library's short ones (`CST`), not
//! Windows' long ones.
//!
//! A year with no change gives a zone with no dates, its time unknown, as
//! Windows' is. A year whose standard offset itself moves is read as its
//! daylight change's clock before: unmeasured.
#![allow(unsafe_code, reason = "the C library's tzset and localtime_r")]

use crate::clock::Clock;
use crate::date::{self, Date};
use crate::{Clocks, InForce, Zone};
use std::ffi::{CStr, c_char, c_int, c_long};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, Once, PoisonError};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

#[cfg(test)]
mod tests;

/// File-time ticks a second.
const TICKS_PER_SECOND: i128 = 10_000_000;

const SECONDS_PER_DAY: i64 = 86_400;

/// The C library's `struct tm`, as glibc and macOS lay it out.
#[repr(C)]
struct Tm {
    sec: c_int,
    min: c_int,
    hour: c_int,
    mday: c_int,
    mon: c_int,
    year: c_int,
    wday: c_int,
    yday: c_int,
    isdst: c_int,
    gmtoff: c_long,
    zone: *const c_char,
}

unsafe extern "C" {
    fn tzset();
    fn localtime_r(time: *const i64, out: *mut Tm) -> *mut Tm;
}

static TZSET: Once = Once::new();

/// The local fields at `at`, seconds since 1970 UTC, by the C library.
fn local(at: i64) -> Tm {
    // SAFETY: reads the zone's rules once, before any localtime_r.
    TZSET.call_once(|| unsafe { tzset() });
    let mut out = Tm {
        sec: 0,
        min: 0,
        hour: 0,
        mday: 0,
        mon: 0,
        year: 0,
        wday: 0,
        yday: 0,
        isdst: 0,
        gmtoff: 0,
        zone: std::ptr::null(),
    };
    // SAFETY: one instant read, one struct the call fills.
    let answer = unsafe { localtime_r(&raw const at, &raw mut out) };
    if answer.is_null() {
        out.zone = std::ptr::null();
    }
    out
}

/// The zone's short name, as a record's 32 words, 0 ended.
fn name(tm: &Tm) -> [u16; 32] {
    let mut name = [0; 32];
    if !tm.zone.is_null() {
        // SAFETY: the C library's own zone name, a C string it keeps.
        let text = unsafe { CStr::from_ptr(tm.zone) };
        for (word, byte) in name[..31].iter_mut().zip(text.to_bytes()) {
            *word = u16::from(*byte);
        }
    }
    name
}

/// The bias of an offset east of UTC, in minutes west.
fn bias(gmtoff: c_long) -> i32 {
    i32::try_from(-gmtoff / 60).unwrap_or(0)
}

/// Seconds since 1970 UTC at 00:00 on 1 January of `year`.
fn year_start(year: i64) -> i64 {
    (Date::days(year, 1, 1) - Date::days(1970, 1, 1)) * SECONDS_PER_DAY
}

/// The first second at which the offset differs from the one at `from`.
fn change(from: i64, to: i64) -> i64 {
    let (mut low, mut high) = (from, to);
    let offset = local(from).gmtoff;
    while high - low > 1 {
        let middle = low + (high - low) / 2;
        if local(middle).gmtoff == offset {
            low = middle;
        } else {
            high = middle;
        }
    }
    high
}

/// The change at `at` as a record's date: the clock before it plus 1 s.
fn rule(at: i64, before: &Tm) -> Date {
    let clock = i128::from(at) + i128::from(before.gmtoff);
    let ticks = i128::from(date::UNIX_EPOCH) + clock * TICKS_PER_SECOND;
    let words = Date::of(u64::try_from(ticks).unwrap_or(0)).0;
    let [year, month, weekday, day, hour, minute, second, _] = words.map(i64::from);
    let next = if month == 12 {
        Date::days(year + 1, 1, 1)
    } else {
        Date::days(year, month + 1, 1)
    };
    let week = if day + 7 > next - Date::days(year, month, 1) {
        5
    } else {
        (day - 1) / 7 + 1
    };
    let word = |value: i64| u16::try_from(value).unwrap_or(0);
    Date([
        0,
        word(month),
        word(weekday),
        word(week),
        word(hour),
        word(minute),
        word(second),
        0,
    ])
}

/// The zone of the year holding `at`, seconds since 1970 UTC.
fn zone_at(at: i64) -> (Zone, InForce) {
    let here = local(at);
    let year = i64::from(here.year) + 1900;
    let (mut standard, mut daylight) = (None, None);
    let end = year_start(year + 1);
    let mut day = year_start(year);
    while day < end {
        let next = day + SECONDS_PER_DAY;
        if local(next).gmtoff != local(day).gmtoff {
            let at = change(day, next);
            let before = local(at - 1);
            let after = local(at);
            let date = rule(at, &before);
            if after.isdst > 0 {
                daylight = Some((date, before, after));
            } else {
                standard = Some(date);
            }
        }
        day = next;
    }
    let Some((daylight_date, before, after)) = daylight else {
        let zone = Zone {
            bias: bias(here.gmtoff),
            standard_name: name(&here),
            ..Zone::UTC
        };
        return (zone, InForce::Unknown);
    };
    let zone = Zone {
        bias: bias(before.gmtoff),
        standard_name: name(&before),
        standard_date: standard.unwrap_or_default(),
        standard_bias: 0,
        daylight_name: name(&after),
        daylight_date,
        daylight_bias: bias(after.gmtoff) - bias(before.gmtoff),
    };
    let in_force = if here.isdst > 0 {
        InForce::Daylight
    } else {
        InForce::Standard
    };
    (zone, in_force)
}

/// File-time ticks of `span`.
fn ticks(span: Duration) -> u64 {
    u64::try_from(span.as_nanos() / 100).unwrap_or(u64::MAX)
}

/// The clocks of the host the program runs on.
pub struct Host {
    start: Instant,
    clock: Mutex<Clock>,
    error: AtomicU32,
}

impl Host {
    /// A game clock at 0 now, whose frames move it at most `clamp`
    /// microseconds each.
    #[must_use]
    pub fn new(clamp: u64) -> Self {
        let mut clock = Clock::new(clamp, 0);
        clock.frame(0);
        Self {
            start: Instant::now(),
            clock: Mutex::new(clock),
            error: AtomicU32::new(0),
        }
    }

    /// The last error kept.
    #[must_use]
    pub fn last_error(&self) -> u32 {
        self.error.load(Ordering::Relaxed)
    }
}

impl Clocks for Host {
    /// The game clock, a frame moved at the host's instant first.
    fn micros(&self) -> u64 {
        let host = u64::try_from(self.start.elapsed().as_micros()).unwrap_or(u64::MAX);
        let mut clock = self.clock.lock().unwrap_or_else(PoisonError::into_inner);
        clock.frame(host);
        clock.micros()
    }

    fn wall(&self) -> u64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(after) => date::UNIX_EPOCH.saturating_add(ticks(after)),
            Err(before) => date::UNIX_EPOCH.saturating_sub(ticks(before.duration())),
        }
    }

    fn zone(&self, at: u64) -> (Zone, InForce) {
        let seconds = (i128::from(at) - i128::from(date::UNIX_EPOCH)).div_euclid(TICKS_PER_SECOND);
        zone_at(i64::try_from(seconds).unwrap_or(0))
    }

    fn sleep(&self, millis: u32) {
        thread::sleep(Duration::from_millis(u64::from(millis)));
    }

    fn fail(&self, code: u32) {
        self.error.store(code, Ordering::Relaxed);
    }
}
