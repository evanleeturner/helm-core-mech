// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100428b0(param_1: *mut i16, param_2: i16) -> i32 {
    mech_trace::enter!("100428b0", param_1, param_2);
    let mut local_8: i16 = 0;
    loop {
        // SAFETY: param_1 points at the caller's record: a count at offset 0
        // and a table pointer at byte offset 4, read as the original does.
        unsafe {
            let count = mech_trace::rd!("100428b0", param_1, read_unaligned);
            if count <= local_8 {
                return mech_trace::rt!(
                    "100428b0",
                    ((count >> 15) as i32).wrapping_shl(16) | 0xffff
                );
            }
            let base = mech_trace::rd!(
                "100428b0",
                param_1.byte_add(4).cast::<i32>(),
                read_unaligned
            );
            let addr = (local_8 as i32).wrapping_mul(0x12).wrapping_add(base);
            let s_var1 = mech_trace::rd!("100428b0", (addr as *const i16), read_unaligned);
            if s_var1 == param_2 {
                return mech_trace::rt!(
                    "100428b0",
                    ((s_var1 >> 15) as i32).wrapping_shl(16) | (local_8 as i32)
                );
            }
        }
        local_8 = local_8.wrapping_add(1);
    }
}
