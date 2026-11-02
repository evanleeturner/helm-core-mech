// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d444(
    param_1: i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
) {
    // SAFETY: param_1 points at the game's struct; its field at 0x20 is a
    // table pointer whose entry is read and written to the caller's outputs
    // exactly where the original does.
    unsafe {
        let base = (param_1.wrapping_add(0x20) as *const i32).read_unaligned();
        if base != 0 {
            let p = (base as u32)
                .wrapping_add(0x18)
                .wrapping_add((param_2 as u32).wrapping_mul(0x44))
                as *const u32;
            if !param_3.is_null() {
                param_3.write_unaligned(p.read_unaligned());
            }
            if !param_4.is_null() {
                param_4.write_unaligned(p.add(1).read_unaligned());
            }
            if !param_5.is_null() {
                param_5.write_unaligned(p.add(2).read_unaligned());
            }
        }
    }
}
