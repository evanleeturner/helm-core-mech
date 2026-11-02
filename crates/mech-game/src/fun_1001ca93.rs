// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ca93(param_1: i32) -> u16 {
    // SAFETY: param_1 is an address supplied by the caller; the u16 at
    // offset 0x16 is read where the original reads it.
    unsafe { ((param_1.wrapping_add(0x16)) as *const u16).read_unaligned() }
}
