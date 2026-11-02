// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_1001c1a3(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d6b2(param_1: i32) -> i32 {
    // SAFETY: param_1 points at the game's own structure, and the linked list
    // and arrays it references are read exactly as the original does.
    unsafe {
        let mut local_10: u32 = 0x54;
        if ((param_1.wrapping_add(0x18)) as *const i32).read_unaligned() != 0 {
            local_10 = 0xd0;
        }
        let mut local_8: i32 = ((param_1.wrapping_add(0x1c)) as *const i32).read_unaligned();
        while local_8 != 0 {
            local_10 = local_10.wrapping_add(0x18);
            let mut local_14: i32 =
                ((local_8.wrapping_add(4)) as *const i16).read_unaligned() as i32;
            while local_14 != 0 {
                local_10 = local_10.wrapping_add(0x44);
                local_14 = local_14.wrapping_sub(1);
            }
            let off: i32 = ((local_8.wrapping_add(8)) as *const i32).read_unaligned();
            let mut local_18: i32 = off.wrapping_add(local_8);
            local_14 = ((local_8.wrapping_add(6)) as *const i16).read_unaligned() as i32;
            while local_14 != 0 {
                let w = ((local_18.wrapping_add(2)) as *const u16).read_unaligned() as u32;
                local_10 = w.wrapping_add(local_10).wrapping_add(0x24);
                local_18 = local_18.wrapping_add(0x24);
                local_14 = local_14.wrapping_sub(1);
            }
            local_8 = ((local_8.wrapping_add(0xc)) as *const i32).read_unaligned();
        }
        let i_var1 = FUN_1001c1a3(param_1);
        (local_10 as i32).wrapping_add(i_var1)
    }
}
