// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100807b5(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008072a(param_1: i32, param_2: i32) {
    // SAFETY: param_1 and param_2 are addresses of the game's own structures;
    // reads happen at the same offsets and in the same order as the original.
    unsafe {
        let idx_off = ((param_1 as u32).wrapping_add(4) as *const i32).read_unaligned();
        let b = ((idx_off.wrapping_add(param_1)) as u32 as *const u8).read();
        let i_var1 = (b as i32).wrapping_mul(0x44).wrapping_add(param_2);
        let a1 = ((i_var1.wrapping_add(0xc)) as u32 as *const i32).read_unaligned();
        let a2 = ((i_var1.wrapping_add(0x10)) as u32 as *const i32).read_unaligned();
        let a3 = ((i_var1.wrapping_add(0x14)) as u32 as *const i32).read_unaligned();
        let a4 = ((param_1.wrapping_add(0x14)) as u32 as *const i32).read_unaligned();
        let a5 = ((param_1.wrapping_add(0x18)) as u32 as *const i32).read_unaligned();
        let a6 = ((param_1.wrapping_add(0x1c)) as u32 as *const i32).read_unaligned();
        FUN_100807b5(a1, a2, a3, a4, a5, a6);
    }
}
