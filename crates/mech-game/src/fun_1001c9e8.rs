// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c9e8(param_1: *mut u16, param_2: u16) {
    // SAFETY: param_1 points at the caller's u16, read and written as the original does.
    unsafe {
        let v = param_1.read_unaligned();
        param_1.write_unaligned((v & 0x810f) | (param_2 & 0x7ef0) | 0x8000);
    }
}
