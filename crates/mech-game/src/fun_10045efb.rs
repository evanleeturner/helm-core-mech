// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045efb(param_1: i32, param_2: u32) {
    // SAFETY: param_1 is an address of a caller-owned structure; the u32 at
    // offset 0xc is written where the original does.
    unsafe {
        ((param_1.wrapping_add(0xc)) as *mut u32).write_unaligned(param_2);
    }
}
