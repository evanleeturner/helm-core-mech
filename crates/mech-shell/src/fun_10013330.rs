// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1001e600(this: *mut u8, item: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10013330(
    collection_owner: *mut u8,
    item: u32,
    use_first_collection: i32,
) {
    mech_trace::enter!("10013330", collection_owner, item, use_first_collection);
    if use_first_collection != 0 {
        // SAFETY: collection_owner points at the game's owner object; the
        // held collection pointer is read at its offset and passed to the
        // game's own function.
        unsafe {
            let held = mech_trace::rd!(
                "10013330",
                collection_owner.byte_add(0x16).cast::<*mut u8>(),
                read_unaligned
            );
            FUN_1001e600(held, item);
        }
        return;
    }
    // SAFETY: as above, for the second collection pointer.
    unsafe {
        let held = mech_trace::rd!(
            "10013330",
            collection_owner.byte_add(0x1a).cast::<*mut u8>(),
            read_unaligned
        );
        FUN_1001e600(held, item);
    }
}
