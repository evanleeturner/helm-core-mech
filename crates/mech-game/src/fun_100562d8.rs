// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100562d8(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: u8,
) -> u32 {
    mech_trace::enter!("100562d8", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 points at the caller's descriptor, whose first field
    // points at a second descriptor; the pixel pointer is read and written
    // where the original does.
    unsafe {
        let pi_var2 = mech_trace::rd!("100562d8", param_1, read_unaligned) as *const i32;
        let i_var3 = mech_trace::rd!("100562d8", pi_var2.byte_add(4), read_unaligned);
        if i_var3.wrapping_add(1) == 0 || i_var3 < -1 {
            return mech_trace::rt!("100562d8", 0xffff_ffff);
        }
        let i_var4 = mech_trace::rd!("100562d8", pi_var2.byte_add(8), read_unaligned);
        if i_var4 < 0 {
            return mech_trace::rt!("100562d8", 0xffff_ffff);
        }
        let mut i_var10 = mech_trace::rd!("100562d8", param_1.byte_add(4), read_unaligned);
        let mut i_var5 = i_var10;
        if i_var10 < 1 {
            i_var5 = 0;
        }
        let mut i_var11 = mech_trace::rd!("100562d8", param_1.byte_add(8), read_unaligned);
        let mut i_var6 = i_var11;
        if i_var11 < 1 {
            i_var6 = 0;
        }
        let mut i_var7 = mech_trace::rd!("100562d8", param_1.byte_add(12), read_unaligned);
        if i_var3 <= i_var7 {
            i_var7 = i_var3;
        }
        let mut i_var8 = mech_trace::rd!("100562d8", param_1.byte_add(16), read_unaligned);
        if i_var4 <= i_var8 {
            i_var8 = i_var4;
        }
        if i_var5 <= i_var7 && i_var6 <= i_var8 {
            i_var10 = param_2.wrapping_add(i_var10);
            i_var11 = param_3.wrapping_add(i_var11);
            if i_var5 <= i_var10 && i_var10 <= i_var7 && i_var6 <= i_var11 && i_var11 <= i_var8 {
                let base = mech_trace::rd!("100562d8", pi_var2, read_unaligned);
                let addr = i_var11
                    .wrapping_mul(i_var3.wrapping_add(1))
                    .wrapping_add(base)
                    .wrapping_add(i_var10);
                let pbvar9 = addr as *mut u8;
                let b_var1 = mech_trace::rd!("100562d8", pbvar9, read);
                mech_trace::wr!("100562d8", pbvar9, write, param_4);
                return mech_trace::rt!("100562d8", u32::from(b_var1));
            }
            return mech_trace::rt!("100562d8", 0xffff_fffd);
        }
        mech_trace::rt!("100562d8", 0xffff_fffe)
    }
}
