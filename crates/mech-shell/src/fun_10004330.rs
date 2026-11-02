// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008379c: u8;
    static mut DAT_10083800: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10004330(param_1: i32) -> i32 {
    // SAFETY: the two globals delimit the game's own table of ints; the
    // pointers walk that table and are read and written where the original does.
    unsafe {
        let mut pi_var4 = (&raw mut DAT_1008379c) as *mut i32;
        let mut pi_var3 = pi_var4;
        let end = (&raw mut DAT_10083800) as *mut i32;
        let mut i_var2: i32;
        loop {
            if pi_var4.read_unaligned() == param_1 {
                pi_var4.write_unaligned(-1);
            }
            i_var2 = pi_var4.read_unaligned();
            if i_var2 != -1 {
                if pi_var3.read_unaligned() == -1 {
                    pi_var3.write_unaligned(i_var2);
                    pi_var4.write_unaligned(-1);
                }
                let i_var1 = pi_var3.read_unaligned();
                i_var2 = i_var1 / -100;
                if param_1 / -100 == i_var2 {
                    i_var2 = i_var1 / 100;
                    if param_1 % 100 < i_var1 % 100 {
                        pi_var3.write_unaligned(i_var1.wrapping_sub(1));
                    }
                }
                pi_var3 = pi_var3.add(1);
            }
            pi_var4 = pi_var4.add(1);
            if !(pi_var4 < end) {
                break;
            }
        }
        i_var2
    }
}
