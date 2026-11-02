// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b547(param_1: i32, param_2: i32) -> u32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_100663e5(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100664ef(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    // SAFETY: the callees are the game's own; param_1 is the address of a
    // rect of i32 fields, read at the offsets the original reads.
    unsafe {
        let u_var1 = FUN_1005b547(param_3, param_4);
        let w = (param_1.wrapping_add(0xc) as *const i32)
            .read_unaligned()
            .wrapping_sub((param_1.wrapping_add(4) as *const i32).read_unaligned())
            .wrapping_add(1);
        let i_var2 = FUN_10002570(((u_var1 as i32) >> 0x10) as u32, w) as i32;
        let h = (param_1.wrapping_add(0x10) as *const i32)
            .read_unaligned()
            .wrapping_sub((param_1.wrapping_add(8) as *const i32).read_unaligned())
            .wrapping_add(1);
        let i_var3 = FUN_10002570(u_var1 & 0xffff, h) as i32;
        FUN_100663e5(param_1, param_2, i_var2, i_var3);
        param_2
    }
}
