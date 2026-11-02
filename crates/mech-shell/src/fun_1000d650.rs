// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083d8c: u8;
    static mut DAT_10083da8: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncpy(dest: *mut i8, source: *const i8, count: usize) -> *mut i8;
    fn FUN_1005bbb0(record_base: *mut i8) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d650(param_1: i32) -> u32 {
    mech_trace::enter!("1000d650", param_1);
    let mut code_text: [i8; 4] = [0; 4];
    let mut row_index: i32 = 0;
    // SAFETY: the table, its rows, and the helper functions are the game's own
    // memory; param_1 is the caller's record address, read where the original does.
    unsafe {
        let mut table_row: *const *mut i8 = (&raw mut DAT_10083d8c).cast::<*mut i8>();
        mech_trace::cr!(
            "1000d650",
            "strncpy",
            strncpy(
                code_text.as_mut_ptr(),
                (param_1.wrapping_add(0x81c)) as *const i8,
                3,
            )
        );
        code_text[3] = 0;
        loop {
            let mut entry_cursor = mech_trace::cr!(
                "1000d650",
                "1005bbb0",
                FUN_1005bbb0(mech_trace::rd!("1000d650", table_row, read_unaligned))
            ) as *const u8;
            let mut code_cursor =
                mech_trace::cr!("1000d650", "1005bbb0", FUN_1005bbb0(code_text.as_mut_ptr()))
                    as *const u8;
            let equal: bool;
            loop {
                let mut entry_char = mech_trace::rd!("1000d650", entry_cursor, read);
                if entry_char != mech_trace::rd!("1000d650", code_cursor, read) {
                    equal = false;
                    break;
                }
                if entry_char == 0 {
                    equal = true;
                    break;
                }
                entry_char = mech_trace::rd!("1000d650", entry_cursor.add(1), read);
                if entry_char != mech_trace::rd!("1000d650", code_cursor.add(1), read) {
                    equal = false;
                    break;
                }
                entry_cursor = entry_cursor.add(2);
                code_cursor = code_cursor.add(2);
                if entry_char == 0 {
                    equal = true;
                    break;
                }
            }
            if equal {
                return mech_trace::rt!(
                    "1000d650",
                    mech_trace::rd!(
                        "1000d650",
                        (&raw mut DAT_10083da8)
                            .cast::<u32>()
                            .offset((row_index.wrapping_mul(10)) as isize),
                        read_unaligned
                    )
                );
            }
            table_row = table_row.add(10);
            row_index = row_index.wrapping_add(1);
            if 0x100848a3usize < table_row as usize {
                return mech_trace::rt!("1000d650", 1);
            }
        }
    }
}
