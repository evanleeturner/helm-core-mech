// SPDX-License-Identifier: MIT
//! `FUN_1001c989` checked against its own DEBUG rows: the port's source,
//! `src/fun_1001c989.rs`, compiled in whole, its callees and heap the kit's
//! stand-ins. Each node of the object's list goes to `1001c8a3` in the order
//! the `mem.rd` chain read them, each next link read before its node goes;
//! then the object goes to `1001c110` and to `HeapFree` with the heap read.
#![cfg(target_arch = "x86")]
#![allow(
    unsafe_code,
    reason = "the port's globals and callees are linked by name"
)]

#[path = "kit/fun_1001c110.rs"]
mod fun_1001c110;
#[path = "kit/fun_1001c8a3.rs"]
mod fun_1001c8a3;
#[path = "kit/heap.rs"]
mod heap;
#[path = "kit/heap_win.rs"]
mod heap_win;
mod kit;
#[path = "../src/fun_1001c989.rs"]
mod port;

use kit::{field, of, one, shown};

/// The object's list head and a node's next link, at the port's offsets:
/// the test's own layout of its input.
const HEAD: usize = 0x1c;
const NEXT: usize = 0xc;

/// An object whose list holds `nodes` nodes, linked in order.
fn object(nodes: usize) -> *mut u8 {
    let object = heap::block(0x20);
    let mut link = object.wrapping_add(HEAD);
    for _ in 0..nodes {
        let node = heap::block(0x10);
        // SAFETY: `link` is the head of a 0x20-byte block or the next link
        // of a 0x10-byte node, both the arena's and in bounds.
        unsafe { link.cast::<*mut u8>().write_unaligned(node) };
        link = node.wrapping_add(NEXT);
    }
    object
}

#[test]
fn each_node_goes_in_the_chains_order_then_the_object_goes() {
    let mut harvest = Vec::new();
    let mut branches = [0; 2];
    for nodes in [0, 2] {
        let object = object(nodes);
        let ((), rows) = kit::rows(|| {
            // SAFETY: the object and its nodes are arena blocks linked as the
            // port walks them; its callees are stand-ins that never touch
            // them; one thread runs it.
            unsafe { port::FUN_1001c989(object.cast()) }
        });
        let entry = field(one(&rows, "fn.in"), "a");
        let reads = of(&rows, "mem.rd");
        let Some((last, chain)) = reads.split_last() else {
            panic!("the head is read: {rows:?}")
        };
        assert_eq!(field(last, "v"), "0", "the chain ends at null: {rows:?}");
        let handed: Vec<String> = fun_1001c8a3::CALLS.take().into_iter().map(shown).collect();
        let linked: Vec<&str> = chain.iter().map(|row| field(row, "v")).collect();
        assert_eq!(handed, linked, "{rows:?}");
        // Each node's next link is read before the node goes.
        let walk: Vec<&str> = rows
            .iter()
            .filter_map(|row| match row.split(' ').next() {
                Some("mem.rd") => Some("rd"),
                Some("call.ret") if kit::value(row, "c") == Some("1001c8a3") => Some("free"),
                _ => None,
            })
            .collect();
        let mut order = vec!["rd"];
        order.extend(std::iter::repeat_n(["rd", "free"], handed.len()).flatten());
        assert_eq!(walk, order, "{rows:?}");
        let objects: Vec<String> = fun_1001c110::CALLS.take().into_iter().map(shown).collect();
        assert_eq!(objects, [entry], "{rows:?}");
        let frees = heap::FREES.take();
        let [free] = frees.as_slice() else {
            panic!("one HeapFree: {frees:?} {rows:?}")
        };
        assert_eq!(shown(free.what), entry, "{rows:?}");
        assert_eq!(
            shown(free.heap),
            field(one(&rows, "glob.rd"), "v"),
            "{rows:?}"
        );
        branches[usize::from(!handed.is_empty())] += 1;
        harvest.extend(rows);
    }
    assert!(
        branches.iter().all(|&ran| ran > 0),
        "both branches ran: {branches:?}"
    );
    kit::harvest("heap_1001c989", &harvest);
}
