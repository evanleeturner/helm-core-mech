// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a1d5(param_1: i32, param_2: i32, param_3: *mut u32) -> *mut u32 {
    mech_trace::enter!("1006a1d5", param_1, param_2, param_3);
    // SAFETY: the caller supplies a valid table at param_1 and a destination
    // buffer at param_3; reads and writes happen in the original's order.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1006a1d5",
            ((param_1
                .wrapping_add(param_2.wrapping_mul(8))
                .wrapping_add(0xc)) as *const i32),
            read_unaligned
        );
        if i_var1 == 0 {
            return mech_trace::rt!("1006a1d5", core::ptr::null_mut());
        }
        let mut pi_var5 = i_var1.wrapping_add(param_1) as usize as *const i32;
        let first = mech_trace::rd!("1006a1d5", pi_var5, read_unaligned) as u32;
        let mut cnt = first;
        let mut dst = param_3;
        let mut cond = param_3 as usize as u32;
        while cond != 0 {
            pi_var5 = pi_var5.wrapping_add(1);
            mech_trace::wr!(
                "1006a1d5",
                dst,
                write_unaligned,
                mech_trace::rd!("1006a1d5", pi_var5, read_unaligned) as u32
            );
            cnt = cnt.wrapping_sub(1);
            dst = dst.wrapping_add(1);
            cond = cnt;
        }
        mech_trace::rt!("1006a1d5", first as usize as *mut u32)
    }
}
