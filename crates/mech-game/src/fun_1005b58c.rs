// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b58c(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1005b58c", param_1, param_2);
    // SAFETY: param_1 points at the caller's table; the offsets read are the
    // ones the original reads, and the computed address is the caller's record.
    unsafe {
        let off = param_1
            .wrapping_add(8)
            .wrapping_add(param_2.wrapping_mul(8));
        let iv = mech_trace::rd!("1005b58c", (off as usize as *const i32), read_unaligned)
            .wrapping_add(param_1);
        let base = iv as usize as *const u8;
        let a = mech_trace::rd!(
            "1005b58c",
            base.byte_add(0x10).cast::<i16>(),
            read_unaligned
        ) as i32;
        let b = mech_trace::rd!("1005b58c", base.byte_add(8).cast::<i16>(), read_unaligned) as i32;
        let c = mech_trace::rd!(
            "1005b58c",
            base.byte_add(0x14).cast::<i16>(),
            read_unaligned
        ) as i32;
        let d =
            mech_trace::rd!("1005b58c", base.byte_add(0xc).cast::<i16>(), read_unaligned) as i32;
        let hi = a.wrapping_sub(b).wrapping_add(1).wrapping_shl(16);
        let lo = (c.wrapping_sub(d).wrapping_add(1) as u16) as i32;
        mech_trace::rt!("1005b58c", hi | lo)
    }
}
