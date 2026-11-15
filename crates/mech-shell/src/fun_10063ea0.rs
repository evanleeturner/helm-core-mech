// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(export_name = "FUN_10063ea0")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FreeEntryTextItems(entry: *mut i32) {
    mech_trace::enter!("10063ea0", entry);
    let mut param_1 = entry;
    if !param_1.is_null() {
        // SAFETY: entry points at the caller's array of 56-byte records,
        // terminated by a record whose first word is -1; the callees are the
        // game's own.
        unsafe {
            let mut first_field = mech_trace::rd!("10063ea0", param_1, read_unaligned);
            while first_field != -1 {
                let item = mech_trace::rd!("10063ea0", param_1.add(6), read_unaligned) as usize
                    as *mut core::ffi::c_void;
                if !item.is_null() {
                    mech_trace::cr!("10063ea0", "10040680", FUN_10040680(item as usize as i32));
                    mech_trace::cr!("10063ea0", "1006ec30", FUN_1006ec30(item));
                    mech_trace::wr!("10063ea0", param_1.add(6), write_unaligned, 0);
                }
                param_1 = param_1.add(0xe);
                first_field = mech_trace::rd!("10063ea0", param_1, read_unaligned);
            }
        }
    }
}
