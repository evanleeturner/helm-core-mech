// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10089118(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
) {
    mech_trace::enter!("10089118", param_1, param_2, param_3, param_4);
    // SAFETY: the caller supplies a 9-element matrix in param_1 and valid
    // pointers for the three vector components, read then written in order.
    unsafe {
        let i_var1 = mech_trace::rd!("10089118", param_2, read_unaligned);
        let i_var2 = mech_trace::rd!("10089118", param_3, read_unaligned);
        let i_var3 = mech_trace::rd!("10089118", param_4, read_unaligned);

        let m0 = mech_trace::rd!("10089118", param_1, read_unaligned);
        let m1 = mech_trace::rd!("10089118", param_1.add(1), read_unaligned);
        let m2 = mech_trace::rd!("10089118", param_1.add(2), read_unaligned);
        let m3 = mech_trace::rd!("10089118", param_1.add(3), read_unaligned);
        let m4 = mech_trace::rd!("10089118", param_1.add(4), read_unaligned);
        let m5 = mech_trace::rd!("10089118", param_1.add(5), read_unaligned);
        let m6 = mech_trace::rd!("10089118", param_1.add(6), read_unaligned);
        let m7 = mech_trace::rd!("10089118", param_1.add(7), read_unaligned);
        let m8 = mech_trace::rd!("10089118", param_1.add(8), read_unaligned);

        let p = |a: i32, b: i32| -> u64 { (a as i64).wrapping_mul(b as i64) as u64 };

        let u_var4: u64 = p(m1, i_var2)
            .wrapping_add(p(m0, i_var1))
            .wrapping_add(p(m2, i_var3));
        let u_var7 = u_var4 as u32;

        let u_var5: u64 = p(m4, i_var2)
            .wrapping_add(p(m3, i_var1))
            .wrapping_add(p(m5, i_var3));
        let u_var8 = u_var5 as u32;

        let u_var6: u64 = p(m7, i_var2)
            .wrapping_add(p(m6, i_var1))
            .wrapping_add(p(m8, i_var3));
        let u_var9 = u_var6 as u32;

        mech_trace::wr!(
            "10089118",
            param_2,
            write_unaligned,
            ((u_var7 >> 0x1d) | ((u_var4 >> 0x20) as u32).wrapping_shl(3))
                .wrapping_add(((u_var7 >> 0x1c) & 1 != 0) as u32) as i32
        );
        mech_trace::wr!(
            "10089118",
            param_3,
            write_unaligned,
            ((u_var8 >> 0x1d) | ((u_var5 >> 0x20) as u32).wrapping_shl(3))
                .wrapping_add(((u_var8 >> 0x1c) & 1 != 0) as u32) as i32
        );
        mech_trace::wr!(
            "10089118",
            param_4,
            write_unaligned,
            ((u_var9 >> 0x1d) | ((u_var6 >> 0x20) as u32).wrapping_shl(3))
                .wrapping_add(((u_var9 >> 0x1c) & 1 != 0) as u32) as i32
        );
    }
}
