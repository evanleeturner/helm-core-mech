// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083d8c: u8;
    static mut DAT_10083d94: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(source: *mut i8) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009840(source: *mut i8) -> *mut u8 {
    let mut key_prefix: [u8; 4] = [0; 4];
    let mut entry_index: usize = 0;
    // SAFETY: the table and the string helper are the game's own; source
    // points at the caller's NUL-terminated string, read as strncpy does.
    unsafe {
        let mut entry_cursor = (&raw mut DAT_10083d8c) as *mut *mut i8;
        let mut i = 0usize;
        while i < 3 {
            let c = source.byte_add(i).cast::<u8>().read();
            key_prefix[i] = c;
            i += 1;
            if c == 0 {
                break;
            }
        }
        key_prefix[3] = 0;
        loop {
            let mut table_cursor = FUN_1005bbb0(entry_cursor.read_unaligned()) as *const u8;
            let mut search_cursor = FUN_1005bbb0(key_prefix.as_mut_ptr() as *mut i8) as *const u8;
            let equal;
            loop {
                let a = table_cursor.read();
                let b = search_cursor.read();
                if a != b {
                    equal = false;
                    break;
                }
                if a == 0 {
                    equal = true;
                    break;
                }
                table_cursor = table_cursor.add(1);
                search_cursor = search_cursor.add(1);
            }
            if equal {
                return ((&raw mut DAT_10083d94) as *const *mut u8)
                    .add(entry_index.wrapping_mul(10))
                    .read_unaligned();
            }
            entry_cursor = entry_cursor.byte_add(40);
            entry_index = entry_index.wrapping_add(1);
            if 0x100848a3usize < entry_cursor as usize {
                return core::ptr::null_mut();
            }
        }
    }
}
