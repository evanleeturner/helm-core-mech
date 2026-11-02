// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10089060(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
) {
    mech_trace::enter!("10089060", param_1, param_2, param_3, param_4);
    // SAFETY: the caller supplies a 12-entry i32 matrix in param_1 and valid
    // pointers for the three in/out values, read and written as the original does.
    unsafe {
        let i_var1 = mech_trace::rd!("10089060", param_2, read_unaligned);
        let i_var2 = mech_trace::rd!("10089060", param_3, read_unaligned);
        let i_var3 = mech_trace::rd!("10089060", param_4, read_unaligned);

        let m = |i: usize| -> i64 {
            mech_trace::rd!("10089060", param_1.add(i), read_unaligned) as i64
        };
        let v1 = i_var1 as i64;
        let v2 = i_var2 as i64;
        let v3 = i_var3 as i64;

        let u_var5: u64 = (m(1).wrapping_mul(v2) as u64)
            .wrapping_add(m(0).wrapping_mul(v1) as u64)
            .wrapping_add(m(2).wrapping_mul(v3) as u64);
        let u_var8 = u_var5 as u32;

        let u_var6: u64 = (m(4).wrapping_mul(v2) as u64)
            .wrapping_add(m(3).wrapping_mul(v1) as u64)
            .wrapping_add(m(5).wrapping_mul(v3) as u64);
        let u_var9 = u_var6 as u32;

        let i_var4 = mech_trace::rd!("10089060", param_1.add(10), read_unaligned);

        let u_var7: u64 = (m(7).wrapping_mul(v2) as u64)
            .wrapping_add(m(6).wrapping_mul(v1) as u64)
            .wrapping_add(m(8).wrapping_mul(v3) as u64);
        let u_var10 = u_var7 as u32;

        let i_var1b = mech_trace::rd!("10089060", param_1.add(0xb), read_unaligned);

        let r2 = ((u_var8 >> 0x1d) | ((u_var5 >> 0x20) as u32).wrapping_mul(8))
            .wrapping_add(mech_trace::rd!("10089060", param_1.add(9), read_unaligned) as u32)
            .wrapping_add(((u_var8 >> 0x1c) & 1 != 0) as u32);
        mech_trace::wr!("10089060", param_2, write_unaligned, r2 as i32);

        let r3 = ((u_var9 >> 0x1d) | ((u_var6 >> 0x20) as u32).wrapping_mul(8))
            .wrapping_add(i_var4 as u32)
            .wrapping_add(((u_var9 >> 0x1c) & 1 != 0) as u32);
        mech_trace::wr!("10089060", param_3, write_unaligned, r3 as i32);

        let r4 = ((u_var10 >> 0x1d) | ((u_var7 >> 0x20) as u32).wrapping_mul(8))
            .wrapping_add(i_var1b as u32)
            .wrapping_add(((u_var10 >> 0x1c) & 1 != 0) as u32);
        mech_trace::wr!("10089060", param_4, write_unaligned, r4 as i32);
    }
}
