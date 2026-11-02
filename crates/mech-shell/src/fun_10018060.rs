// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018020(collection: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018060(collection: i32, index: i32) -> u32 {
    mech_trace::enter!("10018060", collection, index);
    if collection == 0 {
        return mech_trace::rt!("10018060", 0);
    }
    // SAFETY: the callee is the game's own; collection is an address of the
    // game's collection structure, read at the offsets the original uses.
    unsafe {
        FUN_10018020(collection);
        let count = mech_trace::rd!(
            "10018060",
            ((collection.wrapping_add(4)) as *const i32),
            read_unaligned
        );
        if index <= count && -1 < index {
            let base = mech_trace::rd!(
                "10018060",
                ((collection.wrapping_add(0x11)) as *const i32),
                read_unaligned
            );
            return mech_trace::rt!(
                "10018060",
                mech_trace::rd!(
                    "10018060",
                    (base.wrapping_add(index.wrapping_mul(4)) as *const u32),
                    read_unaligned
                )
            );
        }
    }
    mech_trace::rt!("10018060", 0)
}
