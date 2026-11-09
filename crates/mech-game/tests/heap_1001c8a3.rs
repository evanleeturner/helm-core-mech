// SPDX-License-Identifier: MIT
//! `FUN_1001c8a3` checked against its own DEBUG rows: the port's source,
//! `src/fun_1001c8a3.rs`, compiled in whole, its heap the kit's stand-ins.
//! A null block makes no call; any other goes to `HeapFree` with the heap
//! the port read, both read from the same call's `fn.in` and `glob.rd`.
#![cfg(target_arch = "x86")]
#![allow(
    unsafe_code,
    reason = "the port's globals and callees are linked by name"
)]

#[path = "kit/heap.rs"]
mod heap;
#[path = "kit/heap_win.rs"]
mod heap_win;
mod kit;
#[path = "../src/fun_1001c8a3.rs"]
mod port;

use kit::{field, of, one, shown};

#[test]
fn a_block_goes_to_heap_free_with_the_heap_read_and_null_makes_no_call() {
    let mut harvest = Vec::new();
    let mut branches = [0; 2];
    for block in [core::ptr::null_mut(), heap::block(0x10)] {
        let ((), rows) = kit::rows(|| {
            // SAFETY: the port reads the stand-in heap handle and hands the
            // block to the stand-in HeapFree, which never touches it; one
            // thread runs it.
            unsafe { port::FUN_1001c8a3(block.cast()) }
        });
        let frees = heap::FREES.take();
        let entry = one(&rows, "fn.in");
        assert_eq!(field(entry, "a"), shown(block.addr()), "{rows:?}");
        match of(&rows, "call.ret").as_slice() {
            [] => {
                assert!(frees.is_empty(), "{frees:?} {rows:?}");
                branches[0] += 1;
            }
            [ret] => {
                let [free] = frees.as_slice() else {
                    panic!("one HeapFree: {frees:?} {rows:?}")
                };
                assert_eq!(field(ret, "c"), "HeapFree", "{rows:?}");
                assert_eq!(shown(free.what), field(entry, "a"), "{rows:?}");
                assert_eq!(
                    shown(free.heap),
                    field(one(&rows, "glob.rd"), "v"),
                    "{rows:?}"
                );
                branches[1] += 1;
            }
            _ => panic!("one call at most: {rows:?}"),
        }
        harvest.extend(rows);
    }
    assert!(
        branches.iter().all(|&ran| ran > 0),
        "both branches ran: {branches:?}"
    );
    kit::harvest("heap_1001c8a3", &harvest);
}
