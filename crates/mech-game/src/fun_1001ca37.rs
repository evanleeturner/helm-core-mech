// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ca37(param_1: i32, param_2: u16) {
    // SAFETY: param_1 is an address supplied by the caller; the u16 at
    // offset 2 is written where the original does.
    unsafe {
        ((param_1.wrapping_add(2)) as *mut u16).write_unaligned(param_2);
    }
}
