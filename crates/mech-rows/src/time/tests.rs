// SPDX-License-Identifier: MIT
//! Falsifier 3, the test harvested: `t1`'s one time call, played on the
//! page's clocks at `t1`'s wall (`records/2026-11-03-lane-c/
//! helm-kernel-time-kt1/kt1-s8-t1-time-lines.txt`). `t1`'s path calls the
//! C library's `time` once and the 8 KERNEL32 names 0 times
//! (`kt1-s9-f3.txt`). A number in `t1_time_is_its_line` no `t1` line holds
//! is a bug.
//!
//! KZ1's falsifier 2, the three zone fronts at three walls: the epoch,
//! `t1`'s, and a July instant inside Chicago's daylight time
//! (`kz1-july.txt`). No run reaches them (wall 1), so their answer is
//! 1410's, fixed, not harvested: UTC, whatever the wall.
use crate::test_log::debug_lines;
use crate::time::{CLAMP_MICROS, SHELL};
use kernel_time::Times;
use kernel_time::clock::Clock;
use kernel_time::date::UNIX_EPOCH;
use kernel_time::page::Page;
use kernel_time::table::ZONE_BYTES;

/// `t1`'s `_time` line, whole.
const T1_TIME: &str = "imp.call c=_time a=0 v=6aea8734 t=1dd6bf9fd2c271d";

/// `t1`'s wall at that call: the line's `t`.
const T1_WALL: u64 = 0x1dd_6bf9_fd2c_271d;

/// The three walls: the epoch, `t1`'s, and 2026-07-04 12:00 CDT
/// (`date -d @1783184400`, `kz1-july.txt`).
const WALLS: [u64; 3] = [UNIX_EPOCH, T1_WALL, UNIX_EPOCH + 1_783_184_400 * 10_000_000];

fn times(wall: u64) -> Times<Page> {
    Times::new(&SHELL, Page::new(Clock::new(CLAMP_MICROS, 0), wall))
}

#[test]
fn t1_time_is_its_line() {
    let times = times(T1_WALL);
    let mut value = 0;
    let lines = debug_lines(|| value = times.seconds(None));
    assert_eq!(lines, [T1_TIME]);
    assert_eq!(value, 0x6aea_8734);
}

/// `GetLocalTime` writes `GetSystemTime`'s 8 words, and its line is the
/// system line under its own name (wall 1: 1410's answer, not harvested).
#[test]
fn local_time_is_system_time() {
    for wall in WALLS {
        let times = times(wall);
        let (mut system, mut local) = ([0xaa; 16], [0x55; 16]);
        let system_lines = debug_lines(|| times.system_time(&mut system));
        let local_lines = debug_lines(|| times.local_time(&mut local));
        assert_eq!(local, system, "{wall:x}");
        let (from, to) = (SHELL.calls.system_time, SHELL.calls.local_time);
        let renamed: Vec<String> = system_lines
            .iter()
            .map(|line| line.replace(&format!("c={from}"), &format!("c={to}")))
            .collect();
        assert_eq!(local_lines, renamed, "{wall:x}");
        assert_eq!(local_lines.len(), 1, "{wall:x}");
    }
}

/// `FileTimeToLocalFileTime(t)` answers 1 and writes `t`: UTC is its own
/// local time (wall 1: 1410's answer, not harvested).
#[test]
fn a_file_time_is_its_own_local_time() {
    for wall in WALLS {
        let times = times(wall);
        let (mut value, mut local) = (0, 0);
        let lines = debug_lines(|| value = times.to_local_file_time(wall, &mut local));
        assert_eq!((value, local), (1, wall));
        let name = SHELL.calls.to_local_file_time;
        assert_eq!(
            lines,
            [format!("imp.call c={name} a={wall:x} v=1 f={wall:x}")]
        );
    }
}

/// `GetTimeZoneInformation` answers unknown, 0, and writes its 172 bytes
/// all 0, whatever was there (wall 1: 1410's answer, not harvested).
#[test]
fn the_zone_is_none_and_its_record_all_0() {
    for wall in WALLS {
        let times = times(wall);
        let (mut value, mut record) = (u32::MAX, [0xaa; ZONE_BYTES]);
        let lines = debug_lines(|| value = times.zone(&mut record));
        assert_eq!(value, 0, "{wall:x}");
        assert_eq!(record, [0; ZONE_BYTES], "{wall:x}");
        let name = SHELL.calls.zone;
        assert_eq!(lines, [format!("imp.call c={name} v=0")], "{wall:x}");
    }
}
