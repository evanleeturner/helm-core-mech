// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a50f4: u32;
    static mut DAT_100abbbc: u32;
    static mut DAT_100ab9b0: u32;
    static mut DAT_1008f458: [u8; 1];
    static mut DAT_1008f408: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strchr(_Str: *mut i8, _Val: i32) -> *mut i8;
    fn __isctype(_C: i32, _Type: i32) -> i32;
    fn FID_conflict__wprintf(_Format: *mut i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10017b70() -> u32 {
    let mut revision_built = false;
    let mut slots_left: i32 = 4;
    let mut rev_number: u32 = 0;
    // SAFETY: globals, strings and callees are the game's own; cursors walk
    // the game's NUL-terminated string and the local rev_number bytes, as the original does.
    unsafe {
        let mut char_cursor: *mut i8 = (&raw mut DAT_1008f408).cast::<i8>();
        let mut read_cursor: *mut i8;
        let mut found = true;
        loop {
            read_cursor = _strchr(char_cursor, 0x24);
            if read_cursor.is_null() {
                found = false;
                break;
            }
            char_cursor = read_cursor.add(1);
            if *read_cursor.add(1) == b'R' as i8 {
                break;
            }
        }
        if found && !char_cursor.is_null() {
            char_cursor = _strchr(char_cursor, 0x3a);
            if !char_cursor.is_null() {
                let mut ctype_flag: u32;
                loop {
                    read_cursor = char_cursor;
                    let c = *read_cursor.add(1);
                    if DAT_100abbbc < 2 {
                        let addr = DAT_100ab9b0.wrapping_add((c as i32).wrapping_mul(2) as u32);
                        ctype_flag = ((addr as *const u16).read_unaligned() & 8) as u32;
                    } else {
                        ctype_flag = __isctype(c as i32, 8) as u32;
                    }
                    char_cursor = read_cursor.add(1);
                    if ctype_flag == 0 {
                        break;
                    }
                }
                read_cursor = read_cursor.add(4);
                let mut out: *mut u8 = (&raw mut rev_number).cast::<u8>().add(3);
                let mut reads_left: i32 = 4;
                loop {
                    let revision_char = *read_cursor;
                    read_cursor = read_cursor.wrapping_sub(1);
                    if DAT_100abbbc < 2 {
                        let addr = DAT_100ab9b0
                            .wrapping_add((revision_char as i32).wrapping_mul(2) as u32);
                        ctype_flag = ((addr as *const u16).read_unaligned() & 4) as u32;
                    } else {
                        ctype_flag = __isctype(revision_char as i32, 4) as u32;
                    }
                    if ctype_flag != 0 || revision_char == b'.' as i8 {
                        *out = revision_char as u8;
                        out = out.wrapping_sub(1);
                        slots_left = slots_left.wrapping_sub(1);
                    }
                    reads_left = reads_left.wrapping_sub(1);
                    if reads_left == 0 {
                        break;
                    }
                }
                if 0 < slots_left {
                    while slots_left != 0 {
                        *out = b' ';
                        out = out.wrapping_sub(1);
                        slots_left = slots_left.wrapping_sub(1);
                    }
                }
                revision_built = true;
            }
        }
        if !revision_built {
            FID_conflict__wprintf((&raw mut DAT_1008f458).cast::<i8>());
            DAT_100a50f4 = 1;
        }
        rev_number
    }
}
