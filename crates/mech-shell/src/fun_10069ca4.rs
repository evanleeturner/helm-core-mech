// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069ca4(param_1: i32) -> u32 {
    mech_trace::enter!("10069ca4", param_1);
    // SAFETY: param_1 is the address of a caller-owned structure holding
    // 16-bit fields at offsets 4, 6, 8 and 10.
    unsafe {
        let p = param_1 as *const u8;
        let a8 = mech_trace::rd!("10069ca4", p.byte_add(8).cast::<u16>(), read_unaligned) as u32;
        let a4 = mech_trace::rd!("10069ca4", p.byte_add(4).cast::<u16>(), read_unaligned) as u32;
        let a10 = mech_trace::rd!("10069ca4", p.byte_add(10).cast::<u16>(), read_unaligned) as u32;
        let a6 = mech_trace::rd!("10069ca4", p.byte_add(6).cast::<u16>(), read_unaligned) as u32;
        let hi = a8.wrapping_sub(a4).wrapping_add(1) as u16;
        let lo = a10.wrapping_sub(a6).wrapping_add(1) as u16;
        mech_trace::rt!("10069ca4", ((hi as u32) << 16) | (lo as u32))
    }
}
