// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083878: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a590(param_1: i32) -> i32 {
    let mut i_var1: i32 = 0;
    if 0 < param_1 {
        // SAFETY: DAT_10083878 is the game's own table of 8*12 i32 entries;
        // each is read and possibly cleared in the original's order.
        unsafe {
            let mut pi_var4 = (&raw mut DAT_10083878).cast::<i32>();
            let mut i_var2: i32 = 8;
            loop {
                let mut i_var3: i32 = 0xc;
                loop {
                    if pi_var4.read_unaligned() == param_1 {
                        i_var1 = i_var1.wrapping_add(1);
                        pi_var4.write_unaligned(0);
                    }
                    pi_var4 = pi_var4.add(1);
                    i_var3 = i_var3.wrapping_sub(1);
                    if i_var3 == 0 {
                        break;
                    }
                }
                i_var2 = i_var2.wrapping_sub(1);
                if i_var2 == 0 {
                    break;
                }
            }
        }
    }
    i_var1
}
