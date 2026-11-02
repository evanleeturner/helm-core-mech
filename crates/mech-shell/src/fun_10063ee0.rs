// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063ee0(param_1: *mut i32) {
    mech_trace::enter!("10063ee0", param_1);
    let mut entry_cursor = param_1;
    if !entry_cursor.is_null() {
        // SAFETY: the caller passes a list of 0x38-byte entries terminated by
        // an entry whose first field is -1; reads and writes stay within it.
        unsafe {
            let mut entry_first_field = mech_trace::rd!("10063ee0", entry_cursor, read_unaligned);
            while entry_first_field != -1 {
                mech_trace::wr!("10063ee0", entry_cursor.byte_add(0x2c), write_unaligned, 0);
                entry_cursor = entry_cursor.byte_add(0x38);
                entry_first_field = mech_trace::rd!("10063ee0", entry_cursor, read_unaligned);
            }
        }
    }
}
