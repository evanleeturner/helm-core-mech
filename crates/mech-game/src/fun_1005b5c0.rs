// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b5c0(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1005b5c0", param_1, param_2);
    // SAFETY: param_1 is the address of a table in the game's memory; the
    // offsets read are those the original reads.
    unsafe {
        let off = (param_1
            .wrapping_add(8)
            .wrapping_add(param_2.wrapping_mul(8))) as usize as *const i32;
        let i_var1 = mech_trace::rd!("1005b5c0", off, read_unaligned).wrapping_add(param_1);
        let hi = mech_trace::rd!(
            "1005b5c0",
            ((i_var1.wrapping_add(0xc)) as usize as *const u16),
            read_unaligned
        );
        let lo = mech_trace::rd!(
            "1005b5c0",
            ((i_var1.wrapping_add(8)) as usize as *const u16),
            read_unaligned
        );
        mech_trace::rt!("1005b5c0", ((u32::from(hi) << 16) | u32::from(lo)) as i32)
    }
}
