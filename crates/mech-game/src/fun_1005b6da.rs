// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b6da(param_1: i32, param_2: *mut u32) -> i32 {
    mech_trace::enter!("1005b6da", param_1, param_2);
    // SAFETY: param_1 is the address of the caller's table; its count and
    // pairs of ints are read as the original does, and param_2 (if non-null)
    // is the caller's output array.
    unsafe {
        let mut i_var1: i32 = mech_trace::rd!(
            "1005b6da",
            ((param_1.wrapping_add(4)) as usize as *const i32),
            read_unaligned
        );
        let pi_var3: *const i32 = (param_1.wrapping_add(8)) as usize as *const i32;
        let mut i_var2: i32 = 1;
        let mut pi_var4: *const i32 = pi_var3;
        let mut param_2 = param_2;
        if !param_2.is_null() {
            mech_trace::wr!("1005b6da", param_2, write_unaligned, 0);
            param_2 = param_2.wrapping_add(1);
        }
        'outer: loop {
            i_var1 = i_var1.wrapping_sub(1);
            if i_var1 == 0 {
                return mech_trace::rt!("1005b6da", i_var2);
            }
            pi_var4 = pi_var4.wrapping_add(2);
            let mut pi_var5: *const i32 = pi_var3;
            loop {
                if mech_trace::rd!("1005b6da", pi_var4, read_unaligned)
                    == mech_trace::rd!("1005b6da", pi_var5, read_unaligned)
                {
                    continue 'outer;
                }
                pi_var5 = pi_var5.wrapping_add(2);
                if !((pi_var5 as usize as i32) < (pi_var4 as usize as i32)) {
                    break;
                }
            }
            if !param_2.is_null() {
                let d = (pi_var4 as usize as i32).wrapping_sub(pi_var3 as usize as i32);
                mech_trace::wr!("1005b6da", param_2, write_unaligned, (d as u32) >> 3);
                param_2 = param_2.wrapping_add(1);
            }
            i_var2 = i_var2.wrapping_add(1);
        }
    }
}
