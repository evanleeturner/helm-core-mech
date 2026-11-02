// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a27a(param_1: i32, param_2: *mut u32) -> i32 {
    // SAFETY: param_1 is the address of the caller's structure and param_2 is
    // either null or points at the caller's output array; reads and writes
    // happen where the original does.
    unsafe {
        let mut i_var1: i32 = ((param_1.wrapping_add(4)) as *const i32).read_unaligned();
        let pi_var3: i32 = param_1.wrapping_add(8);
        let mut i_var2: i32 = 1;
        let mut pi_var4: i32 = pi_var3;
        let mut out = param_2;
        if !out.is_null() {
            out.write_unaligned(0);
            out = out.add(1);
        }
        'outer: loop {
            i_var1 = i_var1.wrapping_sub(1);
            if i_var1 == 0 {
                return i_var2;
            }
            pi_var4 = pi_var4.wrapping_add(8);
            let mut pi_var5: i32 = pi_var3;
            loop {
                if (pi_var4 as *const i32).read_unaligned()
                    == (pi_var5 as *const i32).read_unaligned()
                {
                    continue 'outer;
                }
                pi_var5 = pi_var5.wrapping_add(8);
                if !(pi_var5 < pi_var4) {
                    break;
                }
            }
            if !out.is_null() {
                out.write_unaligned((pi_var4.wrapping_sub(pi_var3) as u32) >> 3);
                out = out.add(1);
            }
            i_var2 = i_var2.wrapping_add(1);
        }
    }
}
