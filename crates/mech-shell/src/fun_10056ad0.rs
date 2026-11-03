// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056c20(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_10056bb0(param_1: *mut i32, param_2: i16, param_3: i16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056ad0(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
) -> bool {
    mech_trace::enter!("10056ad0", param_1, param_2, param_3, param_4, param_5);
    let mut local: [i32; 4] = [0; 4];
    // SAFETY: the pointers are the caller's own data and param_1 is the
    // address of the caller's structure; reads, writes and calls happen in
    // the original's order.
    unsafe {
        loop {
            let w13 = mech_trace::rd!(
                "10056ad0",
                ((param_1 as usize).wrapping_add(0x13) as *const i32),
                read_unaligned
            );
            let i_var1 = mech_trace::rd!("10056ad0", param_3.add(1), read).wrapping_add(w13);
            mech_trace::wr!("10056ad0", param_3.add(3), write, i_var1);
            if mech_trace::rd!("10056ad0", param_2.add(3), read) < i_var1 {
                let a = mech_trace::rd!("10056ad0", param_2.add(1), read)
                    .wrapping_sub(mech_trace::rd!("10056ad0", param_3.add(1), read));
                let b = mech_trace::rd!("10056ad0", param_3, read)
                    .wrapping_sub(mech_trace::rd!("10056ad0", param_3.add(2), read))
                    .wrapping_add(25000);
                FUN_10056c20(param_3, a, b);
                mech_trace::wr!(
                    "10056ad0",
                    param_3,
                    write,
                    mech_trace::rd!("10056ad0", param_3.add(2), read)
                );
            }
            let w13 = mech_trace::rd!(
                "10056ad0",
                ((param_1 as usize).wrapping_add(0x13) as *const i32),
                read_unaligned
            );
            local[0] = mech_trace::rd!("10056ad0", param_3.add(2), read).wrapping_add(w13);
            local[1] = mech_trace::rd!("10056ad0", param_3.add(1), read);
            local[2] = mech_trace::rd!("10056ad0", param_3.add(2), read);
            local[3] = mech_trace::rd!("10056ad0", param_3.add(3), read);
            let mut v = mech_trace::rd!("10056ad0", param_3, read);
            if v <= local[0] {
                v = local[0];
            }
            mech_trace::wr!("10056ad0", param_3, write, v);
            let wb = mech_trace::rd!(
                "10056ad0",
                ((param_1 as usize).wrapping_add(0xb) as *const i32),
                read_unaligned
            );
            mech_trace::wr!("10056ad0", param_4, write, local[1].wrapping_add(wb / 2));
            let w13 = mech_trace::rd!(
                "10056ad0",
                ((param_1 as usize).wrapping_add(0x13) as *const i32),
                read_unaligned
            );
            mech_trace::wr!("10056ad0", param_5, write, local[2].wrapping_add(w13 / 2));
            let wb = mech_trace::rd!(
                "10056ad0",
                ((param_1 as usize).wrapping_add(0xb) as *const i32),
                read_unaligned
            );
            FUN_10056c20(param_3, wb.wrapping_add(25000), 0);
            let r = mech_trace::cr!(
                "10056ad0",
                "10056bb0",
                FUN_10056bb0(local.as_mut_ptr().add(3), 0, 0)
            );
            if r == 0 {
                break;
            }
        }
        mech_trace::rt!(
            "10056ad0",
            mech_trace::rd!("10056ad0", param_3.add(2), read)
                < mech_trace::rd!("10056ad0", param_2, read)
        )
    }
}
