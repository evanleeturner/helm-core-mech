// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017f30(collection: i32, item: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017ec0(collection: i32, item: i32, invoke_item_callback: i8) {
    mech_trace::enter!("10017ec0", collection, item, invoke_item_callback);
    if collection == 0 {
        return;
    }
    // SAFETY: collection is a pointer to the game's collection structure;
    // fields and slots are read and written at the same offsets as the original.
    unsafe {
        let mut item_index =
            mech_trace::cr!("10017ec0", "10017f30", FUN_10017f30(collection, item));
        if item_index != 0xffff_ffff {
            let count_ptr = (collection.wrapping_add(4)) as *mut i32;
            mech_trace::wr!(
                "10017ec0",
                count_ptr,
                write_unaligned,
                mech_trace::rd!("10017ec0", count_ptr, read_unaligned).wrapping_sub(1)
            );
            if invoke_item_callback == 1 {
                let cb_ptr = (collection.wrapping_add(0x15)) as *const u32;
                if mech_trace::rd!("10017ec0", cb_ptr, read_unaligned) != 0 {
                    let cb: unsafe extern "C" fn(u32) =
                        core::mem::transmute::<usize, unsafe extern "C" fn(u32)>(mech_trace::rd!(
                            "10017ec0",
                            cb_ptr,
                            read_unaligned
                        )
                            as usize);
                    let base = mech_trace::rd!(
                        "10017ec0",
                        (collection.wrapping_add(0x11) as *const i32),
                        read_unaligned
                    );
                    let arg = mech_trace::rd!(
                        "10017ec0",
                        (base.wrapping_add((item_index.wrapping_mul(4)) as i32) as *const u32),
                        read_unaligned
                    );
                    mech_trace::cr!("10017ec0", "ind", cb(arg));
                }
            }
            if (item_index as i32) < mech_trace::rd!("10017ec0", count_ptr, read_unaligned) {
                let mut slot_offset = item_index.wrapping_mul(4) as i32;
                loop {
                    item_index = item_index.wrapping_add(1);
                    let base = mech_trace::rd!(
                        "10017ec0",
                        (collection.wrapping_add(0x11) as *const i32),
                        read_unaligned
                    );
                    let slot_ptr = base.wrapping_add(slot_offset) as *mut u32;
                    slot_offset = slot_offset.wrapping_add(4);
                    let next = mech_trace::rd!("10017ec0", slot_ptr.byte_add(4), read_unaligned);
                    mech_trace::wr!("10017ec0", slot_ptr, write_unaligned, next);
                    if (item_index as i32) >= mech_trace::rd!("10017ec0", count_ptr, read_unaligned)
                    {
                        break;
                    }
                }
            }
        }
    }
}
