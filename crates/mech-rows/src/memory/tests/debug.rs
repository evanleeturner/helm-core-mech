// SPDX-License-Identifier: MIT
//! The debug rules on a test backend, `Board`, which keeps the game's words
//! in a map, counts the lock, and answers each block from a queue. The
//! answer key is c37 under Wine: every call `mercsw`'s C-library wrappers
//! made of the Windows heap (`harvest/c37-mercsw-heap.log`, Wine's relay cut
//! to the calls returning into `10096090` to `100962c0`). The run has no refusal and no block
//! crossing to the unswapped C: those edges are the rules' own, as the
//! module's head reads them.
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};

use crate::memory::{MERCSW, MERCSW_LIST};
use kernel_memory::Memory;
use kernel_memory::debug::{DebugHeap, Words};
use crate::test_log::debug_lines;

const C37: &str = include_str!("../../../harvest/c37-mercsw-heap.log");

/// `mercsw`'s heap handle in c37.
const MERCSW_HEAP: u32 = 0x015e_0000;
/// Wall 3's reading of `1008d040`, kept apart from the module's own so the
/// tests check it: the header before the user's bytes, what a block asks
/// beyond the user's size, and the guards' word.
const HEADER: u32 = 0x20;
const OVERHEAD: u32 = 0x24;
const GUARD: u32 = 0xfdfd_fdfd;

#[derive(Default)]
struct Board {
    /// The game's words by address; a word never written reads 0.
    words: RefCell<HashMap<u32, u32>>,
    /// The blocks to answer, in order; empty, a refusal.
    answers: RefCell<VecDeque<u32>>,
    /// Each ask: the bytes, and the lock's depth.
    asks: RefCell<Vec<(u32, u32)>>,
    /// Each release: the block, and the lock's depth.
    releases: RefCell<Vec<(u32, u32)>>,
    /// The lock's depth now, and the deepest it went.
    held: Cell<u32>,
    deepest: Cell<u32>,
    new_mode: bool,
    /// The handler's answers, in order; empty, false.
    handler: RefCell<VecDeque<bool>>,
    /// Each handler call: the bytes, and the lock's depth.
    handled: RefCell<Vec<(u32, u32)>>,
}

impl Board {
    fn word(&self, address: u32) -> u32 {
        self.words.borrow().get(&address).copied().unwrap_or(0)
    }

    fn set(&self, address: u32, value: u32) {
        self.words.borrow_mut().insert(address, value);
    }
}

impl Memory for Board {
    fn heap(&self) -> usize {
        usize::try_from(MERCSW_HEAP).expect("invariant: the handle fits a pointer")
    }

    fn alloc(&self, bytes: u32, zeroed: bool) -> *mut u8 {
        assert!(!zeroed, "the debug make asks no zeroed block");
        self.asks.borrow_mut().push((bytes, self.held.get()));
        pointer(self.answers.borrow_mut().pop_front().unwrap_or(0))
    }

    unsafe fn resize(&self, _: *mut u8, _: u32) -> *mut u8 {
        panic!("the debug rules resize nothing")
    }

    unsafe fn release(&self, block: *mut u8) {
        self.releases
            .borrow_mut()
            .push((at(block), self.held.get()));
    }

    fn new_mode(&self) -> bool {
        self.new_mode
    }

    fn new_handler(&self, bytes: u32) -> bool {
        self.handled.borrow_mut().push((bytes, self.held.get()));
        self.handler.borrow_mut().pop_front().unwrap_or(false)
    }
}

impl Words for Board {
    fn lock(&self) {
        let held = self.held.get() + 1;
        self.held.set(held);
        self.deepest.set(self.deepest.get().max(held));
    }

    fn unlock(&self) {
        let held = self.held.get().checked_sub(1);
        self.held
            .set(held.expect("invariant: every unlock follows a lock"));
    }

    unsafe fn read(&self, address: u32) -> u32 {
        self.word(address)
    }

    unsafe fn write(&self, address: u32, value: u32) {
        self.set(address, value);
    }
}

/// A game address as a pointer; the board never reads through it.
fn pointer(address: u32) -> *mut u8 {
    core::ptr::without_provenance_mut(
        usize::try_from(address).expect("invariant: the game's addresses fit a pointer"),
    )
}

/// A pointer as the game's address.
fn at(block: *mut u8) -> u32 {
    u32::try_from(block.addr()).expect("invariant: the board's blocks are the game's")
}

fn debug(board: Board) -> DebugHeap<Board> {
    DebugHeap::new(&MERCSW, &MERCSW_LIST, board)
}

