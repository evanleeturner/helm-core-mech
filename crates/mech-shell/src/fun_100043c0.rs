// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083800: [i32; 28];
    static mut DAT_10083870: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100043c0(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table is walked
    // in 8-byte steps up to the end marker, as the original does.
    unsafe {
        let base = &raw mut DAT_10083800 as *mut i32;
        let end = &raw mut DAT_10083870 as *mut i32;
        let mut i_var1: i32 = 0;
        let mut pi_var2: *mut i32 = base;
        let mut i_var3: i32 = param_1.wrapping_sub(param_1 % 100).wrapping_add(1);
        loop {
            let v = pi_var2.read_unaligned();
            if v == -1 {
                base.offset((i_var1.wrapping_mul(2)) as isize)
                    .write_unaligned(i_var3);
                return i_var3;
            }
            if i_var3 / -100 == v / -100 {
                i_var3 = i_var3.wrapping_add(1);
            }
            pi_var2 = pi_var2.wrapping_add(2);
            i_var1 = i_var1.wrapping_add(1);
            if pi_var2 >= end {
                break;
            }
        }
        0
    }
}
