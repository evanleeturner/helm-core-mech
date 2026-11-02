// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b50f(param_1: i32) -> i32 {
    mech_trace::enter!("1005b50f", param_1);
    let mut i_var1: i32 = param_1.wrapping_add(0xd);
    // SAFETY: param_1 is an address of the caller's buffer; the bytes are
    // read where the original reads them.
    unsafe {
        let flags = mech_trace::rd!("1005b50f", ((param_1.wrapping_add(10)) as *const u8), read);
        if (flags & 0x80) != 0 {
            let shift = ((flags & 7) as i32).wrapping_add(1);
            i_var1 = i_var1.wrapping_add((1i32.wrapping_shl(shift as u32)).wrapping_mul(3));
        }
        mech_trace::rt!(
            "1005b50f",
            mech_trace::rd!(
                "1005b50f",
                (i_var1.wrapping_add(5) as *const i32),
                read_unaligned
            )
        )
    }
}
