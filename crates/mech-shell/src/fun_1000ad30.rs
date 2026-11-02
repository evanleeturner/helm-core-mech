// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(collection: i32, search_key: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ad30(collection: i32, search_key: i32) -> i32 {
    mech_trace::enter!("1000ad30", collection, search_key);
    if collection == 0 {
        return mech_trace::rt!("1000ad30", 0);
    }
    let mut item_index: i32 = 0;
    // SAFETY: collection is a pointer to the game's collection structure; its
    // count at +4 and the items returned by the game's accessor are read as
    // the original does.
    unsafe {
        if 0 < mech_trace::rd!(
            "1000ad30",
            ((collection.wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let item =
                    mech_trace::cr!("1000ad30", "10018060", FUN_10018060(collection, item_index))
                        as i32;
                if mech_trace::rd!(
                    "1000ad30",
                    ((item.wrapping_add(0x810)) as *const i32),
                    read_unaligned
                ) == search_key
                {
                    return mech_trace::rt!("1000ad30", item);
                }
                item_index = item_index.wrapping_add(1);
                if item_index
                    >= mech_trace::rd!(
                        "1000ad30",
                        ((collection.wrapping_add(4)) as *const i32),
                        read_unaligned
                    )
                {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("1000ad30", 0)
}
