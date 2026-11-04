// SPDX-License-Identifier: MIT
//! Falsifier 3, the test harvested: `t1`'s one time call, played on the
//! page's clocks at `t1`'s wall (`records/2026-11-03-lane-c/
//! helm-kernel-time-kt1/kt1-s8-t1-time-lines.txt`). `t1`'s path calls the
//! C library's `time` once and the 8 KERNEL32 names 0 times
//! (`kt1-s9-f3.txt`). A number here no `t1` line holds is a bug.
use crate::test_log::debug_lines;
use crate::time::{CLAMP_MICROS, SHELL};
use kernel_time::clock::Clock;
use kernel_time::page::Page;
use kernel_time::{InForce, Times, Zone};

/// `t1`'s `_time` line, whole.
const T1_TIME: &str = "imp.call c=_time a=0 v=6aea8734 t=1dd6bf9fd2c271d";

/// `t1`'s wall at that call: the line's `t`.
const T1_WALL: u64 = 0x1dd_6bf9_fd2c_271d;

#[test]
fn t1_time_is_its_line() {
    let times = Times::new(
        &SHELL,
        Page::new(
            Clock::new(CLAMP_MICROS, 0),
            T1_WALL,
            Zone::UTC,
            InForce::Unknown,
        ),
    );
    let mut value = 0;
    let lines = debug_lines(|| value = times.seconds(None));
    assert_eq!(lines, [T1_TIME]);
    assert_eq!(value, 0x6aea_8734);
}
