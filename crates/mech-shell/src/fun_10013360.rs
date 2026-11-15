// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1001e620(this: *mut u8, param_2: i32);
}

#[unsafe(export_name = "FUN_10013360")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn RemoveFromBothCollections(surface: *mut u8, item: i32) {
    mech_trace::enter!("10013360", surface, item);
    // SAFETY: `surface` points at the game's object; the two member pointers are
    // read at their offsets and passed on to the game's own function.
    unsafe {
        let first_collection_holder = mech_trace::rd!(
            "10013360",
            surface.byte_add(0x16).cast::<*mut u8>(),
            read_unaligned
        );
        FUN_1001e620(first_collection_holder, item);
        let second_collection_holder = mech_trace::rd!(
            "10013360",
            surface.byte_add(0x1a).cast::<*mut u8>(),
            read_unaligned
        );
        FUN_1001e620(second_collection_holder, item);
    }
}
