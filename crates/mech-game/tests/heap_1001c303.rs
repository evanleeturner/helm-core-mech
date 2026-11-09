// SPDX-License-Identifier: MIT
//! `FUN_1001c303` checked against its own DEBUG rows: the port's source,
//! `src/fun_1001c303.rs`, compiled in whole, its callee and heap the kit's
//! stand-ins. The block it answers is the heap's, cleared from its start
//! before any other write, keyed by the key handed, its out pointer set
//! inside it, handed to `1001c299` with the owner, and linked into the
//! owner's list in key order: at an empty head, at the front, at the end,
//! and in the middle, with sizes with and without a remainder.
#![cfg(target_arch = "x86")]
#![allow(
    unsafe_code,
    reason = "the port's globals and callees are linked by name"
)]

#[path = "kit/fun_1001c299.rs"]
mod fun_1001c299;
#[path = "kit/heap.rs"]
mod heap;
#[path = "kit/heap_win.rs"]
mod heap_win;
mod kit;
#[path = "../src/fun_1001c303.rs"]
mod port;

use kit::{arg, field, number, of, one, shown, with};

/// The owner's list head and a node's next link, at the port's offsets:
/// the test's own layout of its input.
const HEAD: usize = 0x1c;
const NEXT: usize = 0xc;

/// Calls the port for `owner` with `key` and the three counts its size is
/// made from; returns its answer and the rows it wrote.
fn insert(owner: *mut u8, key: i32, counts: [i32; 3], out: *mut u32) -> (*mut i32, Vec<String>) {
    kit::rows(|| {
        // SAFETY: owner is null or a 0x20-byte arena block whose list the
        // port wrote; the port writes the block the stand-in sized as it
        // asked, and `out`, a live u32; one thread runs it.
        unsafe {
            port::FUN_1001c303(
                kit::word(owner.addr()),
                key,
                counts[0],
                counts[1],
                counts[2],
                out,
            )
        }
    })
}

