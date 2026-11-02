// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004601c(param_1: i32, param_2: u16) {
    // SAFETY: param_1 is an address of a caller-owned structure; the u16 at
    // offset 6 is written where the original does.
    unsafe {
        ((param_1 as usize).wrapping_add(6) as *mut u16).write_unaligned(param_2);
    }
}
