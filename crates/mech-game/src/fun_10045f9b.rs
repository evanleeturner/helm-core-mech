// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045f9b(param_1: i32, param_2: u32) {
    // SAFETY: param_1 is an address of a game structure; the field at
    // offset 0x30 is written with param_2, as the original does.
    unsafe {
        (param_1 as *mut u8)
            .byte_add(0x30)
            .cast::<u32>()
            .write_unaligned(param_2);
    }
}
