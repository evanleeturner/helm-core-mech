// SPDX-License-Identifier: MIT
//! `FUN_10086951` checked against its own DEBUG rows: the port's source,
//! `src/fun_10086951.rs`, compiled in whole, its `HeapAlloc` the kit's C
//! stand-in, as the port declares it. `HeapAlloc` gets the size the port was
//! handed and the heap it read; the port returns nothing, so no `fn.ret`.
#![cfg(target_arch = "x86")]
#![allow(
    unsafe_code,
    reason = "the port's globals and callees are linked by name"
)]

#[path = "kit/heap.rs"]
mod heap;
#[path = "kit/heap_c.rs"]
mod heap_c;
mod kit;
#[path = "../src/fun_10086951.rs"]
mod port;

use kit::{field, of, one, shown};

#[test]
fn heap_alloc_gets_the_size_handed_and_the_heap_read() {
    let ((), rows) = kit::rows(|| {
        // SAFETY: the port reads the stand-in heap handle and calls the
        // stand-in HeapAlloc, whose block it drops; one thread runs it.
        unsafe { port::FUN_10086951(0x40) }
    });
    let allocs = heap::ALLOCS.take();
    let [alloc] = allocs.as_slice() else {
        panic!("one HeapAlloc: {allocs:?} {rows:?}")
    };
    let ret = one(&rows, "call.ret");
    assert_eq!(field(ret, "c"), "HeapAlloc", "{rows:?}");
    assert_eq!(field(ret, "v"), shown(alloc.answer), "{rows:?}");
    assert_eq!(
        shown(alloc.what),
        field(one(&rows, "fn.in"), "a"),
        "{rows:?}"
    );
    assert_eq!(
        shown(alloc.heap),
        field(one(&rows, "glob.rd"), "v"),
        "{rows:?}"
    );
    assert!(of(&rows, "fn.ret").is_empty(), "{rows:?}");
    kit::harvest("heap_10086951", &rows);
}
