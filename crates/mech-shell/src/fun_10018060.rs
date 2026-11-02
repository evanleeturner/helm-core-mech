// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018020(collection: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018060(collection: i32, index: i32) -> u32 {
    if collection == 0 {
        return 0;
    }
    // SAFETY: the callee is the game's own; collection is an address of the
    // game's collection structure, read at the offsets the original uses.
    unsafe {
        FUN_10018020(collection);
        let count = ((collection.wrapping_add(4)) as *const i32).read_unaligned();
        if index <= count && -1 < index {
            let base = ((collection.wrapping_add(0x11)) as *const i32).read_unaligned();
            return (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        }
    }
    0
}
