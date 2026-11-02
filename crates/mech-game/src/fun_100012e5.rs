// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2dec: i32;
    static mut PTR_DAT_100c2be0: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __isctype(c: i32, mask: i32) -> i32;
}

unsafe fn check_space(c: i8) -> bool {
    // SAFETY: the globals and the callee are the game's own; the table
    // pointer indexes the game's ctype table as the original does.
    unsafe {
        if DAT_100c2dec < 2 {
            let off = (c as i32).wrapping_mul(2) as u32;
            let v = PTR_DAT_100c2be0
                .byte_add(off as usize)
                .cast::<u16>()
                .read_unaligned();
            (v & 8) != 0
        } else {
            __isctype(c as i32, 8) != 0
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100012e5(param_1: *mut i8) -> *mut i8 {
    let mut p = param_1;
    if !p.is_null() {
        // SAFETY: p points at the caller's NUL-terminated string, read and
        // written where the original does.
        unsafe {
            while check_space(*p) {
                p = p.add(1);
            }
            let mut len: i32 = 0;
            while *p.offset(len as isize) != 0 {
                len = len.wrapping_add(1);
            }
            let mut i_var1 = len;
            let mut local_8;
            loop {
                local_8 = i_var1;
                i_var1 = local_8.wrapping_sub(1);
                if local_8 < 1 {
                    break;
                }
                if !check_space(*p.offset(i_var1 as isize)) {
                    break;
                }
            }
            *p.offset(local_8 as isize) = 0;
        }
    }
    p
}
