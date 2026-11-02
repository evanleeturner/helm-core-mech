// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: u32;
    static mut DAT_101f64a8: u8;
    static mut DAT_101f64b8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100645c5(param_1: i32) -> u8;
    fn FUN_1000d05f(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e3ec(param_1: u32) -> u32 {
    let mut local_8: u32 = 0;
    let mut local_10: u32 = 0;
    // SAFETY: the globals and callees are the game's own; the table entries
    // are read at the same strides and offsets as the original.
    unsafe {
        while (local_10 as i32) < (DAT_100a7584 as i32) {
            let p = (&raw mut DAT_101f64a8)
                .byte_add(local_10.wrapping_mul(0x7c) as usize)
                .cast::<u32>()
                .read_unaligned();
            if p == param_1 {
                FUN_1000e3ec(local_10);
            }
            local_10 = local_10.wrapping_add(1);
        }
        if param_1 < 0x402 {
            let v = (&raw mut DAT_101f64b8)
                .byte_add(param_1.wrapping_mul(0x7c) as usize)
                .cast::<u32>()
                .read_unaligned();
            if v < 0xfe {
                let w = (&raw mut DAT_101f64b8)
                    .byte_add(param_1.wrapping_mul(0x7c) as usize)
                    .cast::<u32>()
                    .read_unaligned();
                FUN_100645c5(w as i32);
            }
            FUN_1000d05f(param_1 as i32);
            local_8 = 1;
        }
    }
    local_8
}
