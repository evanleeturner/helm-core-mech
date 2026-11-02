// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b569(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: param_1 is the address of the caller's table; the entry at
    // offset 8 + param_2 * 8 holds an offset, and the word at
    // param_1 + offset + 4 is read, as the original does.
    unsafe {
        let inner = (param_1
            .wrapping_add(param_2.wrapping_mul(8))
            .wrapping_add(8) as *const u32)
            .read_unaligned();
        (inner.wrapping_add(param_1 as u32).wrapping_add(4) as *const u32).read_unaligned()
    }
}
