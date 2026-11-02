// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10085170: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a8c0(record: i32) {
    mech_trace::enter!("1000a8c0", record);
    // SAFETY: record is the address of the game's record; its fields at
    // 0x260 and 0x2f0.. are read and written where the original does, and
    // DAT_10085170 is the game's own table.
    unsafe {
        let total = (record.wrapping_add(0x260)) as *mut i32;
        let mut slot_ptr = (record.wrapping_add(0x2f0)) as *const i32;
        let mut slots_left: i32 = 0xe;
        mech_trace::wr!("1000a8c0", total, write_unaligned, 0);
        loop {
            let slot = mech_trace::rd!("1000a8c0", slot_ptr, read_unaligned);
            if slot != -1 {
                let off = (slot / 100).wrapping_mul(0x40);
                let v = mech_trace::rd!(
                    "1000a8c0",
                    (&raw mut DAT_10085170)
                        .byte_offset(off as isize)
                        .cast::<i32>(),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1000a8c0",
                    total,
                    write_unaligned,
                    mech_trace::rd!("1000a8c0", total, read_unaligned).wrapping_add(v)
                );
            }
            slot_ptr = slot_ptr.add(2);
            slots_left = slots_left.wrapping_sub(1);
            if slots_left == 0 {
                break;
            }
        }
    }
}
