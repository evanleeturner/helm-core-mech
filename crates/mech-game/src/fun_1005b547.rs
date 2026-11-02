// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b547(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: param_1 is an address of the caller's table; the offset entry
    // is read, then the value at that offset from param_1 is read.
    unsafe {
        let off = ((param_1
            .wrapping_add(8)
            .wrapping_add(param_2.wrapping_mul(8))) as usize as *const i32)
            .read_unaligned();
        (param_1.wrapping_add(off) as usize as *const u32).read_unaligned()
    }
}