/// One harvested call: a make's ask and the block it got, or a free's block.
#[derive(Clone, Copy)]
enum Call {
    Make { ask: u32, block: u32 },
    Free { block: u32 },
}

/// c37's calls in order, each on `mercsw`'s heap at flags 0.
fn calls() -> Vec<Call> {
    let hex = |value: &str| u32::from_str_radix(value, 16).expect("invariant: the relay logs hex");
    C37.lines()
        .map(|line| {
            let value = |key: &str| {
                line.split(' ')
                    .find_map(|word| word.strip_prefix(key)?.strip_prefix('='))
                    .expect("invariant: every harvested line logs c, a and v")
            };
            let args: Vec<u32> = value("a").split(',').map(hex).collect();
            assert_eq!(args[..2], [MERCSW_HEAP, 0], "{line}");
            match value("c") {
                "RtlAllocateHeap" => Call::Make {
                    ask: args[2],
                    block: hex(value("v")),
                },
                "HeapFree" => Call::Free { block: args[2] },
                name => panic!("c37's wrappers call no {name}"),
            }
        })
        .collect()
}

/// c37's makes, each its ask and block.
fn makes() -> Vec<(u32, u32)> {
    calls()
        .into_iter()
        .filter_map(|call| match call {
            Call::Make { ask, block } => Some((ask, block)),
            Call::Free { .. } => None,
        })
        .collect()
}

/// c37's calls replayed on the board, the run's own blocks answered: each
/// make's user size the ask less the overhead, each free the user's pointer.
/// The lines, the pointers malloc handed, and the helpers.
fn replay() -> (Vec<String>, Vec<u32>, DebugHeap<Board>) {
    let answers = makes().into_iter().map(|(_, block)| block).collect();
    let heap = debug(Board {
        answers: RefCell::new(answers),
        ..Board::default()
    });
    let mut handed = Vec::new();
    let lines = debug_lines(|| {
        for call in calls() {
            match call {
                Call::Make { ask, .. } => handed.push(at(heap.malloc(ask - OVERHEAD))),
                // SAFETY: the board only keeps the address.
                Call::Free { block } => unsafe { heap.free(pointer(block + HEADER)) },
            }
        }
    });
    (lines, handed, heap)
}

#[test]
fn asks_the_heap_what_mercsw_asked_in_c37_and_hands_the_block_past_its_header() {
    let (_, handed, heap) = replay();
    let makes = makes();
    assert_eq!(makes.len(), 171);
    let asked: Vec<(u32, u32)> = makes.iter().map(|&(ask, _)| (ask, 1)).collect();
    assert_eq!(
        *heap.memory().asks.borrow(),
        asked,
        "each ask under the lock"
    );
    let user: Vec<u32> = makes.iter().map(|&(_, block)| block + HEADER).collect();
    assert_eq!(handed, user);
}

#[test]
fn frees_each_block_c37_freed_and_keeps_the_rest_listed() {
    let (_, _, heap) = replay();
    let board = heap.memory();
    let mut live = Vec::new();
    let mut freed = Vec::new();
    for call in calls() {
        match call {
            Call::Make { block, .. } => live.push(block),
            Call::Free { block } => {
                let made = live.iter().position(|&made| made == block);
                live.remove(made.expect("c37 frees only a live block"));
                freed.push((block, 1));
            }
        }
    }
    assert_eq!(freed.len(), 8);
    assert_eq!(
        *board.releases.borrow(),
        freed,
        "each release under the lock"
    );
    let mut listed = Vec::new();
    let (mut header, mut newer) = (board.word(MERCSW_LIST.first), 0);
    while header != 0 {
        assert_eq!(board.word(header + 4), newer, "{header:x} links back");
        listed.push(header);
        (header, newer) = (board.word(header), header);
    }
    assert_eq!(board.word(MERCSW_LIST.last), newer);
    live.reverse();
    assert_eq!(listed.len(), 163);
    assert_eq!(listed, live, "newest first");
    assert_eq!(
        (board.held.get(), board.deepest.get()),
        (0, 1),
        "lock and unlock paired"
    );
}

#[test]
fn writes_one_line_a_call_as_c37_called() {
    let (lines, _, _) = replay();
    let want: Vec<String> = calls()
        .into_iter()
        .map(|call| match call {
            Call::Make { ask, block } => format!(
                "imp.call c=_malloc a={:x} h=15e0000 v={:x}",
                ask - OVERHEAD,
                block + HEADER
            ),
            Call::Free { block } => {
                format!("imp.call c=FUN_1008d9b0 a={:x} h=15e0000", block + HEADER)
            }
        })
        .collect();
    assert_eq!(lines.len(), 179, "171 makes and 8 frees, one line each");
    assert_eq!(lines, want);
}

