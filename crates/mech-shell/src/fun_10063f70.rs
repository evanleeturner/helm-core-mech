// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f70(
    entry: *mut i32,
    point_x_likely: i32,
    point_y_likely: i32,
) -> *mut i32 {
    let mut entry = entry;
    if !entry.is_null() {
        // SAFETY: entry points at the game's array of 0x38-byte entries,
        // terminated by an entry whose first field is -1.
        unsafe {
            let mut first_field = entry.read_unaligned();
            while first_field != -1 {
                let e0 = entry.read_unaligned();
                if entry.add(8).read_unaligned() != 0
                    && e0 <= point_x_likely
                    && point_x_likely < entry.add(2).read_unaligned().wrapping_add(e0)
                {
                    let e1 = entry.add(1).read_unaligned();
                    if e1 <= point_y_likely
                        && point_y_likely < entry.add(3).read_unaligned().wrapping_add(e1)
                    {
                        return entry;
                    }
                }
                entry = entry.add(0xe);
                first_field = entry.read_unaligned();
            }
        }
    }
    core::ptr::null_mut()
}
