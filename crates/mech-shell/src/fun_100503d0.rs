// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10085190: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100503d0(record: *mut i32) {
    mech_trace::enter!("100503d0", record);
    if record.is_null() {
        return;
    }
    // SAFETY: record is the caller's non-null record; the table is the game's
    // own memory, indexed exactly as the original does.
    unsafe {
        mech_trace::wr!(
            "100503d0",
            record.byte_add(0x16).cast::<u32>(),
            write_unaligned,
            100
        );
        let idx = mech_trace::rd!("100503d0", record, read_unaligned).wrapping_mul(0x10);
        let table = (&raw mut DAT_10085190) as *const i32;
        let table_value = mech_trace::rd!(
            "100503d0",
            table.wrapping_offset(idx as isize),
            read_unaligned
        );
        mech_trace::wr!("100503d0", record.add(1), write_unaligned, table_value);
        if table_value < 1 {
            mech_trace::wr!("100503d0", record.add(1), write_unaligned, 10000);
        }
        mech_trace::wr!(
            "100503d0",
            record.byte_add(0x12).cast::<u32>(),
            write_unaligned,
            0
        );
    }
}
