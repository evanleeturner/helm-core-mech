// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045fc6(param_1: i32, param_2: u32) {
    // SAFETY: param_1 is an address of the game's object; the field at
    // offset 0xac is written where the original does.
    unsafe {
        (param_1.wrapping_add(0xac) as *mut u32).write_unaligned(param_2);
    }
}
