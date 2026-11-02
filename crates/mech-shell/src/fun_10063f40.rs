// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f40(entry: *mut i32) -> *mut i32 {
    mech_trace::enter!("10063f40", entry);
    if entry.is_null() {
        return mech_trace::rt!("10063f40", core::ptr::null_mut());
    }
    let mut entry = entry;
    // SAFETY: entry points at the game's array of 0xe-dword records, walked
    // until a record whose first field is -1, as the original does.
    unsafe {
        let mut first_field = mech_trace::rd!("10063f40", entry, read_unaligned);
        loop {
            if first_field == -1 {
                return mech_trace::rt!("10063f40", core::ptr::null_mut());
            }
            if 0 < mech_trace::rd!("10063f40", entry.add(0xb), read_unaligned) {
                break;
            }
            first_field = mech_trace::rd!("10063f40", entry.add(0xe), read_unaligned);
            entry = entry.add(0xe);
        }
    }
    mech_trace::rt!("10063f40", entry)
}
