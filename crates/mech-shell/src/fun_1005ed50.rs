// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017df0(item: *mut i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005ed50(collection_holder: *mut core::ffi::c_void, item: u32) {
    mech_trace::enter!("1005ed50", collection_holder, item);
    // SAFETY: collection_holder points at the game's holder whose first field
    // is a pointer to the collection; the callee is the game's own.
    unsafe {
        let first = mech_trace::rd!(
            "1005ed50",
            collection_holder.cast::<*mut i32>(),
            read_unaligned
        );
        FUN_10017df0(first, item);
    }
}
