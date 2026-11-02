// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000b7f0(record: i32, base_number_likely: i32) -> i32 {
    let mut new_number = base_number_likely.wrapping_sub(base_number_likely % 100);
    let mut suffix: i32 = 0;
    'outer: loop {
        suffix = suffix.wrapping_add(1);
        let mut row_ptr = record.wrapping_add(0x368) as *const i32;
        let mut rows_left: i32 = 8;
        loop {
            let mut columns_left: i32 = 0xc;
            let mut cursor = row_ptr;
            loop {
                // SAFETY: record points at the game's record; the grid is 8 rows of 12 ints.
                let v = unsafe { cursor.read_unaligned() };
                if (10000i32.wrapping_sub(v)) / 100 == (10000i32.wrapping_sub(new_number)) / 100
                    && v % 100 == suffix
                {
                    continue 'outer;
                }
                cursor = cursor.wrapping_add(1);
                columns_left -= 1;
                if columns_left == 0 {
                    break;
                }
            }
            row_ptr = row_ptr.wrapping_add(0xc);
            rows_left -= 1;
            if rows_left == 0 {
                break 'outer;
            }
        }
    }
    new_number = new_number.wrapping_add(suffix);
    suffix = 0;
    let mut cursor = record.wrapping_add(0x28c) as *const i32;
    loop {
        // SAFETY: the slot table at record + 0x28c holds 0x19 ints.
        let v = unsafe { cursor.read_unaligned() };
        if v == -1 {
            if 0x14 < new_number % 100 {
                return 0;
            }
            // SAFETY: suffix is a valid slot index within the table.
            unsafe {
                (record
                    .wrapping_add(0x28c)
                    .wrapping_add(suffix.wrapping_mul(4)) as *mut i32)
                    .write_unaligned(new_number);
            }
            return new_number;
        }
        cursor = cursor.wrapping_add(1);
        suffix = suffix.wrapping_add(1);
        if suffix >= 0x19 {
            break;
        }
    }
    0
}
