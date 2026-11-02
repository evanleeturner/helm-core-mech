// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __write(_FileHandle: i32, _Buf: *mut u8, _MaxCharCount: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001dfc3(file_handle: i32, param_2: *mut u8, param_3: u32) -> u32 {
    let mut local_c: u32 = param_3;
    let mut local_8: *mut u8 = param_2;
    loop {
        if local_c == 0 {
            return param_3;
        }
        let mut max_char_count: u32 = local_c;
        if 0x3fff < local_c {
            max_char_count = 0x4000;
        }
        // SAFETY: __write is the game's own C runtime function; local_8 points
        // into the caller's buffer with at least local_c bytes remaining.
        let u_var1 = unsafe { __write(file_handle, local_8, max_char_count) } as u32;
        if u_var1 != max_char_count {
            break;
        }
        local_c = local_c.wrapping_sub(max_char_count);
        local_8 = local_8.wrapping_add(max_char_count as usize);
    }
    0xffffffff
}