#[test]
fn writes_the_header_and_guards_as_the_table_reads() {
    let makes = makes();
    let [(older_ask, older), (newer_ask, newer)] = [makes[0], makes[1]];
    let heap = debug(Board {
        answers: RefCell::new([older, newer].into()),
        ..Board::default()
    });
    let board = heap.memory();
    let size = older_ask - OVERHEAD;
    assert_eq!(at(heap.malloc(size)), older + HEADER);
    let offsets = [0, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, HEADER + size];
    assert_eq!(
        offsets.map(|offset| board.word(older + offset)),
        [0, 0, 0, 0, size, 1, 0, GUARD, GUARD]
    );
    let ends = || (board.word(MERCSW_LIST.first), board.word(MERCSW_LIST.last));
    assert_eq!(ends(), (older, older), "one block, both ends");
    assert_eq!(at(heap.malloc(newer_ask - OVERHEAD)), newer + HEADER);
    assert_eq!(
        [
            board.word(newer),
            board.word(newer + 4),
            board.word(older + 4)
        ],
        [older, 0, newer],
        "the newer names the older next; the older names it previous"
    );
    assert_eq!(ends(), (newer, older));
}

#[test]
fn a_block_crosses_between_the_unswapped_c_and_the_rules_both_ways() {
    let makes = makes();
    let [(c_ask, c_block), (rust_ask, rust_block)] = [makes[2], makes[3]];
    let heap = debug(Board {
        answers: RefCell::new([rust_block].into()),
        ..Board::default()
    });
    let board = heap.memory();
    // Made by the unswapped C: the words `1008d040` writes, set by hand.
    let c_size = c_ask - OVERHEAD;
    for (offset, word) in [
        (0x10, c_size),
        (0x14, 1),
        (0x1c, GUARD),
        (HEADER + c_size, GUARD),
    ] {
        board.set(c_block + offset, word);
    }
    board.set(MERCSW_LIST.first, c_block);
    board.set(MERCSW_LIST.last, c_block);
    assert_eq!(at(heap.malloc(rust_ask - OVERHEAD)), rust_block + HEADER);
    // SAFETY: the board only keeps the address.
    unsafe { heap.free(pointer(c_block + HEADER)) };
    assert_eq!(*board.releases.borrow(), [(c_block, 1)]);
    assert_eq!(
        [
            MERCSW_LIST.first,
            MERCSW_LIST.last,
            rust_block,
            rust_block + 4
        ]
        .map(|a| board.word(a)),
        [rust_block, rust_block, 0, 0],
        "the C block unlinked here; the rules' block alone on the list"
    );
    // Freed by the unswapped C, `1008da00`: the words it reads are there.
    let size = board.word(rust_block + 0x10);
    assert_eq!(size, rust_ask - OVERHEAD);
    assert_eq!(
        [0x14, 0x1c, HEADER + size].map(|offset| board.word(rust_block + offset)),
        [1, GUARD, GUARD]
    );
}

#[test]
fn a_refused_block_calls_the_handler_only_under_new_mode_with_the_lock_given_back() {
    let size = makes()[0].0 - OVERHEAD;
    let heap = debug(Board::default());
    assert!(heap.malloc(size).is_null());
    assert_eq!(*heap.memory().asks.borrow(), [(size + OVERHEAD, 1)]);
    assert!(heap.memory().handled.borrow().is_empty());
    let heap = debug(Board {
        new_mode: true,
        handler: RefCell::new([true].into()),
        ..Board::default()
    });
    assert!(heap.malloc(size).is_null());
    assert_eq!(
        *heap.memory().handled.borrow(),
        [(size, 0), (size, 0)],
        "yes retries, no fails, each after the unlock"
    );
    assert_eq!(heap.memory().asks.borrow().len(), 2);
    // The original's add wraps from here; the rules fail it without asking.
    let heap = debug(Board {
        new_mode: true,
        ..Board::default()
    });
    assert!(heap.malloc(0xffff_ffdc).is_null());
    assert!(heap.memory().asks.borrow().is_empty());
    assert_eq!(*heap.memory().handled.borrow(), [(0xffff_ffdc, 0)]);
}

#[test]
fn free_of_null_releases_nothing_and_logs_once() {
    let heap = debug(Board::default());
    // SAFETY: null is the helper's own case.
    let lines = debug_lines(|| unsafe { heap.free(pointer(0)) });
    let board = heap.memory();
    assert!(board.releases.borrow().is_empty());
    assert_eq!(lines, ["imp.call c=FUN_1008d9b0 a=0 h=15e0000"]);
    assert_eq!((board.held.get(), board.deepest.get()), (0, 1));
}
