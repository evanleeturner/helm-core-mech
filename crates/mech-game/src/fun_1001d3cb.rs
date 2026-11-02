// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d3cb(
    param_1: i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
) {
    // SAFETY: param_1 points at the game's structure; the table pointer at
    // offset 0x20 and the output pointers are the caller's, read and written
    // where the original does.
    unsafe {
        let base = ((param_1 as u32).wrapping_add(0x20) as *const i32).read_unaligned();
        if base != 0 {
            let i_var1 = (base as u32)
                .wrapping_add(0x18)
                .wrapping_add((param_2 as u32).wrapping_mul(0x44));
            if !param_3.is_null() {
                param_3
                    .write_unaligned(((i_var1.wrapping_add(0xc)) as *const u32).read_unaligned());
            }
            if !param_4.is_null() {
                param_4
                    .write_unaligned(((i_var1.wrapping_add(0x10)) as *const u32).read_unaligned());
            }
            if !param_5.is_null() {
                param_5
                    .write_unaligned(((i_var1.wrapping_add(0x14)) as *const u32).read_unaligned());
            }
        }
    }
}
