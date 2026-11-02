// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000aa80(record: i32) {
    mech_trace::enter!("1000aa80", record);
    let mut slots_left: i32 = 0x19;
    let mut slot_ptr: *const i32 = record.wrapping_add(0x28c) as *const i32;
    let total = record.wrapping_add(0x268) as *mut i32;
    // SAFETY: record is the address of the game's record; the total field and
    // the 25 slot fields lie within it and are read and written as the original does.
    unsafe {
        mech_trace::wr!("1000aa80", total, write_unaligned, 0);
        loop {
            if mech_trace::rd!("1000aa80", slot_ptr, read_unaligned) != -1 {
                mech_trace::wr!(
                    "1000aa80",
                    total,
                    write_unaligned,
                    mech_trace::rd!("1000aa80", total, read_unaligned).wrapping_add(100)
                );
            }
            slot_ptr = slot_ptr.wrapping_add(1);
            slots_left = slots_left.wrapping_sub(1);
            if slots_left == 0 {
                break;
            }
        }
    }
}
