// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f40(entry: *mut i32) -> *mut i32 {
    if entry.is_null() {
        return core::ptr::null_mut();
    }
    let mut entry = entry;
    // SAFETY: entry points at the game's array of 0xe-dword records, walked
    // until a record whose first field is -1, as the original does.
    unsafe {
        let mut first_field = entry.read_unaligned();
        loop {
            if first_field == -1 {
                return core::ptr::null_mut();
            }
            if 0 < entry.add(0xb).read_unaligned() {
                break;
            }
            first_field = entry.add(0xe).read_unaligned();
            entry = entry.add(0xe);
        }
    }
    entry
}
