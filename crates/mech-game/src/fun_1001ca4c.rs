// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ca4c(param_1: i32, param_2: u16) {
    // SAFETY: param_1 is the address of a caller-owned structure; the u16 at
    // offset 0x16 is written as the original does.
    unsafe {
        ((param_1 as usize).wrapping_add(0x16) as *mut u16).write_unaligned(param_2);
    }
}
