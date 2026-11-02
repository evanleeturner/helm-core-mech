// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b520(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
) {
    mech_trace::enter!("1001b520", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 and param_2 point at the caller's valid i32 values,
    // read and written where the original does.
    unsafe {
        let i_var1 = mech_trace::rd!("1001b520", param_2, read_unaligned);
        let prod = (param_3 as u32).wrapping_mul(param_4 as u32);
        mech_trace::wr!(
            "1001b520",
            param_2,
            write_unaligned,
            i_var1.wrapping_add(prod as i32)
        );
        let mid = ((prod as i32) >> 1).wrapping_add(i_var1);
        let l_var2 = (mid as i64).wrapping_mul(param_4 as i64);
        let lo = l_var2 as u32;
        let hi = ((l_var2 as u64) >> 32) as u32;
        let v = mech_trace::rd!("1001b520", param_1, read_unaligned);
        mech_trace::wr!(
            "1001b520",
            param_1,
            write_unaligned,
            v.wrapping_add(((lo >> 0x10) | (hi << 0x10)) as i32)
        );
    }
}
