// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045fdd(
    param_1: i32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) {
    // SAFETY: param_1 is the address of the caller's structure; the four
    // u16 fields at offsets 0xb4..0xbc are written as the original does.
    unsafe {
        ((param_1 as usize).wrapping_add(0xb4) as *mut u16).write_unaligned(param_2);
        ((param_1 as usize).wrapping_add(0xb6) as *mut u16).write_unaligned(param_3);
        ((param_1 as usize).wrapping_add(0xb8) as *mut u16).write_unaligned(param_4);
        ((param_1 as usize).wrapping_add(0xba) as *mut u16).write_unaligned(param_5);
    }
}
