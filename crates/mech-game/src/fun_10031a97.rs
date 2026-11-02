// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f6e8(param_1: i32) -> u8;
    fn FUN_100317f8(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031a97(param_1: i32, param_2: i32) {
    if param_2 != 0 && param_1 != 0 {
        // SAFETY: all addresses are the game's own structures, read and
        // passed to the game's functions in the original order.
        unsafe {
            let i_var2 = ((param_2 as u32).wrapping_add(0x6c) as *const i32).read_unaligned();
            let i_var3 = ((i_var2 as u32).wrapping_add(0x20) as *const i32).read_unaligned();
            let a = ((i_var3 as u32).wrapping_add(0x10) as *const i32).read_unaligned();
            let b = ((i_var2 as u32).wrapping_add(0x50) as *const i32).read_unaligned();
            if a != b {
                FUN_1002f6e8(i_var2);
            }
            let s_var1 = ((i_var3 as u32).wrapping_add(6) as *const i16).read_unaligned();
            let mut local_14: i32 = 0;
            while local_14 < s_var1 as i32 {
                let off = ((i_var3 as u32).wrapping_add(8) as *const i32).read_unaligned();
                FUN_100317f8(
                    param_1,
                    off.wrapping_add(i_var3)
                        .wrapping_add(local_14.wrapping_mul(0x24)),
                );
                local_14 = local_14.wrapping_add(1);
            }
            let child = ((param_2 as u32).wrapping_add(4) as *const i32).read_unaligned();
            if child != 0 {
                let mut local_8 = ((param_2 as u32).wrapping_add(4) as *const i32).read_unaligned();
                while local_8 != 0 {
                    FUN_10031a97(param_1, local_8);
                    local_8 = ((local_8 as u32).wrapping_add(8) as *const i32).read_unaligned();
                }
            }
        }
    }
}
