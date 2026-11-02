// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10085190: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100503d0(record: *mut i32) {
    if record.is_null() {
        return;
    }
    // SAFETY: record is the caller's non-null record; the table is the game's
    // own memory, indexed exactly as the original does.
    unsafe {
        record.byte_add(0x16).cast::<u32>().write_unaligned(100);
        let idx = record.read_unaligned().wrapping_mul(0x10);
        let table = (&raw mut DAT_10085190) as *const i32;
        let table_value = table.wrapping_offset(idx as isize).read_unaligned();
        record.add(1).write_unaligned(table_value);
        if table_value < 1 {
            record.add(1).write_unaligned(10000);
        }
        record.byte_add(0x12).cast::<u32>().write_unaligned(0);
    }
}
