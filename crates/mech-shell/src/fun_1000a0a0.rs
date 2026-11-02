// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008379c: u32;
    static mut DAT_10083800: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a0a0(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own table; the pointer walk stays
    // between the table start and the end marker as in the original.
    unsafe {
        let base = (&raw mut DAT_1008379c) as *mut i32;
        let end = (&raw mut DAT_10083800) as *mut i32;
        let mut i_var3: i32 = 0;
        let mut pi_var1: *mut i32 = base;
        let mut i_var2: i32 = param_1.wrapping_sub(param_1 % 100).wrapping_add(1);
        loop {
            let v = pi_var1.read_unaligned();
            if v == -1 {
                if 0x14 < i_var2 % 100 {
                    return 0;
                }
                base.offset(i_var3 as isize).write_unaligned(i_var2);
                return i_var2;
            }
            if i_var2 / -100 == v / -100 {
                i_var2 = i_var2.wrapping_add(1);
            }
            pi_var1 = pi_var1.wrapping_add(1);
            i_var3 = i_var3.wrapping_add(1);
            if !(pi_var1 < end) {
                break;
            }
        }
        0
    }
}
