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
    mech_trace::enter!("10009840", source);
    let mut key_prefix: [u8; 4] = [0; 4];
    let mut entry_index: usize = 0;
    // SAFETY: the table and the string helper are the game's own; source
    // points at the caller's NUL-terminated string, read as strncpy does.
    unsafe {
        let mut entry_cursor = (&raw mut DAT_10083d8c) as *mut *mut i8;
        let mut i = 0usize;
        while i < 3 {
            let c = mech_trace::rd!("10009840", source.byte_add(i).cast::<u8>(), read);
            key_prefix[i] = c;
            i += 1;
            if c == 0 {
                break;
            }
        }
        key_prefix[3] = 0;
        loop {
            let mut table_cursor = mech_trace::cr!(
                "10009840",
                "1005bbb0",
                FUN_1005bbb0(mech_trace::rd!("10009840", entry_cursor, read_unaligned))
            ) as *const u8;
            let mut search_cursor = mech_trace::cr!(
                "10009840",
                "1005bbb0",
                FUN_1005bbb0(key_prefix.as_mut_ptr() as *mut i8)
            ) as *const u8;
            let equal;
            loop {
                let a = mech_trace::rd!("10009840", table_cursor, read);
                let b = mech_trace::rd!("10009840", search_cursor, read);
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
                return mech_trace::rt!(
                    "10009840",
                    mech_trace::rd!(
                        "10009840",
                        ((&raw mut DAT_10083d94) as *const *mut u8)
                            .add(entry_index.wrapping_mul(10)),
                        read_unaligned
                    )
                );
            }
            entry_cursor = entry_cursor.byte_add(40);
            entry_index = entry_index.wrapping_add(1);
            if 0x100848a3usize < entry_cursor as usize {
                return mech_trace::rt!("10009840", core::ptr::null_mut());
            }
        }
    }
}
