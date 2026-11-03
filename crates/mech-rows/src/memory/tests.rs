// SPDX-License-Identifier: MIT
//! The helpers on a test backend, `Recorder`, which answers each block
//! from a queue and keeps every ask. The answer key is two runs under
//! Wine: c36's own lines, the Rust's (`harvest/c36-heap.log`), and c35's
//! layer lines on the Windows heap, the original's
//! (`harvest/c35-heap.log`). Neither run has a null or a zero: those
//! edges are the bodies' own rules, as the crate's head reads them. The
//! debug build's heap, c37's, is [`debug`].
mod debug;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use crate::memory::{LAUNCHER, ROWS, SHELL};
use crate::test_log::debug_lines;
use kernel_memory::table::Row;
use kernel_memory::{Heap, Memory};

const C36: &str = include_str!("../../harvest/c36-heap.log");
const C35: &str = include_str!("../../harvest/c35-heap.log");

/// The shell's heap handle in both runs.
const SHELL_HEAP: usize = 0x015e_0000;

#[derive(Default)]
struct Recorder {
    /// The blocks to answer, in order; empty, a refusal.
    answers: RefCell<VecDeque<usize>>,
    /// Each ask: the bytes, and whether zeroed.
    asks: RefCell<Vec<(u32, bool)>>,
    releases: RefCell<Vec<usize>>,
    resizes: RefCell<Vec<(usize, u32)>>,
    new_mode: bool,
    /// The handler's answers, in order; empty, false.
    handler: RefCell<VecDeque<bool>>,
    handled: RefCell<Vec<u32>>,
}

impl Memory for Recorder {
    fn heap(&self) -> usize {
        SHELL_HEAP
    }

    fn alloc(&self, bytes: u32, zeroed: bool) -> *mut u8 {
        self.asks.borrow_mut().push((bytes, zeroed));
        block(self.answers.borrow_mut().pop_front().unwrap_or(0))
    }

    unsafe fn resize(&self, block: *mut u8, bytes: u32) -> *mut u8 {
        self.resizes.borrow_mut().push((block.addr(), bytes));
        self::block(self.answers.borrow_mut().pop_front().unwrap_or(0))
    }

    unsafe fn release(&self, block: *mut u8) {
        self.releases.borrow_mut().push(block.addr());
    }

    fn new_mode(&self) -> bool {
        self.new_mode
    }

    fn new_handler(&self, bytes: u32) -> bool {
        self.handled.borrow_mut().push(bytes);
        self.handler.borrow_mut().pop_front().unwrap_or(false)
    }
}

/// A block's address as a pointer; the recorder never reads through it.
fn block(address: usize) -> *mut u8 {
    core::ptr::without_provenance_mut(address)
}

fn heap(file: &'static Row, recorder: Recorder) -> Heap<Recorder> {
    Heap::new(file, recorder)
}

/// A line's value for `key`, as hex.
fn field(line: &str, key: &str) -> Option<usize> {
    line.split(' ')
        .find_map(|word| word.strip_prefix(key)?.strip_prefix('='))
        .map(|value| usize::from_str_radix(value, 16).expect("invariant: the runs log hex"))
}

/// The run's word after `c=`.
fn helper(line: &str) -> &str {
    line.split(' ')
        .find_map(|word| word.strip_prefix("c="))
        .expect("invariant: every harvested line names its call")
}

/// Each free's maker, by the makes' order: `None` for a block made before
/// the log began. An address made again after its free is a new block.
fn makers(events: &[(bool, usize)]) -> Vec<Option<usize>> {
    let mut live = HashMap::new();
    let mut made = 0;
    let mut frees = Vec::new();
    for &(make, address) in events {
        if make {
            live.insert(address, made);
            made += 1;
        } else {
            frees.push(live.remove(&address));
        }
    }
    frees
}

/// c36's calls replayed on the recorder, the run's own blocks answered.
fn replay() -> (Vec<String>, Heap<Recorder>) {
    let calls: Vec<&str> = C36.lines().collect();
    let answers = calls.iter().filter_map(|line| field(line, "v")).collect();
    let shell = heap(
        &SHELL,
        Recorder {
            answers: RefCell::new(answers),
            ..Recorder::default()
        },
    );
    let operator_new = SHELL
        .operator_new
        .expect("invariant: the shell carries new")
        .name;
    let operator_delete = SHELL
        .operator_delete
        .expect("invariant: the shell carries delete")
        .name;
    let lines = debug_lines(|| {
        for line in &calls {
            let a = field(line, "a").expect("invariant: every call logs its argument");
            let bytes = u32::try_from(a).expect("invariant: sizes are 32 bits");
            match helper(line) {
                name if name == SHELL.malloc.name => drop(shell.malloc(bytes)),
                name if name == operator_new => drop(shell.operator_new(bytes)),
                // SAFETY: the recorder only keeps the address.
                name if name == SHELL.free.name => unsafe { shell.free(block(a)) },
                // SAFETY: as above.
                name if name == operator_delete => unsafe { shell.operator_delete(block(a)) },
                name => panic!("c36 calls no {name}"),
            }
        }
    });
    (lines, shell)
}

#[test]
fn writes_each_line_c36_wrote() {
    let (lines, _) = replay();
    let want: Vec<&str> = C36
        .lines()
        .map(|line| line.splitn(3, ' ').nth(2).unwrap_or(""))
        .collect();
    assert_eq!(lines.len(), 267, "c36 wrote 267 lines");
    for (number, (got, want)) in lines.iter().zip(&want).enumerate() {
        assert_eq!(got, want, "harvest/c36-heap.log:{}", number + 1);
    }
}

