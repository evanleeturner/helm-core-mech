// SPDX-License-Identifier: MIT

#[unsafe(export_name = "FUN_10063f70")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FindEntryContainingPoint(
    entry: *mut i32,
    point_x: i32,
    point_y: i32,
) -> *mut i32 {
    mech_trace::enter!("10063f70", entry, point_x, point_y);
    let mut entry = entry;
    if !entry.is_null() {
        // SAFETY: entry points at the game's array of 0x38-byte entries,
        // terminated by an entry whose first field is -1.
        unsafe {
            let mut first_field = mech_trace::rd!("10063f70", entry, read_unaligned);
            while first_field != -1 {
                let entry_x = mech_trace::rd!("10063f70", entry, read_unaligned);
                if mech_trace::rd!("10063f70", entry.add(8), read_unaligned) != 0
                    && entry_x <= point_x
                    && point_x
                        < mech_trace::rd!("10063f70", entry.add(2), read_unaligned)
                            .wrapping_add(entry_x)
                {
                    let entry_y = mech_trace::rd!("10063f70", entry.add(1), read_unaligned);
                    if entry_y <= point_y
                        && point_y
                            < mech_trace::rd!("10063f70", entry.add(3), read_unaligned)
                                .wrapping_add(entry_y)
                    {
                        return mech_trace::rt!("10063f70", entry);
                    }
                }
                entry = entry.add(0xe);
                first_field = mech_trace::rd!("10063f70", entry, read_unaligned);
            }
        }
    }
    mech_trace::rt!("10063f70", core::ptr::null_mut())
}
