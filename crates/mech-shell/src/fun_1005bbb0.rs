// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _toupper(_C: i32) -> i32;
}

unsafe fn str_len(text: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: text points at a NUL-terminated string owned by the caller.
    unsafe {
        while *text.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005bbb0(text: *mut i8) -> *mut i8 {
    // SAFETY: text is the caller's NUL-terminated buffer; it is read and
    // written in place at the same indices as the original.
    unsafe {
        let len = str_len(text);
        let mut char_index: u32 = 0;
        if len != 0 {
            loop {
                let next_index = char_index.wrapping_add(1);
                let up = _toupper(*text.add(char_index as usize) as i32);
                *text.add(char_index as usize) = up as i8;
                let l = str_len(text);
                char_index = next_index;
                if !(next_index < l) {
                    break;
                }
            }
        }
    }
    text
}
