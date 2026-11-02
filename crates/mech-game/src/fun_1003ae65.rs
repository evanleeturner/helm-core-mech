// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52e0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> i32;
    fn __ftol() -> i64;
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ae65(param_1: i32) -> u32 {
    mech_trace::enter!("1003ae65", param_1);
    if param_1 == 0 {
        return mech_trace::rt!("1003ae65", 0);
    }
    let mut local_44: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_1c: i32 = 0;
    let mut local_28: i32 = 0;
    let mut local_30: i32 = 0;
    let mut local_34: i32 = 0;
    let mut local_3c: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_18: i32 = 0;
    let base = param_1 as *const u8;
    // SAFETY: param_1 points at the game's object; offsets, globals and callees
    // are the game's own, accessed exactly as the original does.
    unsafe {
        let tmp = mech_trace::rd!(
            "1003ae65",
            base.byte_add(0x1d2).cast::<i32>(),
            read_unaligned
        );
        FUN_1002f322(tmp, &raw mut local_44, &raw mut local_14, &raw mut local_1c);
        let local_20 = mech_trace::rd!(
            "1003ae65",
            base.byte_add(0x1f6).cast::<i32>(),
            read_unaligned
        );
        let p20 = local_20 as *const u8;
        let local_8 = mech_trace::rd!("1003ae65", p20.byte_add(0x20).cast::<i32>(), read_unaligned);
        let local_38 =
            mech_trace::rd!("1003ae65", p20.byte_add(0x14).cast::<i32>(), read_unaligned) >> 2;
        let local_40 =
            mech_trace::rd!("1003ae65", p20.byte_add(0x18).cast::<i32>(), read_unaligned) >> 2;
        let local_10 =
            mech_trace::rd!("1003ae65", p20.byte_add(0x1c).cast::<i32>(), read_unaligned) >> 2;
        let mut local_2c: i32 = 0;
        while local_2c < 1 {
            let off = (local_2c as usize).wrapping_mul(0xe);
            let vx = mech_trace::rd!(
                "1003ae65",
                base.byte_add(0x12).byte_add(off).cast::<i32>(),
                read_unaligned
            );
            let vy = mech_trace::rd!(
                "1003ae65",
                base.byte_add(0x16).byte_add(off).cast::<i32>(),
                read_unaligned
            );
            let vz = mech_trace::rd!(
                "1003ae65",
                base.byte_add(0x1a).byte_add(off).cast::<i32>(),
                read_unaligned
            );
            local_28 = vx.wrapping_add(local_44);
            local_30 = vy.wrapping_add(local_14);
            local_34 = vz.wrapping_add(local_1c);
            let i_var2 = mech_trace::cr!(
                "1003ae65",
                "1003ba89",
                FUN_1003ba89(&raw mut local_28, &raw mut local_30, &raw mut local_34)
            );
            if i_var2 != 0 {
                let l_var3 = mech_trace::cr!("1003ae65", "__ftol", __ftol());
                let mut local_24 = l_var3 as i32;
                if local_24 < 0 {
                    local_24 = 0;
                } else if 0x1f < local_24 {
                    local_24 = 0x1f;
                }
                let addr = local_8
                    .wrapping_add(0x18)
                    .wrapping_add(local_24.wrapping_mul(4));
                local_24 = mech_trace::rd!("1003ae65", (addr as *const i32), read_unaligned);
                local_3c = vx.wrapping_add(local_38).wrapping_add(local_44);
                local_c = vy.wrapping_add(local_40).wrapping_add(local_14);
                local_18 = vz.wrapping_add(local_10).wrapping_add(local_1c);
                mech_trace::cr!(
                    "1003ae65",
                    "1003ba89",
                    FUN_1003ba89(&raw mut local_3c, &raw mut local_c, &raw mut local_18)
                );
                mech_trace::cr!(
                    "1003ae65",
                    "10056489",
                    FUN_10056489(
                        &raw mut DAT_101f52e0,
                        local_28,
                        local_30,
                        local_3c,
                        local_c,
                        0,
                        local_24,
                    )
                );
            }
            local_2c = local_2c.wrapping_add(1);
        }
    }
    mech_trace::rt!("1003ae65", 1)
}
