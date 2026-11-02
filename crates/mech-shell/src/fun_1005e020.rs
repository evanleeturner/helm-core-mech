// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100986b4: u32;
    static mut PTR_DAT_1009860c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100425b0(table_key: i32, str2: u32, param_3: *mut i32) -> *mut u16;
    fn __strcmpi(str1: *const i8, str2: *const i8) -> i32;
    fn FUN_10042590(table_key: i32, str2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e020(table_key: u32, str2: *const i8) -> i32 {
    let mut found_value: i32 = -1;
    let mut decoded_name: [i8; 12] = [0; 12];
    // SAFETY: the callees and globals are the game's own; the entry table
    // returned by the loader is read where the original reads it.
    unsafe {
        let entry_table =
            FUN_100425b0(DAT_100986b4 as i32, table_key, PTR_DAT_1009860c as *mut i32);
        if !entry_table.is_null() {
            let mut entry_cursor = entry_table.add(6);
            let mut i4: i32 = 0;
            loop {
                let c = entry_cursor.byte_add(i4 as usize).cast::<i8>().read();
                decoded_name[i4 as usize] = c.wrapping_neg();
                i4 += 1;
                if i4 >= 10 {
                    break;
                }
            }
            i4 = 0;
            if 0 < (entry_table.add(4).read_unaligned() as i16) {
                loop {
                    let r = __strcmpi(decoded_name.as_ptr(), str2);
                    if r == 0 {
                        break;
                    }
                    entry_cursor = entry_cursor.add(6);
                    i4 += 1;
                    let mut i2: i32 = 0;
                    loop {
                        let c = entry_cursor.byte_add(i2 as usize).cast::<i8>().read();
                        decoded_name[i2 as usize] = c.wrapping_neg();
                        i2 += 1;
                        if i2 >= 10 {
                            break;
                        }
                    }
                    if i4 >= (entry_table.add(4).read_unaligned() as i16) as i32 {
                        break;
                    }
                }
            }
            if __strcmpi(decoded_name.as_ptr(), str2) == 0 {
                found_value = (entry_cursor.add(5).read_unaligned() as i16) as i32;
            }
            FUN_10042590(table_key as i32, PTR_DAT_1009860c as *mut i32);
        }
    }
    found_value
}
