// SPDX-License-Identifier: MIT
//! `FUN_1001c5d1` checked against its own DEBUG rows: the port's source,
//! `src/fun_1001c5d1.rs`, compiled in whole, its callee and heap the kit's
//! stand-ins. With no block from the heap it writes nothing and answers
//! null; with one, every write lands inside it and each 32-bit write after a
//! read writes the value read; when `1001c303` answers null the block goes
//! back to `HeapFree` with the heap read and the answer is null, else the
//! answer is the block.
#![cfg(target_arch = "x86")]
#![allow(
    unsafe_code,
    reason = "the port's globals and callees are linked by name"
)]

#[path = "kit/fun_1001c303.rs"]
mod fun_1001c303;
#[path = "kit/heap.rs"]
mod heap;
#[path = "kit/heap_win.rs"]
mod heap_win;
mod kit;
#[path = "../src/fun_1001c5d1.rs"]
mod port;

use kit::{arg, field, number, of, one, shown, with};

#[test]
fn the_block_is_written_inside_and_freed_when_its_list_node_fails() {
    // SAFETY: one thread sets the stand-in global before any call reads it.
    unsafe { heap::DAT_100ab0c0 = 0x0123 };
    let mut harvest = Vec::new();
    let mut branches = [0; 3];
    let mut out = 0u32;
    let node = heap::block(0x20);
    for case in 0..3 {
        match case {
            0 => heap::fail_next(),
            1 => fun_1001c303::answer(0),
            _ => fun_1001c303::answer(node.addr()),
        }
        let (_, rows) = kit::rows(|| {
            // SAFETY: the port writes only inside the block the stand-in
            // HeapAlloc sized as it asked; its callee is a stand-in that
            // never touches `out`; one thread runs it.
            unsafe { port::FUN_1001c5d1(1, 2, 3, &raw mut out) }
        });
        let entry = one(&rows, "fn.in");
        let answer = field(one(&rows, "fn.ret"), "v");
        let allocs = heap::ALLOCS.take();
        let [alloc] = allocs.as_slice() else {
            panic!("one HeapAlloc: {allocs:?} {rows:?}")
        };
        let [got] = with(&rows, "call.ret", "c", "HeapAlloc")[..] else {
            panic!("one HeapAlloc row: {rows:?}")
        };
        let block = field(got, "v");
        assert_eq!(block, shown(alloc.answer), "{rows:?}");
        let heap_reads = with(&rows, "glob.rd", "g", "100adf74");
        assert_eq!(shown(alloc.heap), field(heap_reads[0], "v"), "{rows:?}");
        let frees = heap::FREES.take();
        let calls = fun_1001c303::CALLS.take();
        if alloc.answer == 0 {
            assert!(
                of(&rows, "mem.wr").is_empty() && calls.is_empty() && frees.is_empty(),
                "{rows:?}"
            );
            assert_eq!(answer, block, "{rows:?}");
            branches[0] += 1;
            harvest.extend(rows);
            continue;
        }
        let inside = alloc.answer..alloc.answer + alloc.what;
        assert!(
            of(&rows, "mem.wr")
                .iter()
                .all(|row| inside.contains(&number(row, "a"))),
            "{rows:?}"
        );
        // Each 32-bit write after a read writes the value read.
        for pair in rows.windows(2) {
            if pair[0].starts_with("mem.rd ") {
                assert!(pair[1].starts_with("mem.wr "), "{rows:?}");
                assert_eq!(field(&pair[1], "v"), field(&pair[0], "v"), "{rows:?}");
            }
        }
        let [(owner, _, first, second, third, out_at)] = calls.as_slice() else {
            panic!("one 1001c303 call: {calls:?} {rows:?}")
        };
        assert_eq!(shown(*owner), block, "{rows:?}");
        let handed = [shown(*first), shown(*second), shown(*third), shown(*out_at)];
        assert_eq!(
            handed,
            [arg(entry, 0), arg(entry, 1), arg(entry, 2), arg(entry, 3)],
            "{rows:?}"
        );
        let [list] = with(&rows, "call.ret", "c", "1001c303")[..] else {
            panic!("one 1001c303 row: {rows:?}")
        };
        if field(list, "v") == "0" {
            let [free] = frees.as_slice() else {
                panic!("one HeapFree: {frees:?} {rows:?}")
            };
            assert_eq!(shown(free.what), block, "{rows:?}");
            assert_eq!(
                shown(free.heap),
                field(heap_reads[heap_reads.len() - 1], "v"),
                "{rows:?}"
            );
            assert_eq!(answer, field(list, "v"), "{rows:?}");
            branches[1] += 1;
        } else {
            assert!(frees.is_empty(), "{frees:?} {rows:?}");
            assert_eq!(answer, block, "{rows:?}");
            branches[2] += 1;
        }
        harvest.extend(rows);
    }
    assert!(
        branches.iter().all(|&ran| ran > 0),
        "each branch ran: {branches:?}"
    );
    kit::harvest("heap_1001c5d1", &harvest);
}
