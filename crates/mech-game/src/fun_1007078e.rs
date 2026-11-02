// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007078e(param_1: i32) {
    // SAFETY: param_1 is the address of the caller's object; the two fields
    // at offsets 0x88 and 0x8c are written as the original does.
    unsafe {
        (param_1.wrapping_add(0x8c) as *mut u32).write_unaligned(0);
        (param_1.wrapping_add(0x88) as *mut u32).write_unaligned(0xffff_ffff);
    }
}
