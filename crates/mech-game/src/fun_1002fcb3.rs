// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fcb3(param_1: i32, param_2: *const u32) {
    let mut src = param_2;
    let mut dst = param_1.wrapping_add(0x3c) as *mut u32;
    let mut count: i32 = 0xc;
    while count != 0 {
        // SAFETY: the caller supplies a source of twelve dwords and a
        // destination object with room at offset 0x3c for twelve dwords.
        unsafe {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
        }
        count -= 1;
    }
}
