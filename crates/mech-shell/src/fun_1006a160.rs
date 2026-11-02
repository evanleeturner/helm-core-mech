// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a160(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: param_1 is the address of the caller's table; every offset read
    // here is one the original reads, at the same widths.
    unsafe {
        let entry = param_1
            .wrapping_add(8)
            .wrapping_add(param_2.wrapping_mul(8)) as *const i32;
        let i_var1 = entry.read_unaligned().wrapping_add(param_1);
        let lo = (i_var1.wrapping_add(8) as *const u16).read_unaligned() as i32;
        let hi = (i_var1.wrapping_add(0xc) as *const u16).read_unaligned() as i32;
        lo | hi.wrapping_shl(16)
    }
}
