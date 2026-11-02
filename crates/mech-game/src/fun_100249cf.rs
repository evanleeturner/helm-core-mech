// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_100562d8(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100249cf(param_1: i32) -> u32 {
    if param_1 == 0 {
        return 0;
    }
    let mut local_24: i32 = 0;
    let mut local_20: i32 = 0;
    let mut local_1c: i32 = 0;
    let mut local_18: i32;
    let mut local_14: i32 = 0;
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    // SAFETY: the callees and the global are the game's own; param_1 points at
    // the caller's object, read at the same offsets as the original.
    unsafe {
        FUN_1002f322(
            ((param_1.wrapping_add(0x1d2)) as *const i32).read_unaligned(),
            (&raw mut local_24).cast::<u32>(),
            (&raw mut local_8).cast::<u32>(),
            (&raw mut local_c).cast::<u32>(),
        );
        local_14 = 0;
        local_20 = local_c;
        FUN_1003ba89(&raw mut local_14, &raw mut local_14, &raw mut local_20);
        let local_10: u32 = ((param_1.wrapping_add(0x1e6)) as *const u32).read_unaligned();
        let step: i32 = if local_20 < 0x2711 { 1 } else { 2 };
        local_18 = 0;
        while local_18 < 0x10 {
            let off = local_18.wrapping_mul(0xe);
            local_14 = (((param_1.wrapping_add(0x12).wrapping_add(off)) as *const i32)
                .read_unaligned())
            .wrapping_add(local_24);
            local_1c = (((param_1.wrapping_add(0x16).wrapping_add(off)) as *const i32)
                .read_unaligned())
            .wrapping_add(local_8);
            local_20 = (((param_1.wrapping_add(0x1a).wrapping_add(off)) as *const i32)
                .read_unaligned())
            .wrapping_add(local_c);
            let i_var2 = FUN_1003ba89(&raw mut local_14, &raw mut local_1c, &raw mut local_20);
            if i_var2 != 0 {
                FUN_100562d8(
                    (&raw mut DAT_101f52e0).cast::<i32>(),
                    local_14,
                    local_1c,
                    local_10 as u8,
                );
            }
            local_18 = local_18.wrapping_add(step);
        }
    }
    1
}
