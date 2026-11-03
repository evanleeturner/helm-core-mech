// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(collection_holder_likely: i32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100402b0(collection_holder_likely: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_1001e6b0(collection_holder_likely: *mut i32) {
    mech_trace::enter!("1001e6b0", collection_holder_likely);
    let mut item_index: i32 = 0;
    // SAFETY: the pointer and the collection it holds belong to the game; the
    // reads and calls happen in the original's order.
    unsafe {
        if 0 < mech_trace::rd!(
            "1001e6b0",
            ((mech_trace::rd!("1001e6b0", collection_holder_likely, read_unaligned) as u32)
                .wrapping_add(4) as *const i32),
            read_unaligned
        ) {
            loop {
                let item = mech_trace::cr!(
                    "1001e6b0",
                    "10018060",
                    FUN_10018060(
                        mech_trace::rd!("1001e6b0", collection_holder_likely, read_unaligned),
                        item_index
                    )
                ) as *mut u32;
                if !item.is_null() {
                    mech_trace::cr!("1001e6b0", "100402b0", FUN_100402b0(item));
                }
                item_index = item_index.wrapping_add(1);
                if !(item_index
                    < mech_trace::rd!(
                        "1001e6b0",
                        ((mech_trace::rd!("1001e6b0", collection_holder_likely, read_unaligned)
                            as u32)
                            .wrapping_add(4) as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
    }
}
