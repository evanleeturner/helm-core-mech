// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032e00(param_1: i32) -> u32 {
    let mut pi_var2 = param_1.wrapping_add(0x368) as *const i32;
    let mut i_var1: i32 = 0;
    let mut i_var5: i32 = 0;
    loop {
        let mut i_var3: i32 = 0;
        let mut pi_var4 = pi_var2;
        loop {
            // SAFETY: param_1 points at the caller's structure, whose table of
            // 8 rows of 12 ints at offset 0x368 is read as the original does.
            i_var1 = unsafe { pi_var4.read_unaligned() };
            if i_var1 / 10 == 700 {
                return 0x201;
            }
            pi_var4 = pi_var4.wrapping_add(1);
            i_var3 = i_var3.wrapping_add(1);
            if i_var3 >= 0xc {
                break;
            }
        }
        pi_var2 = pi_var2.wrapping_add(0xc);
        i_var5 = i_var5.wrapping_add(1);
        if i_var5 >= 8 {
            break;
        }
    }
    ((i_var1 / 10) as u32) & 0xffffff00
}
