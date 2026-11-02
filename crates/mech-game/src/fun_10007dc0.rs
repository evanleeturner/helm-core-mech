// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7148: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006af90(queue_entry: u32) -> u8;
    fn FUN_100108a0(queue_entry: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007dc0(queue_entry: *mut u32) -> u32 {
    mech_trace::enter!("10007dc0", queue_entry);
    // SAFETY: queue_entry points at the caller's queue entry; the fields are
    // read and written where the original does, and the callees and the
    // global string are the game's own.
    unsafe {
        let next_entry = mech_trace::rd!("10007dc0", queue_entry.byte_add(0x4f), read_unaligned);
        mech_trace::wr!("10007dc0", queue_entry, write_unaligned, 0);
        mech_trace::wr!("10007dc0", queue_entry.byte_add(0x4f), write_unaligned, 0);
        if mech_trace::rd!("10007dc0", queue_entry.add(3), read_unaligned) != 0 {
            mech_trace::cr!(
                "10007dc0",
                "1006af90",
                FUN_1006af90(mech_trace::rd!(
                    "10007dc0",
                    queue_entry.add(3),
                    read_unaligned
                ))
            );
        }
        if mech_trace::rd!("10007dc0", queue_entry.add(4), read_unaligned) != 0 {
            mech_trace::cr!(
                "10007dc0",
                "100108a0",
                FUN_100108a0(0x20, &raw mut DAT_100a7148)
            );
        }
        mech_trace::rt!("10007dc0", next_entry)
    }
}
