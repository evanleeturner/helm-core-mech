// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6aa6: u8;
}

unsafe extern "C" {
    fn strncmp(str1: *const i8, str2: *const i8, max_count: usize) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a230(slot_index: i32, str2: *const i8) -> u16 {
    mech_trace::enter!("1005a230", slot_index, str2);
    let mut found = false;
    let mut entry_index: u32 = 0;
    let mut result_index: u16 = 0;
    // SAFETY: the global table and the entry data it points at are the game's
    // own memory; reads happen where the original does, str2 is the caller's string.
    unsafe {
        let slot_ptr = (&raw mut DAT_100c6aa6)
            .cast::<u8>()
            .byte_add(slot_index.wrapping_mul(0x14a) as usize);
        let entry_table: i32 = mech_trace::rd!("1005a230", slot_ptr.cast::<i32>(), read_unaligned);
        let entry_count: u16 = mech_trace::rd!(
            "1005a230",
            (entry_table.wrapping_add(0xc) as usize as *const u16),
            read_unaligned
        );
        if entry_count != 0 {
            loop {
                let addr = entry_table
                    .wrapping_add(0xe)
                    .wrapping_add(entry_index.wrapping_mul(0x18) as i32);
                let cmp = mech_trace::cr!(
                    "1005a230",
                    "strncmp",
                    strncmp(addr as usize as *const i8, str2, 4)
                );
                if cmp == 0 {
                    found = true;
                } else {
                    entry_index = (entry_index as u16).wrapping_add(1) as u32;
                }
                result_index = entry_index as u16;
                if !(!found && result_index < entry_count) {
                    break;
                }
            }
        }
    }
    if !found {
        return mech_trace::rt!("1005a230", 0xffff);
    }
    mech_trace::rt!("1005a230", result_index)
}
