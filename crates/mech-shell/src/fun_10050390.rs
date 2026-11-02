// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008518c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050390(record: *mut i32) {
    if !record.is_null() {
        // SAFETY: record points at the caller's record; the table is the game's
        // own memory, indexed by the record's first field as the original does.
        unsafe {
            record.byte_add(0x12).cast::<u32>().write_unaligned(100);
            let idx = record.read_unaligned();
            let base = &raw mut DAT_1008518c as usize;
            let addr = base.wrapping_add(idx.wrapping_mul(0x40) as usize) as *const i32;
            let table_value = addr.read_unaligned();
            record.byte_add(4).write_unaligned(table_value);
            if table_value < 1 {
                record.byte_add(4).write_unaligned(10000);
            }
            record
                .byte_add(10)
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
            record.byte_add(0xe).cast::<u32>().write_unaligned(0);
        }
    }
}
