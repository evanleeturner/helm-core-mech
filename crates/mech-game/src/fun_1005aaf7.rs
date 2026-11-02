// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_1005aaf7(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: param_1 is an address of the caller's structure; the table
    // offset and the pointed-to offset are read as the original does.
    unsafe {
        let inner = param_2
            .wrapping_mul(4)
            .wrapping_add(param_1)
            .wrapping_add(0x10);
        let off = (inner as usize as *const i32).read_unaligned();
        (off.wrapping_add(param_1) as usize as *const i32).read_unaligned()
    }
}
