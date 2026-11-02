// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d6f0(param_1: i32, param_2: i32) -> u8 {
    mech_trace::enter!("1000d6f0", param_1, param_2);
    // SAFETY: param_1 + param_2 * 0x12 + 0x760 addresses a record in the
    // game's own memory; it is read at the same offsets as the original.
    unsafe {
        let mut i_var4: i32 = 0;
        let mut i_var7: i32 = 0;
        let p = param_2
            .wrapping_mul(0x12)
            .wrapping_add(0x760)
            .wrapping_add(param_1) as usize as *const u8;
        let u_var2: u16 = mech_trace::rd!("1000d6f0", p.byte_add(16).cast::<u16>(), read_unaligned);
        let mut i_var5: i32 = mech_trace::rd!("1000d6f0", p.cast::<i32>(), read_unaligned);
        let i_var6: i32 = ((u_var2 & 0xf0) >> 4) as i16 as i32;
        if i_var5 < 0 {
            i_var5 = 0;
        }
        let mut i_var3: i32 =
            mech_trace::rd!("1000d6f0", p.byte_add(12).cast::<i32>(), read_unaligned);
        if i_var3 < 0 {
            i_var3 = 0;
        }
        if i_var6 != 0 {
            i_var7 = i_var3.wrapping_add(i_var5).wrapping_mul(-0x30000)
                / (i_var6 as u32).wrapping_shl(16) as i32;
            i_var7 = i_var7.wrapping_add(0xf);
        }
        if i_var7 < 1 {
            i_var7 = 0;
        } else if 0xf < i_var7 {
            i_var7 = 0xf;
        }
        i_var5 = mech_trace::rd!("1000d6f0", p.cast::<i32>(), read_unaligned);
        if i_var5 < 0 {
            i_var5 = 0;
        }
        let i_var6b: i32 = mech_trace::rd!("1000d6f0", p.byte_add(8).cast::<i32>(), read_unaligned);
        i_var3 = i_var6b;
        if i_var6b < 0 {
            i_var3 = 0;
        }
        if (u_var2 & 0xf) != 0 {
            i_var4 = i_var3.wrapping_add(i_var5).wrapping_mul(-0x30000)
                / ((u_var2 & 0xf) as u32).wrapping_shl(16) as i32;
            i_var4 = i_var4.wrapping_add(0xf);
        }
        if i_var4 < 1 {
            i_var4 = 0;
        } else if 0xf < i_var4 {
            i_var4 = 0xf;
        }
        if i_var7 <= i_var4 {
            i_var7 = i_var4;
        }
        if i_var6b < 1
            && mech_trace::rd!("1000d6f0", p.byte_add(12).cast::<i32>(), read_unaligned) < 1
            && mech_trace::rd!("1000d6f0", p.cast::<i32>(), read_unaligned) < 1
        {
            return mech_trace::rt!("1000d6f0", 3);
        }
        if 0xb < i_var7 {
            return mech_trace::rt!("1000d6f0", 2);
        }
        mech_trace::rt!("1000d6f0", (0 < i_var7) as u8)
    }
}
