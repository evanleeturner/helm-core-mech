// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10042370(lookup_id: i32, lookup_key_ptr: *mut i32) -> *mut i16;
    fn FUN_100421c0(lookup_id: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042590(lookup_id_likely: i32, lookup_key_ptr_likely: *mut i32) {
    mech_trace::enter!("10042590", lookup_id_likely, lookup_key_ptr_likely);
    // SAFETY: the callees are the game's own; the pointer is passed through
    // unchanged and the found entry's address is handed on as an integer.
    unsafe {
        let found_entry_likely = mech_trace::cr!(
            "10042590",
            "10042370",
            FUN_10042370(lookup_id_likely, lookup_key_ptr_likely)
        );
        if !found_entry_likely.is_null() {
            FUN_100421c0(found_entry_likely as usize as i32);
        }
    }
}
