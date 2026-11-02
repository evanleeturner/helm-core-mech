// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2dec: u32;
    static mut DAT_100c2be0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __isctype(_c: i32, _type: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d6e0(param_1: *mut i8) {
    // SAFETY: param_1 points at a NUL-terminated string the caller owns; the
    // globals and __isctype are the game's own, and the table pointer is read
    // where the original reads it.
    unsafe {
        let mut local_c: *mut i8 = param_1;
        let mut local_8: *mut i8;
        let mut p_var1: *mut i8 = param_1;
        loop {
            let mut local_10: u32;
            loop {
                local_8 = p_var1;
                if *local_c == 0 {
                    *local_8 = 0;
                    return;
                }
                *local_8 = *local_c;
                local_c = local_c.add(1);
                if DAT_100c2dec < 2 {
                    let idx = (*local_c as i32).wrapping_mul(2);
                    let addr = DAT_100c2be0.wrapping_add(idx as u32) as *const u16;
                    local_10 = (addr.read_unaligned() as u32) & 8;
                } else {
                    local_10 = __isctype(*local_c as i32, 8) as u32;
                }
                p_var1 = local_8.add(1);
                if local_10 != 0 {
                    break;
                }
            }
            while *local_c != 0 {
                let local_14: u32;
                if DAT_100c2dec < 2 {
                    let idx = (*local_c as i32).wrapping_mul(2);
                    let addr = DAT_100c2be0.wrapping_add(idx as u32) as *const u16;
                    local_14 = (addr.read_unaligned() as u32) & 8;
                } else {
                    local_14 = __isctype(*local_c as i32, 8) as u32;
                }
                if local_14 == 0 {
                    break;
                }
                local_c = local_c.add(1);
            }
            *local_8.add(1) = b' ' as i8;
            p_var1 = local_8.add(2);
        }
    }
}
