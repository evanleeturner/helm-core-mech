// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10042370(lookup_id: i32, lookup_key_ptr: *mut i32) -> *mut i16;
    fn FUN_100423d0(entry: *mut i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042730(lookup_id_likely: i32, lookup_key_ptr_likely: *mut i32) {
    mech_trace::enter!("10042730", lookup_id_likely, lookup_key_ptr_likely);
    // SAFETY: the callees are the game's own; the pointer is passed through unchanged.
    unsafe {
        let found_entry_likely = mech_trace::cr!(
            "10042730",
            "10042370",
            FUN_10042370(lookup_id_likely, lookup_key_ptr_likely)
        );
        if !found_entry_likely.is_null() {
            FUN_100423d0(found_entry_likely);
        }
    }
}
