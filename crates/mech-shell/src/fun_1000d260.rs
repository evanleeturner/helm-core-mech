// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082cf0: [i32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d260(param_1: i32, param_2: i32) -> i32 {
    if param_1 == 0 {
        return param_2 / 100;
    }
    if param_1 != 1 {
        if param_1 != 2 {
            return -1;
        }
        let mut row_index: i32 = 0x16;
        // SAFETY: the table is the game's own memory; entries are read at
        // the same 12-byte stride and bounds as the original.
        let mut entry: *mut i32 = unsafe { (&raw mut DAT_10082cf0).cast::<i32>() };
        loop {
            // SAFETY: entry lies within the game's table.
            let v = unsafe { entry.read_unaligned() };
            if v / 0x32 == param_2 / 0x32 {
                return row_index;
            }
            entry = entry.wrapping_sub(3);
            row_index = row_index.wrapping_sub(1);
            if (entry as usize) <= 0x10082be7 {
                break;
            }
        }
        return -1;
    }
    param_2.wrapping_sub(10000) / 10000
}