#[test]
fn asks_the_heap_what_the_original_asked_in_c35() {
    let (_, shell) = replay();
    let original: Vec<&str> = C35.lines().collect();
    let asked: Vec<(u32, bool)> = original
        .iter()
        .filter(|line| helper(line) == "KERNEL32!HeapAlloc")
        .map(|line| {
            let a = line
                .split(' ')
                .find_map(|word| word.strip_prefix("a="))
                .unwrap_or("");
            let parts: Vec<&str> = a.split(',').collect();
            assert_eq!(parts[..2], ["15e0000", "0"], "{line}");
            (
                u32::from_str_radix(parts[2], 16).expect("invariant: hex"),
                false,
            )
        })
        .collect();
    assert_eq!(asked.len(), 148);
    assert_eq!(*shell.memory().asks.borrow(), asked);
}

#[test]
fn frees_each_block_the_make_the_original_freed_made() {
    let (_, shell) = replay();
    let c36: Vec<(bool, usize)> = C36
        .lines()
        .map(|line| match field(line, "v") {
            Some(block) => (true, block),
            None => (false, field(line, "a").unwrap_or(0)),
        })
        .collect();
    let c35: Vec<(bool, usize)> = C35
        .lines()
        .filter_map(|line| {
            let a = line.split(' ').find_map(|word| word.strip_prefix("a="))?;
            let parts: Vec<usize> = a
                .split(',')
                .map(|part| usize::from_str_radix(part, 16).unwrap_or(0))
                .collect();
            match helper(line) {
                "KERNEL32!HeapAlloc" => Some((true, field(line, "v")?)),
                "KERNEL32!HeapFree" if parts[0] == SHELL_HEAP => Some((false, parts[2])),
                _ => None,
            }
        })
        .collect();
    let released: Vec<usize> = c36
        .iter()
        .filter(|(make, _)| !make)
        .map(|&(_, a)| a)
        .collect();
    assert_eq!(*shell.memory().releases.borrow(), released);
    let (rust, original) = (makers(&c36), makers(&c35));
    assert_eq!(rust.len(), 119);
    assert_eq!(rust, original);
    assert_eq!(
        rust.iter().filter(|maker| maker.is_none()).count(),
        19,
        "made before the swap"
    );
}

#[test]
fn malloc_of_zero_asks_one_byte() {
    let shell = heap(
        &SHELL,
        Recorder {
            answers: RefCell::new([0x10].into()),
            ..Recorder::default()
        },
    );
    assert_eq!(shell.malloc(0).addr(), 0x10);
    assert_eq!(*shell.memory().asks.borrow(), [(1, false)]);
}

#[test]
fn free_and_delete_of_null_release_nothing_and_log_once() {
    let shell = heap(&SHELL, Recorder::default());
    // SAFETY: null is the helpers' own case.
    let lines = debug_lines(|| unsafe {
        shell.free(block(0));
        shell.operator_delete(block(0));
    });
    assert!(shell.memory().releases.borrow().is_empty());
    assert_eq!(
        lines,
        [
            "imp.call c=FUN_1006e790 a=0 h=15e0000",
            "imp.call c=FUN_1006ec30 a=0 h=15e0000"
        ]
    );
}

#[test]
fn a_refused_block_calls_the_handler_only_under_new_mode_or_new() {
    let shell = heap(&SHELL, Recorder::default());
    assert!(shell.malloc(8).is_null());
    assert!(shell.memory().handled.borrow().is_empty());
    let retried = Recorder {
        handler: RefCell::new([true].into()),
        ..Recorder::default()
    };
    let shell = heap(&SHELL, retried);
    assert!(shell.operator_new(8).is_null());
    assert_eq!(
        *shell.memory().handled.borrow(),
        [8, 8],
        "yes retries, no fails"
    );
    let shell = heap(
        &SHELL,
        Recorder {
            new_mode: true,
            ..Recorder::default()
        },
    );
    assert!(shell.malloc(0xffff_ffe1).is_null());
    assert!(shell.memory().asks.borrow().is_empty() && shell.memory().handled.borrow().is_empty());
}

#[test]
fn calloc_of_zero_asks_one_zeroed_byte_and_realloc_edges_are_malloc_and_free() {
    let answers = RefCell::new([0x10, 0x20].into());
    let shell = heap(
        &SHELL,
        Recorder {
            answers,
            ..Recorder::default()
        },
    );
    assert_eq!(shell.calloc(0, 4).addr(), 0x10);
    // SAFETY: the recorder only keeps the address.
    unsafe {
        assert_eq!(shell.realloc(block(0), 5).addr(), 0x20);
        assert!(shell.realloc(block(0x30), 0).is_null());
    }
    assert_eq!(*shell.memory().asks.borrow(), [(1, true), (5, false)]);
    assert_eq!(*shell.memory().releases.borrow(), [0x30]);
    assert!(shell.memory().resizes.borrow().is_empty());
}

#[test]
fn each_file_is_a_row_and_logs_its_own_names() {
    assert_eq!(ROWS, [&SHELL, &LAUNCHER]);
    for file in ROWS {
        let answers = RefCell::new([0x10].into());
        let heap = heap(
            file,
            Recorder {
                answers,
                ..Recorder::default()
            },
        );
        let lines = debug_lines(|| {
            let made = heap.malloc(4);
            // SAFETY: the recorder only keeps the address.
            unsafe { heap.free(made) };
        });
        let want = [
            format!("imp.call c={} a=4 h=15e0000 v=10", file.malloc.name),
            format!("imp.call c={} a=10 h=15e0000", file.free.name),
        ];
        assert_eq!(lines, want, "{}", file.file);
    }
}
