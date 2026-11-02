// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100150df(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_100141fb(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ffd8(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    mech_trace::enter!("1005ffd8", param_1, param_2, param_3, param_4);
    let mut local_8: i32 = 0;
    let mut b_var1: i32 = 0;
    let mut local_c: i32 = 0;
    let mut i_var2: i32;
    // SAFETY: param_1 points at the game's own structure; all reads follow
    // the original's pointer chain and the callees are the game's own.
    unsafe {
        loop {
            if b_var1 != 0 {
                return mech_trace::rt!("1005ffd8", 0);
            }
            let p20 = mech_trace::rd!(
                "1005ffd8",
                (param_1.wrapping_add(0x20) as *const i32),
                read_unaligned
            );
            let off = mech_trace::rd!(
                "1005ffd8",
                ((p20.wrapping_add(8)) as *const i32),
                read_unaligned
            );
            i_var2 = off
                .wrapping_add(mech_trace::rd!(
                    "1005ffd8",
                    (param_1.wrapping_add(0x20) as *const i32),
                    read_unaligned
                ))
                .wrapping_add(local_8.wrapping_mul(0x24));
            if mech_trace::rd!(
                "1005ffd8",
                ((i_var2.wrapping_add(0x18)) as *const i32),
                read_unaligned
            ) > 0
            {
                let p = mech_trace::rd!(
                    "1005ffd8",
                    (param_1.wrapping_add(0x20) as *const i32),
                    read_unaligned
                );
                let i_var3 = mech_trace::cr!(
                    "1005ffd8",
                    "100150df",
                    FUN_100150df(i_var2, p.wrapping_add(0x18), param_2, param_4)
                );
                if i_var3 != 0 {
                    break;
                }
            }
            local_8 = local_8.wrapping_add(1);
            let p = mech_trace::rd!(
                "1005ffd8",
                (param_1.wrapping_add(0x20) as *const i32),
                read_unaligned
            );
            if mech_trace::rd!(
                "1005ffd8",
                ((p.wrapping_add(6)) as *const i16),
                read_unaligned
            ) as i32
                == local_8
            {
                b_var1 = 1;
            }
        }
        let p = mech_trace::rd!(
            "1005ffd8",
            (param_1.wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        b_var1 = mech_trace::cr!(
            "1005ffd8",
            "100141fb",
            FUN_100141fb(
                i_var2,
                p.wrapping_add(0x18),
                param_2,
                param_3,
                param_4,
                &raw mut local_c,
            )
        );
    }
    if b_var1 != 0 {
        return mech_trace::rt!("1005ffd8", 1);
    }
    mech_trace::rt!("1005ffd8", 0)
}
