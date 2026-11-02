// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160eb0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10080130(param_1: i32) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10035f3b() -> i32;
}

#[inline(always)]
fn lerp(diff: i32, t: i32, d: i32, base: i32) -> i32 {
    let p = (diff as i64).wrapping_mul(t as i64);
    ((p / (d as i64)) as i32).wrapping_add(base)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10080241(param_1: i32, param_2: i32) -> *mut i32 {
    // SAFETY: the game's own functions and globals are used as the original
    // does; the pointers they return and the parameters' fields at offset
    // 0x24 point at the game's records, read and written where the C does.
    unsafe {
        let mut local_24 = ((param_1.wrapping_add(0x24)) as *const *mut i32).read_unaligned();
        if local_24.is_null() {
            local_24 = FUN_10080130(param_1);
        }
        let mut local_2c = ((param_2.wrapping_add(0x24)) as *const *mut i32).read_unaligned();
        if local_2c.is_null() {
            local_2c = FUN_10080130(param_2);
        }
        let pi_var8 = FUN_10035f3b() as *mut i32;

        let mut local_3c = local_24.read_unaligned();
        let mut local_48 = local_24.add(1).read_unaligned();
        let mut local_18 = local_24.add(5).read_unaligned();
        let mut local_30 = local_24.add(6).read_unaligned();
        let mut i_var9 = local_24.add(2).read_unaligned();
        let mut local_44 = local_24.add(8).read_unaligned();
        let mut local_8 = local_24.add(9).read_unaligned();
        let mut local_20 = local_24.add(10).read_unaligned();
        let i_var1 = local_2c.read_unaligned();
        let i_var2 = local_2c.add(1).read_unaligned();
        let i_var3 = local_2c.add(5).read_unaligned();
        let i_var4 = local_2c.add(6).read_unaligned();
        let mut i_var10 = local_2c.add(2).read_unaligned();
        let i_var5 = local_2c.add(8).read_unaligned();
        let i_var6 = local_2c.add(9).read_unaligned();
        let i_var7 = local_2c.add(10).read_unaligned();

        if i_var9 < i_var7 {
            i_var10 = i_var10.wrapping_sub(i_var9);
            if i_var10 != 0 {
                i_var9 = (DAT_10160eb0 as i32).wrapping_sub(i_var9);
                local_3c = lerp(i_var1.wrapping_sub(local_3c), i_var9, i_var10, local_3c);
                local_48 = lerp(i_var2.wrapping_sub(local_48), i_var9, i_var10, local_48);
                local_18 = lerp(i_var3.wrapping_sub(local_18), i_var9, i_var10, local_18);
                local_30 = lerp(i_var4.wrapping_sub(local_30), i_var9, i_var10, local_30);
                local_44 = lerp(i_var5.wrapping_sub(local_44), i_var9, i_var10, local_44);
                local_8 = lerp(i_var6.wrapping_sub(local_8), i_var9, i_var10, local_8);
                local_20 = lerp(i_var7.wrapping_sub(local_20), i_var9, i_var10, local_20);
            }
        } else {
            i_var9 = i_var9.wrapping_sub(i_var10);
            if i_var9 != 0 {
                i_var10 = (DAT_10160eb0 as i32).wrapping_sub(i_var10);
                local_3c = lerp(local_3c.wrapping_sub(i_var1), i_var10, i_var9, i_var1);
                local_48 = lerp(local_48.wrapping_sub(i_var2), i_var10, i_var9, i_var2);
                local_18 = lerp(local_18.wrapping_sub(i_var3), i_var10, i_var9, i_var3);
                local_30 = lerp(local_30.wrapping_sub(i_var4), i_var10, i_var9, i_var4);
                local_44 = lerp(local_44.wrapping_sub(i_var5), i_var10, i_var9, i_var5);
                local_8 = lerp(local_8.wrapping_sub(i_var6), i_var10, i_var9, i_var6);
                local_20 = lerp(local_20.wrapping_sub(i_var7), i_var10, i_var9, i_var7);
            }
        }

        pi_var8.write_unaligned(local_3c);
        pi_var8.add(1).write_unaligned(local_48);
        pi_var8.add(2).write_unaligned(DAT_10160eb0 as i32);
        pi_var8.add(5).write_unaligned(local_18);
        pi_var8.add(6).write_unaligned(local_30);
        pi_var8.add(8).write_unaligned(local_44);
        pi_var8.add(9).write_unaligned(local_8);
        pi_var8.add(10).write_unaligned(local_20);
        pi_var8
    }
}