/// The keys of the owner's list, walked in memory from its head.
fn keys(owner: *mut u8) -> Vec<i32> {
    let mut keys = Vec::new();
    // SAFETY: the head and each link are null or a block the port linked.
    unsafe {
        let mut node = owner.add(HEAD).cast::<*mut u8>().read_unaligned();
        while !node.is_null() {
            keys.push(node.cast::<i32>().read_unaligned());
            node = node.add(NEXT).cast::<*mut u8>().read_unaligned();
        }
    }
    keys
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one call per branch, in order, so the harvest is one run's log"
)]
fn each_block_is_the_heaps_cleared_keyed_and_linked_in_key_order() {
    let mut harvest = Vec::new();
    let mut out = 0u32;

    // No owner: null, and no heap call.
    let (_, rows) = insert(core::ptr::null_mut(), 1, [0; 3], &raw mut out);
    assert_eq!(
        field(one(&rows, "fn.ret"), "v"),
        arg(one(&rows, "fn.in"), 0),
        "{rows:?}"
    );
    assert!(
        of(&rows, "call.ret").is_empty() && heap::ALLOCS.take().is_empty(),
        "{rows:?}"
    );
    harvest.extend(rows);

    // The heap out of room: its null is the answer, and nothing is written.
    let owner = heap::block(0x20);
    heap::fail_next();
    let (_, rows) = insert(owner, 1, [0; 3], &raw mut out);
    let ret = field(one(&rows, "call.ret"), "v");
    assert_eq!(
        heap::ALLOCS
            .take()
            .iter()
            .map(|a| shown(a.answer))
            .collect::<Vec<_>>(),
        [ret]
    );
    assert_eq!(field(one(&rows, "fn.ret"), "v"), ret, "{rows:?}");
    assert!(
        of(&rows, "mem.wr").is_empty() && fun_1001c299::CALLS.take().is_empty(),
        "{rows:?}"
    );
    harvest.extend(rows);

    // (key, counts): an empty head, the front, the end, the middle; the
    // third count never 0, as a 0 leaves the out pointer at the block's end.
    let inserts = [
        (5, [1, 1, 3]),
        (2, [0, 0, 4]),
        (9, [1, 0, 2]),
        (7, [0, 1, 1]),
    ];
    let mut branches = [0; 4];
    let mut remainders = [0; 2];
    let head = shown(owner.addr() + HEAD);
    for (made, (key, counts)) in inserts.into_iter().enumerate() {
        let (block, rows) = insert(owner, key, counts, &raw mut out);
        let entry = one(&rows, "fn.in");
        let allocs = heap::ALLOCS.take();
        let [alloc] = allocs.as_slice() else {
            panic!("one HeapAlloc: {allocs:?} {rows:?}")
        };
        let at = shown(block.addr());
        assert_eq!(field(one(&rows, "fn.ret"), "v"), at, "{rows:?}");
        assert_eq!(
            field(one(&rows, "call.ret"), "v"),
            shown(alloc.answer),
            "{rows:?}"
        );
        assert_eq!(
            shown(alloc.heap),
            field(one(&rows, "glob.rd"), "v"),
            "{rows:?}"
        );
        remainders[usize::from(alloc.what % 4 != 0)] += 1;
        // The clear: zeros at rising addresses from the block's start, ending
        // within a word of the size asked, before the block's second write.
        let writes = of(&rows, "mem.wr");
        let key_at = writes
            .iter()
            .skip(1)
            .position(|row| field(row, "a") == at)
            .unwrap_or_else(|| panic!("a second write at the block: {rows:?}"))
            + 1;
        let clear = &writes[..key_at];
        assert_eq!(field(clear[0], "a"), at, "{rows:?}");
        assert!(clear.iter().all(|row| field(row, "v") == "0"), "{rows:?}");
        assert!(
            clear
                .windows(2)
                .all(|w| number(w[0], "a") < number(w[1], "a")),
            "{rows:?}"
        );
        let end = number(clear[clear.len() - 1], "a");
        assert!(
            end < alloc.answer + alloc.what && alloc.answer + alloc.what <= end + 4,
            "{rows:?}"
        );
        assert_eq!(
            field(writes[key_at], "v"),
            arg(entry, 1),
            "the key: {rows:?}"
        );
        // The out pointer is set inside the block.
        let set = with(&rows, "mem.wr", "a", arg(entry, 5));
        let [set] = set.as_slice() else {
            panic!("one write through the out pointer: {rows:?}")
        };
        assert!(
            (alloc.answer..alloc.answer + alloc.what).contains(&number(set, "v")),
            "{rows:?}"
        );
        // 1001c299 got the owner and the block.
        let handed: Vec<(String, String)> = fun_1001c299::CALLS
            .take()
            .into_iter()
            .map(|(o, b)| (shown(o), shown(b)))
            .collect();
        assert_eq!(handed, [(arg(entry, 0).to_owned(), at.clone())], "{rows:?}");
        // The branch, by the head's rows and the walk's reads.
        let head_read = with(&rows, "mem.rd", "a", &head);
        let branch = if field(head_read[0], "v") == "0" {
            0
        } else if !with(&rows, "mem.wr", "a", &head).is_empty() {
            1
        } else if !with(&rows, "mem.rd", "v", "0").is_empty() {
            2
        } else {
            3
        };
        branches[branch] += 1;
        let listed = keys(owner);
        assert_eq!(listed.len(), made + 1, "{listed:?}");
        assert!(
            listed.is_sorted() && listed.contains(&key),
            "{listed:?} {rows:?}"
        );
        harvest.extend(rows);
    }
    assert!(
        branches.iter().all(|&ran| ran > 0),
        "each insert ran: {branches:?}"
    );
    assert!(
        remainders.iter().all(|&ran| ran > 0),
        "both sizes ran: {remainders:?}"
    );
    kit::harvest("heap_1001c303", &harvest);
}
