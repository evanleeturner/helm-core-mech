// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52e0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> i32;
    fn FUN_100563b3(param_1: *mut i32, param_2: i32, param_3: i32) -> u64;
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
pub unsafe extern "C" fn FUN_1003a8ca(param_1: i32) -> u32 {
    mech_trace::enter!("1003a8ca", param_1);
    if param_1 == 0 {
        return mech_trace::rt!("1003a8ca", 0);
    }
    let p = param_1 as *const u8;
    let mut local_40: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_2c: i32 = 0;
    let mut local_30: i32 = 0;
    let mut local_38: i32 = 0;
    let mut local_8: i32 = 0;
    let mut local_14: i32 = 0;
    // SAFETY: param_1 points at the caller's object; the offsets read and the
    // callees and global are the game's own.
    unsafe {
        let rd = |o: usize| -> i32 {
            mech_trace::rd!("1003a8ca", p.byte_add(o).cast::<i32>(), read_unaligned)
        };
        FUN_1002f322(rd(0x1d2), &mut local_40, &mut local_10, &mut local_18);
        let _local_1c = rd(0x1f6);
        let local_34 = rd(0x1ea) >> 4;
        let local_3c = rd(0x1ee) >> 4;
        let local_c = rd(0x1f2) >> 4;
        let mut local_20 = rd(0x1e6);

        let mut local_28: i32 = 0;
        while local_28 < 2 {
            let off = (local_28 as u32).wrapping_mul(0xe) as usize;

            local_24 = rd(0x12 + off).wrapping_add(local_40);
            local_2c = rd(0x16 + off).wrapping_add(local_10);
            local_30 = rd(0x1a + off).wrapping_add(local_18);

            let iv = mech_trace::cr!(
                "1003a8ca",
                "1003ba89",
                FUN_1003ba89(&mut local_24, &mut local_2c, &mut local_30)
            );
            if iv != 0 {
                local_38 = rd(0x12 + off).wrapping_add(local_34).wrapping_add(local_40);
                local_8 = rd(0x16 + off).wrapping_add(local_3c).wrapping_add(local_10);
                local_14 = rd(0x1a + off).wrapping_add(local_c).wrapping_add(local_18);

                mech_trace::cr!(
                    "1003a8ca",
                    "1003ba89",
                    FUN_1003ba89(&mut local_38, &mut local_8, &mut local_14)
                );

                let u5 = mech_trace::cr!(
                    "1003a8ca",
                    "100563b3",
                    FUN_100563b3(&raw mut DAT_101f52e0, local_24, local_2c)
                );
                let u3 = u5 as u32;
                let u4 = ((u3 as i32) >> 0x1f) as u32;
                let t = ((u3 ^ u4).wrapping_sub(u4) & 0xf) ^ u4;
                if t.wrapping_sub(u4) == 0xf {
                    local_20 = u3.wrapping_sub(1) as i32;
                } else {
                    local_20 = u3.wrapping_add(1) as i32;
                }
                mech_trace::cr!(
                    "1003a8ca",
                    "10056489",
                    FUN_10056489(
                        &raw mut DAT_101f52e0,
                        local_24,
                        local_2c,
                        local_38,
                        local_8,
                        0,
                        local_20,
                    )
                );
            }
            local_28 += 1;
        }
        let _ = local_20;
    }
    mech_trace::rt!("1003a8ca", 1)
}
