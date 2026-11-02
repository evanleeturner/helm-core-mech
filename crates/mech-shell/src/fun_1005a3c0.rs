// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6aa6: u8;
    static mut DAT_100c6aca: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005a230(slot_index: i32, entry_name: *mut i8) -> u16;
    fn FUN_1005a2d0(slot_index: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a3c0(
    slot_index: i32,
    entry_name: *mut i8,
    item_index: u16,
) -> i32 {
    mech_trace::enter!("1005a3c0", slot_index, entry_name, item_index);
    if item_index == 0 {
        return mech_trace::rt!("1005a3c0", -1);
    }
    // SAFETY: the callees and globals are the game's own; the computed
    // addresses point into the game's slot tables, read where the original reads.
    unsafe {
        let lookup = mech_trace::cr!("1005a3c0", "1005a230", FUN_1005a230(slot_index, entry_name));
        if (lookup as i16) < 0 {
            return mech_trace::rt!("1005a3c0", -1);
        }
        let entry_index = (lookup as i16) as i32;
        let off = slot_index.wrapping_mul(0x14a);
        let a_addr = (&raw mut DAT_100c6aca as usize as u32)
            .wrapping_add(entry_index.wrapping_mul(8) as u32)
            .wrapping_add(off as u32) as usize as *const i32;
        if mech_trace::rd!("1005a3c0", a_addr, read_unaligned) == 0 {
            let status = mech_trace::cr!("1005a3c0", "1005a2d0", FUN_1005a2d0(slot_index));
            if status as i32 == -1 {
                return mech_trace::rt!("1005a3c0", -1);
            }
        }
        let table = mech_trace::rd!("1005a3c0", a_addr, read_unaligned) as u32;
        let item_addr = table
            .wrapping_add(0x1a)
            .wrapping_add((item_index as u32).wrapping_mul(8)) as usize
            as *const i32;
        let item = mech_trace::rd!("1005a3c0", item_addr, read_unaligned);
        let b_addr =
            (&raw mut DAT_100c6aa6 as usize as u32).wrapping_add(off as u32) as usize as *const i32;
        let base_ptr = mech_trace::rd!("1005a3c0", b_addr, read_unaligned) as u32;
        let base_addr = base_ptr
            .wrapping_add(0x22)
            .wrapping_add(entry_index.wrapping_mul(0x18) as u32) as usize
            as *const u16;
        let base = mech_trace::rd!("1005a3c0", base_addr, read_unaligned);
        let result = item.wrapping_sub(base as u32 as i32);
        if result < 1 {
            return mech_trace::rt!("1005a3c0", 0);
        }
        mech_trace::rt!("1005a3c0", result)
    }
}
