// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a2dc(param_1: i32, param_2: *mut u32) -> i32 {
    mech_trace::enter!("1006a2dc", param_1, param_2);
    // SAFETY: param_1 is the address of a caller-owned structure whose count
    // sits at offset 4 and whose pairs of ints start at offset 0xc; param_2,
    // when non-null, is a caller buffer receiving the indices.
    unsafe {
        let mut i_var1: i32 = mech_trace::rd!(
            "1006a2dc",
            ((param_1 as usize).wrapping_add(4) as *const i32),
            read_unaligned
        );
        let pi_var3: *const i32 = (param_1 as usize).wrapping_add(0xc) as *const i32;
        let mut i_var2: i32 = 1;
        let mut pi_var4: *const i32 = pi_var3;
        let mut param_2 = param_2;
        if !param_2.is_null() {
            mech_trace::wr!("1006a2dc", param_2, write_unaligned, 0);
            param_2 = param_2.add(1);
        }
        'outer: loop {
            i_var1 = i_var1.wrapping_sub(1);
            if i_var1 == 0 {
                return mech_trace::rt!("1006a2dc", i_var2);
            }
            pi_var4 = pi_var4.byte_add(8);
            let mut pi_var5: *const i32 = pi_var3;
            loop {
                if mech_trace::rd!("1006a2dc", pi_var4, read_unaligned)
                    == mech_trace::rd!("1006a2dc", pi_var5, read_unaligned)
                {
                    continue 'outer;
                }
                pi_var5 = pi_var5.byte_add(8);
                if !((pi_var5 as usize as i32) < (pi_var4 as usize as i32)) {
                    break;
                }
            }
            if !param_2.is_null() {
                mech_trace::wr!(
                    "1006a2dc",
                    param_2,
                    write_unaligned,
                    ((pi_var4 as usize as i32).wrapping_sub(pi_var3 as usize as i32) as u32) >> 3
                );
                param_2 = param_2.add(1);
            }
            i_var2 = i_var2.wrapping_add(1);
        }
    }
}
