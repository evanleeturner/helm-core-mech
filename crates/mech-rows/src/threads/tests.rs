// SPDX-License-Identifier: MIT
//! Falsifier 3, the tests harvested: `h2`'s own thread lines under Wine,
//! played on the page's one thread and the host's real ones
//! (`harvest/h2-threads.log`, cut by `records/2026-11-03-lane-c/
//! helm-kernel-threads-kh1/kh1-s14-harvest.py`, its map of logbook line
//! numbers `kh1-s14-harvest-map.txt`). Each line the fronts write is the
//! harvest's, its clock, its level and the swap's `tid` dropped; each
//! `from` is h2's, passed in first as the swap's entries pass it. A number
//! here no h2 line holds is a bug.
//!
//! What h2's path never did is named, not tested (`kh1-s14-locks.txt`):
//! its 27 locks were all entered on the main thread, none twice at once,
//! none left with no take; `GetCurrentThreadId`, `TlsAlloc` and the two
//! counters made 0 calls after the first attach (their callers in
//! [`super`]'s table).
use crate::test_log::debug_lines;
use crate::threads::{GAME, SHELL};
use kernel_threads::host::Host;
use kernel_threads::page::Page;
use kernel_threads::{ThreadCalls, Threads};
use std::thread;

const H2: &str = include_str!("../../harvest/h2-threads.log");

/// The main thread's id in h2, the swap's `tid`.
const MAIN: &str = "158";

/// h2's line `number` (from 1) as the fronts write it, and its thread.
fn h2(number: usize) -> (&'static str, &'static str) {
    let line = H2
        .lines()
        .nth(number - 1)
        .expect("invariant: the harvest holds the line");
    let body = line
        .splitn(3, ' ')
        .nth(2)
        .expect("invariant: a clock, a level, then the body");
    body.rsplit_once(" tid=")
        .expect("invariant: the swap's tid ends each line")
}

/// `key`'s value in `line`.
fn field<'a>(line: &'a str, key: &str) -> &'a str {
    line.split(' ')
        .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
        .unwrap_or_else(|| panic!("{line} carries {key}"))
}

fn hex(text: &str) -> u32 {
    u32::from_str_radix(text, 16).expect("invariant: the line's numbers are hex")
}

/// Calls the front `line` names with its numbers and its `from`, and
/// checks the one line it writes is `line`.
fn play<T: Threads>(calls: &ThreadCalls<T>, number: usize, line: &str) {
    let from = hex(field(line, "from"));
    let a: Vec<u32> = field(line, "a").split(',').map(hex).collect();
    let lines = debug_lines(|| match field(line, "c") {
        "KERNEL32!InitializeCriticalSection" => calls.initialize(from, a[0]),
        "KERNEL32!EnterCriticalSection" => calls.enter(from, a[0]),
        "KERNEL32!LeaveCriticalSection" => calls.leave(from, a[0]),
        "KERNEL32!DeleteCriticalSection" => calls.delete(from, a[0]),
        "KERNEL32!TlsFree" => _ = calls.slot_free(from, a[0]),
        "KERNEL32!TlsGetValue" => _ = calls.slot_get(from, a[0]),
        "KERNEL32!TlsSetValue" => _ = calls.slot_set(from, a[0], a[1]),
        other => panic!("h2 harvested no {other}"),
    });
    assert_eq!(lines, [line], "harvest/h2-threads.log:{number}");
}

/// Plays a lock's lines on the page, checking its takes after each.
/// Returns the enters and leaves h2's main thread made.
fn lock_on_the_page(
    calls: &ThreadCalls<Page>,
    numbers: std::ops::RangeInclusive<usize>,
) -> (u32, u32) {
    let (mut enters, mut leaves) = (0, 0);
    for number in numbers {
        let (line, tid) = h2(number);
        assert_eq!(tid, MAIN, "harvest/h2-threads.log:{number}");
        play(calls, number, line);
        let lock = hex(field(line, "a"));
        match field(line, "c") {
            "KERNEL32!EnterCriticalSection" => enters += 1,
            "KERNEL32!LeaveCriticalSection" => leaves += 1,
            _ => {}
        }
        assert_eq!(
            calls.threads().takes(lock),
            enters - leaves,
            "harvest/h2-threads.log:{number}"
        );
    }
    (enters, leaves)
}

/// Lines 1 to 26: the shell's lock `15e1c70`, its whole life on the main
/// thread: initialized by the C library's `__lock`, entered and left 12
/// times, never twice at once, deleted by `__mtdeletelocks` as the shell
/// was freed.
#[test]
fn h2_the_shell_lock_15e1c70_whole_life() {
    let calls = ThreadCalls::new(&SHELL, Page::new());
    assert_eq!(lock_on_the_page(&calls, 1..=26), (12, 12));
}

/// Lines 27 to 29: the game file's lock `1710d90`, initialized, entered
/// and left once; the game died before its delete.
#[test]
fn h2_the_game_file_lock_1710d90() {
    let calls = ThreadCalls::new(&GAME, Page::new());
    assert_eq!(lock_on_the_page(&calls, 27..=29), (1, 1));
}

/// Takes slots on `host` up to `slot`, the slots below it standing for
/// those Wine and the files loaded before held, and sets the calling
/// thread's value in it: the C library's `__mtinit`, as the file loaded,
/// before the swap's attach, so h2 holds no line of it.
fn slot_as_loaded(host: &Host, slot: u32, value: u32) {
    for taken in 0..=slot {
        assert_eq!(host.slot_alloc(), Some(taken));
    }
    assert!(host.slot_set(slot, value));
}

/// Lines `pair` on a thread of their own: a second thread's get, which
/// reads 0, and its set of 0, as the C library's `__freeptd` runs when
/// that thread ends.
fn on_a_second_thread<T: Threads + Sync>(calls: &ThreadCalls<T>, pair: [usize; 2]) {
    thread::scope(|scope| {
        scope.spawn(|| {
            for number in pair {
                let (line, tid) = h2(number);
                assert_ne!(tid, MAIN, "harvest/h2-threads.log:{number}");
                play(calls, number, line);
            }
        });
    });
}

/// Lines 30 to 37: the shell's slot 5 on the host's threads: the main
/// thread's get of its value (`__getptd`); three second threads' get
/// and set as each ended; the main thread's free as the shell was freed
/// (`__mtterm`).
#[test]
fn h2_the_shell_slot_5_across_threads() {
    let calls = ThreadCalls::new(&SHELL, Host::new());
    slot_as_loaded(calls.threads(), 5, 0x015e_0488);
    play(&calls, 30, h2(30).0);
    for pair in [[31, 32], [33, 34], [35, 36]] {
        on_a_second_thread(&calls, pair);
    }
    let (line, tid) = h2(37);
    assert_eq!(tid, MAIN);
    play(&calls, 37, line);
}

/// Lines 38 to 40: the game file's slot 4: the main thread's get of its
/// value, then one second thread's get and set as it ended; the game died
/// before its free.
#[test]
fn h2_the_game_file_slot_4_across_threads() {
    let calls = ThreadCalls::new(&GAME, Host::new());
    slot_as_loaded(calls.threads(), 4, 0x0171_04a8);
    let (line, tid) = h2(38);
    assert_eq!(tid, MAIN);
    play(&calls, 38, line);
    on_a_second_thread(&calls, [39, 40]);
}
