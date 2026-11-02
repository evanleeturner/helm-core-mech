// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050490(param_1: i32) -> u32 {
    // SAFETY: the caller passes an address of a readable structure; the
    // field at offset 4 is read where the original does.
    unsafe {
        (param_1 as *const u8)
            .byte_add(4)
            .cast::<u32>()
            .read_unaligned()
    }
}
