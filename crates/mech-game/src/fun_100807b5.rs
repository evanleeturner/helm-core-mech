// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c96b0: i32;
    static mut DAT_100c9ec4: i32;
    static mut DAT_100c9ec8: i32;
    static mut DAT_100c9ec0: i32;
    static mut DAT_10165944: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100807b5(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut local_8: u32 = param_1 as u32;
    let mut local_c: u32 = param_2 as u32;
    let mut local_10: u32 = param_3 as u32;
    // SAFETY: the globals are the game's own memory.
    unsafe {
        if DAT_100c96b0 != 0 {
            local_10 = 0;
            local_c = 0;
            local_8 = 0;
        }
        let u_var4 = (DAT_100c9ec4 as u32).wrapping_sub(local_8);
        let b = DAT_100c9ec4 < (local_8 as i32);
        local_8 = u_var4;
        if b {
            local_8 = 0u32.wrapping_sub(u_var4);
        }
        let u_var5 = (DAT_100c9ec8 as u32).wrapping_sub(local_c);
        let b = DAT_100c9ec8 < (local_c as i32);
        local_c = u_var5;
        if b {
            local_c = 0u32.wrapping_sub(u_var5);
        }
        let u_var6 = (DAT_100c9ec0 as u32).wrapping_sub(local_10);
        let b = DAT_100c9ec0 < (local_10 as i32);
        local_10 = u_var6;
        if b {
            local_10 = 0u32.wrapping_sub(u_var6);
        }
        let mut u_var11 = local_8 | local_c | local_10;
        if u_var11 == 0 {
            return 0x7f;
        }
        let sum = ((u_var4 as i32 as i64).wrapping_mul(param_4 as i64) as u64)
            .wrapping_add((u_var5 as i32 as i64).wrapping_mul(param_5 as i64) as u64)
            .wrapping_add((u_var6 as i32 as i64).wrapping_mul(param_6 as i64) as u64);
        let mut l_var3: i64 = (sum as i64) >> 16;
        let u_var4 = l_var3 as u32;
        let i_var12 = ((l_var3 as u64) >> 32) as i32;
        let mut s_var8: i16 = 0;
        if (u_var11 & 0xff000000) == 0 {
            if (u_var11 & 0xffff0000) != 0 {
                s_var8 = 8;
                u_var11 >>= 8;
            }
        } else {
            s_var8 = 0x10;
            u_var11 >>= 16;
        }
        let mut s_var2: i16 = 0xf;
        if (u_var11 as u16) != 0 {
            while ((u_var11 as u16) >> (s_var2 as u32)) == 0 {
                s_var2 = s_var2.wrapping_sub(1);
            }
        }
        let s_var9 = s_var8.wrapping_add(s_var2).wrapping_sub(7);
        if s_var9 != 0 {
            let mut b_var7 = s_var9 as u8;
            if s_var8.wrapping_add(s_var2) < 7 {
                b_var7 = b_var7.wrapping_neg();
                let n = (b_var7 & 0x1f) as u32;
                local_8 = local_8.wrapping_shl(n);
                local_c = local_c.wrapping_shl(n);
                local_10 = local_10.wrapping_shl(n);
                let hi = ((i_var12 as u32).wrapping_shl(n))
                    | u_var4.wrapping_shr(0x20u32.wrapping_sub(n));
                let lo = u_var4.wrapping_shl(n);
                l_var3 = (((hi as u64) << 32) | (lo as u64)) as i64;
            } else {
                let n = (b_var7 & 0x1f) as u32;
                local_8 = local_8.wrapping_shr(n);
                local_c = local_c.wrapping_shr(n);
                local_10 = local_10.wrapping_shr(n);
                let hi = i_var12.wrapping_shr(n) as u32;
                let lo =
                    u_var4.wrapping_shr(n) | (i_var12 as u32).wrapping_shl(0x20u32.wrapping_sub(n));
                l_var3 = (((hi as u64) << 32) | (lo as u64)) as i64;
            }
        }
        let a2 = ((local_8 as u8 as u16).wrapping_mul(local_8 as u8 as u16)) as u32;
        let b2 = ((local_c as u8 as u16).wrapping_mul(local_c as u8 as u16)) as u32;
        let c2 = ((local_10 as u8 as u16).wrapping_mul(local_10 as u8 as u16)) as u32;
        let u_var10: u16 = a2.wrapping_add(b2) as u16;
        let carry1: u16 = ((a2 + b2) > 0xffff) as u16;
        let carry2: u16 = (((u_var10 as u32) + c2) > 0xffff) as u16;
        let t: u16 = ((u_var10.wrapping_add(c2 as u16)) >> 7) | carry1.wrapping_add(carry2);
        let idx = ((t as i32).wrapping_mul(0x200) & 0xfffe) as u32;
        let addr = idx.wrapping_add(DAT_10165944 as u32);
        let i_var12 = (addr as *const i16).read_unaligned() as i32;
        let mut local_14: i16 = 0;
        if i_var12 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        } else {
            if i_var12 == -1 {
                l_var3 = l_var3.wrapping_neg();
            } else {
                l_var3 /= i_var12 as i64;
            }
            if l_var3 != (l_var3 as i32 as i64) {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            } else {
                local_14 = l_var3 as i16;
            }
        }
        local_14 as i32
    }
}
