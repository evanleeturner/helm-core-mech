// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100891d0(param_1: *mut i32, param_2: i32) {
    mech_trace::enter!("100891d0", param_1, param_2);
    let mut local_28: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;

    // SAFETY: param_1 points at the caller's array of nine i32 values.
    unsafe {
        if param_2 == 0 {
            local_10 = mech_trace::rd!("100891d0", param_1.add(1), read_unaligned);
            local_24 = mech_trace::rd!("100891d0", param_1.add(4), read_unaligned);
            local_8 = mech_trace::rd!("100891d0", param_1.add(7), read_unaligned);
            local_18 = mech_trace::rd!("100891d0", param_1.add(2), read_unaligned);
            local_28 = mech_trace::rd!("100891d0", param_1.add(5), read_unaligned);
            local_c = mech_trace::rd!("100891d0", param_1.add(8), read_unaligned);
        } else if param_2 == 1 {
            local_10 = mech_trace::rd!("100891d0", param_1.add(2), read_unaligned);
            local_24 = mech_trace::rd!("100891d0", param_1.add(5), read_unaligned);
            local_8 = mech_trace::rd!("100891d0", param_1.add(8), read_unaligned);
            local_18 = mech_trace::rd!("100891d0", param_1, read_unaligned);
            local_28 = mech_trace::rd!("100891d0", param_1.add(3), read_unaligned);
            local_c = mech_trace::rd!("100891d0", param_1.add(6), read_unaligned);
        } else if param_2 == 2 {
            local_10 = mech_trace::rd!("100891d0", param_1, read_unaligned);
            local_24 = mech_trace::rd!("100891d0", param_1.add(3), read_unaligned);
            local_8 = mech_trace::rd!("100891d0", param_1.add(6), read_unaligned);
            local_18 = mech_trace::rd!("100891d0", param_1.add(1), read_unaligned);
            local_28 = mech_trace::rd!("100891d0", param_1.add(4), read_unaligned);
            local_c = mech_trace::rd!("100891d0", param_1.add(7), read_unaligned);
        }

        let f = |a: i32, b: i32, c: i32, d: i32| -> i32 {
            let l1 = ((a as i64).wrapping_mul(b as i64)) as u64;
            let l2 = ((c as i64).wrapping_mul(d as i64)) as u64;
            let l = l1.wrapping_sub(l2);
            let u_lo = l as u32;
            let u_hi = (l >> 32) as u32;
            ((u_lo >> 0x1d) | u_hi.wrapping_mul(8)).wrapping_add(((u_lo >> 0x1c) & 1 != 0) as u32)
                as i32
        };

        let i_var3 = f(local_24, local_c, local_28, local_8);
        let i_var4 = f(local_8, local_18, local_c, local_10);
        let i_var5 = f(local_10, local_28, local_18, local_24);

        if param_2 == 0 {
            mech_trace::wr!("100891d0", param_1, write_unaligned, i_var3);
            mech_trace::wr!("100891d0", param_1.add(3), write_unaligned, i_var4);
            mech_trace::wr!("100891d0", param_1.add(6), write_unaligned, i_var5);
        } else if param_2 == 1 {
            mech_trace::wr!("100891d0", param_1.add(1), write_unaligned, i_var3);
            mech_trace::wr!("100891d0", param_1.add(4), write_unaligned, i_var4);
            mech_trace::wr!("100891d0", param_1.add(7), write_unaligned, i_var5);
        } else if param_2 == 2 {
            mech_trace::wr!("100891d0", param_1.add(2), write_unaligned, i_var3);
            mech_trace::wr!("100891d0", param_1.add(5), write_unaligned, i_var4);
            mech_trace::wr!("100891d0", param_1.add(8), write_unaligned, i_var5);
        }
    }
}
