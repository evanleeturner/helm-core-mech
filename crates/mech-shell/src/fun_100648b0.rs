// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(test_arg1: i32, test_arg2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10064ca0(list_owner_likely: *mut u8, test_arg1: i32, test_arg2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_100648b0(
    list_owner_likely: *mut u8,
    test_arg1: i32,
    test_arg2: i32,
) -> u32 {
    mech_trace::enter!("100648b0", list_owner_likely, test_arg1, test_arg2);
    let mut found_value: u32 = 0xffff_ffff;
    let mut entry_index: i32 = 0;
    // SAFETY: list_owner_likely points at the game's list owner, whose first
    // field is a pointer to a collection with a count at offset 4; entries
    // returned by the game's lookup are read at the offsets the original uses.
    unsafe {
        let count_of = || {
            let coll = mech_trace::rd!("100648b0", list_owner_likely.cast::<i32>(), read_unaligned);
            mech_trace::rd!(
                "100648b0",
                (coll as usize as *const u8).byte_add(4).cast::<i32>(),
                read_unaligned
            )
        };
        if 0 < count_of() {
            loop {
                let coll =
                    mech_trace::rd!("100648b0", list_owner_likely.cast::<i32>(), read_unaligned);
                let entry = mech_trace::cr!("100648b0", "10018060", FUN_10018060(coll, entry_index))
                    as usize as *mut u8;
                let test_result = mech_trace::cr!(
                    "100648b0",
                    "10064ca0",
                    FUN_10064ca0(entry, test_arg1, test_arg2)
                );
                if (test_result as u8) == 1
                    && mech_trace::rd!("100648b0", entry.byte_add(0x29), read) == 1
                {
                    found_value = mech_trace::rd!(
                        "100648b0",
                        entry.byte_add(0x24).cast::<u32>(),
                        read_unaligned
                    );
                }
                entry_index = entry_index.wrapping_add(1);
                if !(entry_index < count_of()) {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("100648b0", found_value)
}
