// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007b890(param_1: u32, param_2: *mut i8) -> i32;
    fn FUN_1007bac8(param_1: u32) -> *mut u8;
    fn FUN_10070f83(param_1: i32, param_2: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007b9d2(param_1: u32, param_2: *mut i8) -> i32 {
    let mut local_84: [i8; 128] = [0; 128];
    // SAFETY: the callees and the global are the game's own; param_2 is a
    // NUL-terminated string from the caller, and the copies write into
    // local_84 exactly where the original does.
    unsafe {
        let mut i_var2 = FUN_1007b890(param_1, param_2);
        if i_var2 == -1 {
            if DAT_100ab2d4 == 0 {
                i_var2 = -1;
            } else {
                // strcpy(local_84, param_2)
                let mut len: usize = 0;
                while *param_2.add(len) != 0 {
                    len += 1;
                }
                len += 1;
                let dst = local_84.as_mut_ptr();
                let mut k = 0usize;
                while k < len {
                    *dst.add(k) = *param_2.add(k);
                    k += 1;
                }
                let src = FUN_1007bac8(param_1);
                let mut slen: usize = 0;
                while *src.add(slen) != 0 {
                    slen += 1;
                }
                slen += 1;
                let mut end: usize = 0;
                while *dst.add(end) != 0 {
                    end += 1;
                }
                let mut k = 0usize;
                while k < slen {
                    *dst.add(end + k) = *src.add(k) as i8;
                    k += 1;
                }
                i_var2 = FUN_10070f83(DAT_100ab2d4 as i32, dst);
                if i_var2 == -1 {
                    i_var2 = -1;
                } else {
                    i_var2 = i_var2.wrapping_add(0xf000);
                }
            }
        }
        i_var2
    }
}
