// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a21d(param_1: i32, param_2: i32, param_3: *mut u32) -> *mut u32 {
    mech_trace::enter!("1006a21d", param_1, param_2, param_3);
    // SAFETY: param_1 is the caller's table base and every address derived
    // from it and from param_3 is read and written exactly as the original does.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1006a21d",
            ((param_1
                .wrapping_add(param_2.wrapping_mul(8))
                .wrapping_add(0xc)) as usize as *const i32),
            read_unaligned
        );
        if i_var1 == 0 {
            return mech_trace::rt!("1006a21d", core::ptr::null_mut());
        }
        let mut pi_var5 = i_var1.wrapping_add(param_1) as usize as *mut i32;
        let pu_var3 = mech_trace::rd!("1006a21d", pi_var5, read_unaligned) as u32;
        let mut pu_var4: u32 = pu_var3;
        let mut p3 = param_3;
        let mut pu_var2: u32 = p3 as usize as u32;
        while pu_var2 != 0 {
            pi_var5 = pi_var5.wrapping_add(1);
            mech_trace::wr!(
                "1006a21d",
                pi_var5,
                write_unaligned,
                mech_trace::rd!("1006a21d", (p3 as *const i32), read_unaligned)
            );
            pu_var4 = pu_var4.wrapping_sub(1);
            p3 = p3.wrapping_add(1);
            pu_var2 = pu_var4;
        }
        mech_trace::rt!("1006a21d", pu_var3 as usize as *mut u32)
    }
}
