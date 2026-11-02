// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100104bb(param_1: *mut u32) {
    // SAFETY: param_1 points at the caller's buffer of at least 8 bytes,
    // written at the same offsets and widths as the original.
    unsafe {
        param_1.write_unaligned(0xffff_ffff);
        param_1.add(1).cast::<u16>().write_unaligned(0);
        param_1.byte_add(6).cast::<u16>().write_unaligned(0);
    }
}
