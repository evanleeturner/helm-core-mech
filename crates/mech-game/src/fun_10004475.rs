// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10004475(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's structure; the pointer stored at
    // its start addresses another structure, read and written at the same
    // offsets as the original.
    unsafe {
        if param_1.add(0xfc).read_unaligned() != 1 {
            let i_var1 = param_1.read_unaligned();
            let base = i_var1 as u32 as usize;
            let a = (base.wrapping_add(0x60) as *const i32).read_unaligned() as u32;
            let u_var2 = a
                .wrapping_add(0x1680000)
                .wrapping_add(param_1.add(3).read_unaligned() as u32);
            (base.wrapping_add(0xfc) as *mut i32).write_unaligned((u_var2 as i32) % 0x1680000);
        }
    }
}
