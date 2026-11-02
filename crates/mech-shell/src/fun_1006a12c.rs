// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_1006a12c(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1006a12c", param_1, param_2);
    // SAFETY: param_1 points at the caller's table; the offset read from it
    // locates a record whose 16-bit fields are read where the original does.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1006a12c",
            (param_1 as *const u8)
                .offset(8)
                .offset(param_2.wrapping_mul(8) as isize)
                .cast::<i32>(),
            read_unaligned
        )
        .wrapping_add(param_1);
        let p = i_var1 as *const u8;
        let s_10 = mech_trace::rd!("1006a12c", p.byte_add(0x10).cast::<i16>(), read_unaligned);
        let s_08 = mech_trace::rd!("1006a12c", p.byte_add(0x8).cast::<i16>(), read_unaligned);
        let s_14 = mech_trace::rd!("1006a12c", p.byte_add(0x14).cast::<i16>(), read_unaligned);
        let s_0c = mech_trace::rd!("1006a12c", p.byte_add(0xc).cast::<i16>(), read_unaligned);
        let dim1 = (s_10 as i32).wrapping_sub(s_08 as i32).wrapping_add(1);
        let dim2 = (s_14 as i32).wrapping_sub(s_0c as i32).wrapping_add(1);
        mech_trace::rt!("1006a12c", (dim1.wrapping_shl(16)) | dim2)
    }
}
