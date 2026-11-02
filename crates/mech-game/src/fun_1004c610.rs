// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c610(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: the parameters are addresses of the game's own structures,
    // read at the same offsets as the original.
    unsafe {
        let a = ((param_1 as u32).wrapping_add(0xc0) as *const i32).read_unaligned();
        let b = ((param_2 as u32).wrapping_add(0x4c) as *const i32).read_unaligned();
        if a < b {
            return 3;
        }
        let a = ((param_1 as u32).wrapping_add(0xc0) as *const i32).read_unaligned();
        let c = ((param_2 as u32).wrapping_add(0x50) as *const i32).read_unaligned();
        if a < c {
            return 2;
        }
        let c = ((param_2 as u32).wrapping_add(0x50) as *const i32).read_unaligned();
        let a = ((param_1 as u32).wrapping_add(0xc0) as *const i32).read_unaligned();
        if c < a { 0 } else { 1 }
    }
}
