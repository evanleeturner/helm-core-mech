// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
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
pub unsafe extern "C" fn FUN_100594ec(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> u32 {
    mech_trace::enter!(
        "100594ec", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut param_5 = param_5;
    if param_4 == 0 || param_5 == 0 {
        // SAFETY: the callee is the game's own function; arguments are passed through.
        return mech_trace::rt!("100594ec", unsafe {
            mech_trace::cr!(
                "100594ec",
                "10056489",
                FUN_10056489(
                    param_1,
                    param_2.wrapping_sub(param_4),
                    param_3.wrapping_sub(param_5),
                    param_2.wrapping_add(param_4),
                    param_3.wrapping_add(param_5),
                    0,
                    param_6,
                )
            )
        });
    }
    // SAFETY: param_1 points at the game's clip/surface descriptor; the surface
    // pointer it holds is written only inside the clip rectangle.
    unsafe {
        let pi_var1 = mech_trace::rd!("100594ec", param_1, read_unaligned) as *mut i32;
        let i_var13 = mech_trace::rd!("100594ec", pi_var1.add(1), read_unaligned);
        let i_var9 = i_var13.wrapping_add(1);
        if i_var9 == 0 || i_var13 < -1 {
            return mech_trace::rt!("100594ec", 0xffff_ffff);
        }
        let i_var2 = mech_trace::rd!("100594ec", pi_var1.add(2), read_unaligned);
        if i_var2 < 0 {
            return mech_trace::rt!("100594ec", 0xffff_ffff);
        }
        let i_var3 = mech_trace::rd!("100594ec", param_1.add(1), read_unaligned);
        let mut i_var10 = i_var3;
        if i_var3 < 1 {
            i_var10 = 0;
        }
        let i_var4 = mech_trace::rd!("100594ec", param_1.add(2), read_unaligned);
        let mut i_var11 = i_var4;
        if i_var4 < 1 {
            i_var11 = 0;
        }
        let mut i_var12 = mech_trace::rd!("100594ec", param_1.add(3), read_unaligned);
        if i_var13 <= mech_trace::rd!("100594ec", param_1.add(3), read_unaligned) {
            i_var12 = i_var13;
        }
        let mut i_var13 = mech_trace::rd!("100594ec", param_1.add(4), read_unaligned);
        if i_var2 <= mech_trace::rd!("100594ec", param_1.add(4), read_unaligned) {
            i_var13 = i_var2;
        }
        if i_var12 < i_var10 || i_var13 < i_var11 {
            return mech_trace::rt!("100594ec", 0xffff_fffe);
        }
        let base = mech_trace::rd!("100594ec", pi_var1, read_unaligned);
        let u_var7 = param_6 as u8;
        let i_var3 = param_2.wrapping_add(i_var3);
        let i_var4 = param_3.wrapping_add(i_var4);
        let mut local_10: i32 = 0;
        let mut local_14: i32 = param_5;
        let i_var5 = param_5.wrapping_mul(param_5);
        let u_var6 = param_4.wrapping_mul(param_4) as u32;
        let mut local_28: i32 = 0;
        let mut local_2c: i32 = u_var6.wrapping_mul(2).wrapping_mul(param_5 as u32) as i32;
        let mut local_30: i32 = (u_var6 >> 2)
            .wrapping_add(i_var5 as u32)
            .wrapping_sub(u_var6.wrapping_mul(param_5 as u32))
            as i32;
        let u6x2 = u_var6.wrapping_mul(2) as i32;

        let plot = |x: i32, y: i32| {
            if i_var10 <= x && x <= i_var12 && i_var11 <= y && y <= i_var13 {
                let addr = y.wrapping_mul(i_var9).wrapping_add(base).wrapping_add(x);
                mech_trace::wr!("100594ec", (addr as *mut u8), write, u_var7);
            }
        };

        while local_28.wrapping_sub(local_2c) < 0 {
            plot(i_var3.wrapping_add(local_10), i_var4.wrapping_add(local_14));
            plot(i_var3.wrapping_add(local_10), i_var4.wrapping_sub(local_14));
            plot(i_var3.wrapping_sub(local_10), i_var4.wrapping_add(local_14));
            plot(i_var3.wrapping_sub(local_10), i_var4.wrapping_sub(local_14));
            if -1 < local_30 {
                local_14 = local_14.wrapping_sub(1);
                param_5 = param_5.wrapping_sub(1);
                local_2c = local_2c.wrapping_sub(u6x2);
                local_30 = local_30.wrapping_sub(local_2c);
            }
            local_10 = local_10.wrapping_add(1);
            local_28 = local_28.wrapping_add(i_var5.wrapping_mul(2));
            local_30 = local_30.wrapping_add(local_28).wrapping_add(i_var5);
        }
        let d = u_var6.wrapping_sub(i_var5 as u32) as i32;
        let t = (d >> 1)
            .wrapping_add(d)
            .wrapping_sub(local_28)
            .wrapping_sub(local_2c);
        local_30 = local_30.wrapping_add(t >> 1);
        let mut u_var8: u32;
        loop {
            plot(i_var3.wrapping_add(local_10), i_var4.wrapping_add(local_14));
            plot(i_var3.wrapping_add(local_10), i_var4.wrapping_sub(local_14));
            plot(i_var3.wrapping_sub(local_10), i_var4.wrapping_add(local_14));
            plot(i_var3.wrapping_sub(local_10), i_var4.wrapping_sub(local_14));
            if local_30 < 0 {
                local_10 = local_10.wrapping_add(1);
                local_28 = local_28.wrapping_add(i_var5.wrapping_mul(2));
                local_30 = local_30.wrapping_add(local_28);
            }
            local_14 = local_14.wrapping_sub(1);
            local_2c = local_2c.wrapping_sub(u6x2);
            u_var8 = (local_2c as u32).wrapping_sub(u_var6);
            local_30 = local_30.wrapping_sub(u_var8 as i32);
            param_5 = param_5.wrapping_sub(1);
            if !(-1 < param_5) {
                break;
            }
        }
        mech_trace::rt!("100594ec", u_var8)
    }
}
