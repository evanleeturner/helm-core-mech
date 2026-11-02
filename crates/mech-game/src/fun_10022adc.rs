// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022adc(param_1: i32) -> u32 {
    // SAFETY: param_1 is a pointer-sized address of a game object whose field
    // at +0x20 points at another object; the u32 at +0x3ec of that is read.
    unsafe {
        let inner = ((param_1 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        ((inner as usize).wrapping_add(0x3ec) as *const u32).read_unaligned()
    }
}
