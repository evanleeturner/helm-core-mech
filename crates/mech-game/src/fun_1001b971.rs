// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10165944: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

fn cross(a1: i32, a2: i32, b1: i32, b2: i32) -> (u32, u32, u32, u32) {
    let a = (a1 as i64).wrapping_mul(a2 as i64);
    let b = (b1 as i64).wrapping_mul(b2 as i64);
    let d = (a as u64).wrapping_sub(b as u64);
    let ad = if a < b { d.wrapping_neg() } else { d };
    (d as u32, (d >> 32) as u32, ad as u32, (ad >> 32) as u32)
}

unsafe fn div_part(l: u32, d: u32) -> i32 {
    let num: u64 = ((((((l as i32) >> 31) as u32).wrapping_shl(13)) | (l >> 19)) as u64) << 32
        | (l.wrapping_shl(13) as u64);
    let q: i64;
    if d == 0 {
        // SAFETY: raising the structured exception as the original does.
        unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
        q = 0;
    } else {
        q = (num as i64) / ((d as i32) as i64);
        if q != (q as i32) as i64 {
            // SAFETY: raising the structured exception as the original does.
            unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
        }
    }
    0u32.wrapping_sub(q as i32 as u32) as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b971(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
    param_9: i32,
    param_10: *mut i32,
    param_11: *mut i32,
    param_12: *mut i32,
) -> i32 {
    mech_trace::enter!(
        "1001b971", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8,
        param_9, param_10, param_11, param_12
    );
    let (mut local_18, u_var15, u_var7a, u_var16a) = cross(
        param_5.wrapping_sub(param_2),
        param_9.wrapping_sub(param_6),
        param_8.wrapping_sub(param_5),
        param_6.wrapping_sub(param_3),
    );
    let (mut local_20, u_var17, u_var8, u_var18) = cross(
        param_6.wrapping_sub(param_3),
        param_7.wrapping_sub(param_4),
        param_9.wrapping_sub(param_6),
        param_4.wrapping_sub(param_1),
    );
    let (mut local_28, u_var19, u_var9, u_var20) = cross(
        param_4.wrapping_sub(param_1),
        param_8.wrapping_sub(param_5),
        param_7.wrapping_sub(param_4),
        param_5.wrapping_sub(param_2),
    );

    let mut u_var7 = u_var9 | u_var8 | u_var7a;
    let mut u_var16 = u_var20 | u_var18 | u_var16a;
    let mut local_24: i16;

    if u_var16 == 0 {
        if u_var7 == 0 {
            // SAFETY: caller supplies valid output pointers.
            unsafe {
                mech_trace::dw!("1001b971", param_12, -1);
                mech_trace::dw!("1001b971", param_11, mech_trace::dr!("1001b971", param_12));
                mech_trace::dw!("1001b971", param_10, mech_trace::dr!("1001b971", param_11));
            }
            return mech_trace::rt!("1001b971", 0);
        }
        let mut s_var5: i16 = 0x18;
        if (u_var7 & 0xffff0000) != 0 {
            s_var5 = 8;
            u_var7 >>= 0x10;
        }
        if (u_var7 & 0xff00) != 0 {
            s_var5 = s_var5.wrapping_sub(8);
            u_var7 >>= 8;
        }
        let mut s_var2: i16 = 0xf;
        let u_var14 = (u_var7.wrapping_shl(8)) as u16;
        if u_var14 != 0 {
            while ((u_var14 as u32) >> (s_var2 as u32 & 0xffff_ffff)) == 0 {
                s_var2 = s_var2.wrapping_sub(1);
            }
        }
        let s_var11: i16 = (0xdi16.wrapping_sub(s_var2)).wrapping_add(s_var5);
        local_24 = 0;
        let mut skip_add = false;
        if s_var11 != 0 {
            if s_var11 > 0 {
                let b = s_var11 as u8;
                local_18 = local_18.wrapping_shl((b & 0x1f) as u32);
                local_20 = local_20.wrapping_shl((b & 0x1f) as u32);
                local_28 = local_28.wrapping_shl((b & 0x1f) as u32);
                local_24 = 0x1di16.wrapping_sub(s_var11);
                skip_add = true;
            } else {
                local_24 = s_var11.wrapping_neg();
                let cnt = (local_24 as u8 as u32) & 0x1f;
                if cnt != 0 {
                    local_18 = (local_18 >> cnt) | u_var15.wrapping_shl(0x20 - cnt);
                    local_20 = (local_20 >> cnt) | u_var17.wrapping_shl(0x20 - cnt);
                    local_28 = (local_28 >> cnt) | u_var19.wrapping_shl(0x20 - cnt);
                }
            }
        }
        if !skip_add {
            local_24 = local_24.wrapping_add(0x1d);
        }
    } else {
        local_24 = 0;
        if (u_var16 & 0xffff0000) != 0 {
            local_24 = 0x10;
            u_var16 >>= 0x10;
        }
        if (u_var16 & 0xff00) != 0 {
            local_24 = local_24.wrapping_add(8);
            u_var16 >>= 8;
        }
        let mut s_var5: i16 = 0xf;
        let u_var14 = (u_var16.wrapping_shl(8)) as u16;
        if u_var14 != 0 {
            while ((u_var14 as u32) >> (s_var5 as u32)) == 0 {
                s_var5 = s_var5.wrapping_sub(1);
            }
        }
        local_24 = s_var5.wrapping_sub(5).wrapping_add(local_24);
        let cnt = (local_24 as u8 as u32) & 0x1f;
        if cnt != 0 {
            local_18 = (local_18 >> cnt) | u_var15.wrapping_shl(0x20 - cnt);
            local_20 = (local_20 >> cnt) | u_var17.wrapping_shl(0x20 - cnt);
            local_28 = (local_28 >> cnt) | u_var19.wrapping_shl(0x20 - cnt);
        }
        local_24 = local_24.wrapping_add(0x1d);
    }

    let s1 = (local_18 >> 0x10) as i16 as i32;
    let i_var1 = s1.wrapping_mul(s1);
    let u_var14 = i_var1 as u16;
    let s2 = (local_20 >> 0x10) as i16 as i32;
    let i_var12 = s2.wrapping_mul(s2);
    let u_var4 = i_var12 as u16;
    let s3 = (local_28 >> 0x10) as i16 as i32;
    let i_var13 = s3.wrapping_mul(s3);

    let carry1: u16 = if (u_var14 as u32 + u_var4 as u32) > 0xffff {
        1
    } else {
        0
    };
    let carry2: u16 = if ((u_var14.wrapping_add(u_var4)) as u32 + (i_var13 as u16) as u32) > 0xffff
    {
        1
    } else {
        0
    };
    let sum: u16 = ((i_var1 as u32) >> 0x10) as u16;
    let sum = sum
        .wrapping_add(((i_var12 as u32) >> 0x10) as u16)
        .wrapping_add(carry1)
        .wrapping_add(((i_var13 as u32) >> 0x10) as u16)
        .wrapping_add(carry2);
    let idx: u16 = (sum >> 4).wrapping_shl(1);

    // SAFETY: the table is the game's own; idx stays within it; outputs are caller's pointers.
    unsafe {
        let base = (&raw mut DAT_10165944) as u32;
        let addr = (idx as u32).wrapping_add(base) as *const u16;
        let u_var6 = mech_trace::rd!("1001b971", addr, read_unaligned) as u32;

        mech_trace::dw!("1001b971", param_10, div_part(local_18, u_var6));
        mech_trace::dw!("1001b971", param_11, div_part(local_20, u_var6));
        mech_trace::dw!("1001b971", param_12, div_part(local_28, u_var6));
    }

    mech_trace::rt!("1001b971", local_24 as i32)
}
