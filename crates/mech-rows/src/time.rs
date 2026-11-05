// SPDX-License-Identifier: MIT
//! mercs's time rows (KT1, ruling 1398): the shell's time calls, as
//! `kernel_time`'s seam serves them. Wall 5 read mercs's clock: whole
//! milliseconds everywhere, no tick cut, the counter compared only with
//! itself. The game's step and network play's step are unread: no
//! constant here stands for them.
use kernel_time::table::{Calls, Row};

#[cfg(test)]
mod tests;

/// The shell, `mw2shell`: its imports' names, its C library's `time` by the
/// name `ORDER.tsv` gives its stand-in, and the zone call's one answer,
/// unknown 0, as its C library reads it (1410).
pub const SHELL: Row = Row {
    file: "mw2shell",
    calls: Calls {
        tick_count: "KERNEL32!GetTickCount",
        sleep: "KERNEL32!Sleep",
        counter: "KERNEL32!QueryPerformanceCounter",
        system_time: "KERNEL32!GetSystemTime",
        local_time: "KERNEL32!GetLocalTime",
        zone: "KERNEL32!GetTimeZoneInformation",
        to_local_file_time: "KERNEL32!FileTimeToLocalFileTime",
        to_system_time: "KERNEL32!FileTimeToSystemTime",
        time: "_time",
    },
    zone_unknown: 0,
    bad_argument: 0x57,
};

/// The most one frame moves the game clock. mercs's own clamp is unread
/// (wall 5), never measured: none, so every gap counts whole, as Windows'
/// own clock moves. A choice, named in `RESULT.md`.
pub const CLAMP_MICROS: u64 = u64::MAX;
