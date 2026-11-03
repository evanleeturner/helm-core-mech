// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6aa0: u8;
    static mut DAT_100c6aa6: u8;
    static mut DAT_100c6aca: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005a230(slot_index: i32, entry_name: *mut i8) -> u16;
    fn FUN_1005a2d0(slot_index: i32) -> u32;
    fn FUN_10074a70(file_handle: i32, buf: *mut u8, max_char_count: u32) -> i32;
    fn FUN_10059f10(slot_index: i32, entry_name: *mut u8, item_index: u32) -> u32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller supplies an address in the game's own memory.
    unsafe { mech_trace::rd!("1005a570", (addr as usize as *const i32), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a570(
    slot_index: i32,
    entry_name: *mut i8,
    item_index: u32,
    data_buffer: *mut u8,
) -> i32 {
    mech_trace::enter!("1005a570", slot_index, entry_name, item_index, data_buffer);
    // SAFETY: the callees and globals are the game's own; all addresses
    // computed here are offsets into the game's slot tables, read as the
    // original does.
    unsafe {
        let lookup = mech_trace::cr!("1005a570", "1005a230", FUN_1005a230(slot_index, entry_name));
        if lookup == 0xffff {
            return mech_trace::rt!("1005a570", -1);
        }
        let entry_index = (lookup as i16) as i32;
        let slot_offset = slot_index.wrapping_mul(0x14a);
        let a0 = (&raw mut DAT_100c6aa0) as usize as i32;
        let a6 = (&raw mut DAT_100c6aa6) as usize as i32;
        let ca = (&raw mut DAT_100c6aca) as usize as i32;
        let tbl = ca
            .wrapping_add(entry_index.wrapping_mul(8))
            .wrapping_add(slot_offset);
        if rd(tbl) == 0 {
            let status = mech_trace::cr!("1005a570", "1005a2d0", FUN_1005a2d0(slot_index));
            if status == 0xffff_ffff {
                return mech_trace::rt!("1005a570", -1);
            }
        }
        let base_addr = rd(a6.wrapping_add(slot_offset))
            .wrapping_add(0x22)
            .wrapping_add(entry_index.wrapping_mul(0x18));
        let entry_base = mech_trace::rd!(
            "1005a570",
            (base_addr as usize as *const u16),
            read_unaligned
        ) as u32;
        let item_off = ((item_index & 0xffff) as i32).wrapping_mul(8);
        let item_count =
            (rd(rd(tbl).wrapping_add(0x1a).wrapping_add(item_off)) as u32).wrapping_sub(entry_base);
        let handle = rd(a0.wrapping_add(slot_offset));
        let pos =
            rd(rd(tbl).wrapping_add(0x16).wrapping_add(item_off)).wrapping_add(entry_base as i32);
        let r = mech_trace::cr!(
            "1005a570",
            "10074a70",
            FUN_10074a70(handle, pos as usize as *mut u8, 0)
        );
        if r == -1 {
            return mech_trace::rt!("1005a570", -1);
        }
        let handle2 = rd(a0.wrapping_add(slot_offset));
        let got = mech_trace::cr!(
            "1005a570",
            "10059f10",
            FUN_10059f10(handle2, data_buffer, item_count)
        );
        mech_trace::rt!("1005a570", ((got == item_count) as i32).wrapping_sub(1))
    }
}
