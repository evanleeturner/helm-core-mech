// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(item: *mut i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1001e600(collection_holder: *mut u8, item: u32) {
    mech_trace::enter!("1001e600", collection_holder, item);
    // SAFETY: collection_holder points at the game's holder object, whose first
    // field is the collection pointer; the callee is the game's own.
    unsafe {
        let collection = mech_trace::rd!(
            "1001e600",
            collection_holder.cast::<*mut i32>(),
            read_unaligned
        );
        FUN_10017df0(collection, item);
    }
}
