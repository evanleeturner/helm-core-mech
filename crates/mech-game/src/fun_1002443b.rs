// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
    fn FUN_100562d8(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002443b(param_1: i32) -> u32 {
    if param_1 == 0 {
        return 0;
    }
    let mut local_3c: i32 = 0;
    let mut local_38: i32;
    let mut local_34: i32 = 0;
    let local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32 = 0;
    let mut local_24: i32;
    let mut local_20: i32;
    let local_1c: i32;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let local_c: i32;
    let mut local_8: i32 = 0;

    // SAFETY: param_1 points at the caller's game object with the offsets
    // the original reads; callees and the global are the game's own.
    unsafe {
        FUN_1002f322(
            (param_1.wrapping_add(0x1d2) as *const i32).read_unaligned(),
            (&raw mut local_3c).cast::<u32>(),
            (&raw mut local_10).cast::<u32>(),
            (&raw mut local_18).cast::<u32>(),
        );
        local_30 = (param_1.wrapping_add(0x1ea) as *const i32).read_unaligned() / 10;
        local_38 = (param_1.wrapping_add(0x1ee) as *const i32).read_unaligned() / 10;
        local_c = (param_1.wrapping_add(0x1f2) as *const i32).read_unaligned() / 10;
        local_1c = (param_1.wrapping_add(0x1e6) as *const i32).read_unaligned();
        local_20 = 0;
        local_2c = local_18;
        FUN_1003ba89(&raw mut local_20, &raw mut local_20, &raw mut local_2c);
        if local_2c < 0x4e21 {
            local_24 = 0;
            while local_24 < 0x10 {
                let off = local_24.wrapping_mul(0xe);
                let x =
                    (param_1.wrapping_add(0x12).wrapping_add(off) as *const i32).read_unaligned();
                let y =
                    (param_1.wrapping_add(0x16).wrapping_add(off) as *const i32).read_unaligned();
                let z =
                    (param_1.wrapping_add(0x1a).wrapping_add(off) as *const i32).read_unaligned();
                local_20 = x.wrapping_add(local_3c);
                local_28 = y.wrapping_add(local_10);
                local_2c = z.wrapping_add(local_18);
                let i_var2 = FUN_1003ba89(&raw mut local_20, &raw mut local_28, &raw mut local_2c);
                if i_var2 != 0 {
                    local_34 = x.wrapping_add(local_30).wrapping_add(local_3c);
                    local_8 = y.wrapping_add(local_38).wrapping_add(local_10);
                    local_14 = z.wrapping_add(local_c).wrapping_add(local_18);
                    FUN_1003ba89(&raw mut local_34, &raw mut local_8, &raw mut local_14);
                    FUN_10056489(
                        (&raw mut DAT_101f52e0).cast::<i32>(),
                        local_20,
                        local_28,
                        local_34,
                        local_8,
                        0,
                        local_1c,
                    );
                }
                local_24 = local_24.wrapping_add(1);
            }
        } else {
            local_24 = 0;
            while local_24 < 0x10 {
                let off = local_24.wrapping_mul(0xe);
                let x =
                    (param_1.wrapping_add(0x12).wrapping_add(off) as *const i32).read_unaligned();
                let y =
                    (param_1.wrapping_add(0x16).wrapping_add(off) as *const i32).read_unaligned();
                let z =
                    (param_1.wrapping_add(0x1a).wrapping_add(off) as *const i32).read_unaligned();
                local_20 = x.wrapping_add(local_3c);
                local_28 = y.wrapping_add(local_10);
                local_2c = z.wrapping_add(local_18);
                let i_var2 = FUN_1003ba89(&raw mut local_20, &raw mut local_28, &raw mut local_2c);
                if i_var2 != 0 {
                    FUN_100562d8(
                        (&raw mut DAT_101f52e0).cast::<i32>(),
                        local_20,
                        local_28,
                        local_1c as u8,
                    );
                }
                local_24 = local_24.wrapping_add(1);
            }
        }
    }
    let _ = (local_34, local_8, local_14, local_28, local_38);
    local_38 = 0;
    let _ = local_38;
    1
}
