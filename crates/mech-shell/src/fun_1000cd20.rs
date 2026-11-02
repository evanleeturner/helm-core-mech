// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(collection: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cd20(collection: i32) -> i32 {
    mech_trace::enter!("1000cd20", collection);
    let mut distinct_count: u32 = 0;
    if collection == 0 {
        return mech_trace::rt!("1000cd20", 0);
    }
    // SAFETY: collection is the game's own collection structure; items returned
    // by the game's accessor are read and written at the same offsets as the original.
    unsafe {
        let count_ptr = collection.wrapping_add(4) as *const i32;
        let mut item_index: i32 = 0;
        if 0 < mech_trace::rd!("1000cd20", count_ptr, read_unaligned) {
            loop {
                let inner_index = item_index.wrapping_add(1);
                let item =
                    mech_trace::cr!("1000cd20", "10018060", FUN_10018060(collection, item_index));
                mech_trace::wr!("1000cd20", (item.wrapping_add(9) as *mut u8), write, 0);
                item_index = inner_index;
                if !(inner_index < mech_trace::rd!("1000cd20", count_ptr, read_unaligned)) {
                    break;
                }
            }
        }
        item_index = 0;
        if 0 < mech_trace::rd!("1000cd20", count_ptr, read_unaligned) {
            loop {
                let outer_item =
                    mech_trace::cr!("1000cd20", "10018060", FUN_10018060(collection, item_index))
                        as *mut i32;
                if mech_trace::rd!("1000cd20", (outer_item as *mut u8).byte_add(9), read) != 1 {
                    let mut inner_index = item_index.wrapping_add(1);
                    mech_trace::wr!("1000cd20", (outer_item as *mut u8).byte_add(9), write, 1);
                    distinct_count = distinct_count.wrapping_add(1);
                    if inner_index < mech_trace::rd!("1000cd20", count_ptr, read_unaligned) {
                        loop {
                            let later_item = mech_trace::cr!(
                                "1000cd20",
                                "10018060",
                                FUN_10018060(collection, inner_index)
                            ) as *mut i32;
                            if mech_trace::rd!("1000cd20", later_item, read_unaligned)
                                == mech_trace::rd!("1000cd20", outer_item, read_unaligned)
                            {
                                mech_trace::wr!(
                                    "1000cd20",
                                    (later_item as *mut u8).byte_add(9),
                                    write,
                                    1
                                );
                            }
                            inner_index = inner_index.wrapping_add(1);
                            if !(inner_index
                                < mech_trace::rd!("1000cd20", count_ptr, read_unaligned))
                            {
                                break;
                            }
                        }
                    }
                }
                item_index = item_index.wrapping_add(1);
                if !(item_index < mech_trace::rd!("1000cd20", count_ptr, read_unaligned)) {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("1000cd20", distinct_count as i32)
}
