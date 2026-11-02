// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056c60(param_1: i16, param_2: i16, param_3: i32) {
    // SAFETY: param_3 is the base of the caller's array of 4-byte entries;
    // both indices address entries within it, read and written as the original does.
    unsafe {
        let second = (param_3.wrapping_add((param_2 as i32).wrapping_mul(4))) as *mut u32;
        let first = (param_3.wrapping_add((param_1 as i32).wrapping_mul(4))) as *mut u32;
        let saved = second.read_unaligned();
        second.write_unaligned(first.read_unaligned());
        first.write_unaligned(saved);
    }
}
