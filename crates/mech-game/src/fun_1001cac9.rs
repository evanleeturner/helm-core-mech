// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001cac9(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
) -> u32 {
    // SAFETY: param_1 is an address of the caller's structure; the output
    // pointers are written only when non-null, as in the original.
    unsafe {
        if !param_2.is_null() {
            param_2.write_unaligned(((param_1.wrapping_add(0x34)) as *const u32).read_unaligned());
        }
        if !param_3.is_null() {
            param_3.write_unaligned(((param_1.wrapping_add(0x38)) as *const u32).read_unaligned());
        }
        if !param_4.is_null() {
            param_4.write_unaligned(((param_1.wrapping_add(0x3c)) as *const u32).read_unaligned());
        }
        ((param_1.wrapping_add(0x40)) as *const u32).read_unaligned()
    }
}
