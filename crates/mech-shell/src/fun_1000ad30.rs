// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(collection: i32, search_key: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ad30(collection: i32, search_key: i32) -> i32 {
    if collection == 0 {
        return 0;
    }
    let mut item_index: i32 = 0;
    // SAFETY: collection is a pointer to the game's collection structure; its
    // count at +4 and the items returned by the game's accessor are read as
    // the original does.
    unsafe {
        if 0 < ((collection.wrapping_add(4)) as *const i32).read_unaligned() {
            loop {
                let item = FUN_10018060(collection, item_index) as i32;
                if ((item.wrapping_add(0x810)) as *const i32).read_unaligned() == search_key {
                    return item;
                }
                item_index = item_index.wrapping_add(1);
                if item_index >= ((collection.wrapping_add(4)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
    }
    0
}
