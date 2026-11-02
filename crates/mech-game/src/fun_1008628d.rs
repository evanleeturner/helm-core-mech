// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ba7bc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008628d(entry_id: u32, four_char_key: *mut i32) -> *mut u16 {
    if (entry_id as i32) < 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: the key points at the caller's four bytes; the table global and
    // the chain entries are the game's own memory, read where the original does.
    unsafe {
        let kb = four_char_key.cast::<i8>();
        let sum = (kb.byte_add(2).read() as i32)
            .wrapping_add(kb.byte_add(3).read() as i32)
            .wrapping_add(kb.read() as i32)
            .wrapping_add(kb.byte_add(1).read() as i32)
            .wrapping_add(entry_id as i32);
        let idx = sum % 0x3f1;
        let slot = DAT_100ba7bc.wrapping_add((idx as u32).wrapping_mul(4)) as *const *mut u16;
        let mut entry: *mut u16 = slot.read_unaligned();
        while !entry.is_null() {
            if u32::from(entry.read_unaligned()) == entry_id
                && entry.byte_add(4).cast::<i32>().read_unaligned()
                    == four_char_key.read_unaligned()
            {
                break;
            }
            entry = entry.byte_add(8).cast::<*mut u16>().read_unaligned();
        }
        entry
    }
}
