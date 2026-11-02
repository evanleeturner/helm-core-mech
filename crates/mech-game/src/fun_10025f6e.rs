// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100150df(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_100141fb(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i32,
    ) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025f6e(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut i32,
) -> u32 {
    mech_trace::enter!(
        "10025f6e", param_1, param_2, param_3, param_4, param_5, param_6
    );
    if param_1.is_null() {
        return mech_trace::rt!("10025f6e", 0);
    }
    // SAFETY: param_1 points at the caller's node; its child pointers, items
    // and param_6 are the game's own memory, accessed where the original does.
    unsafe {
        if param_3 < mech_trace::rd!("10025f6e", param_1, read_unaligned)
            || mech_trace::rd!("10025f6e", param_1.add(1), read_unaligned) < param_3
            || param_5 < mech_trace::rd!("10025f6e", param_1.add(4), read_unaligned)
            || mech_trace::rd!("10025f6e", param_1.add(5), read_unaligned) < param_5
        {
            return mech_trace::rt!("10025f6e", 0);
        }
        let count = mech_trace::rd!("10025f6e", param_1.add(6), read_unaligned);
        mech_trace::rt!(
            "10025f6e",
            if count == 0 {
                for i in 7..=10usize {
                    let child = mech_trace::rd!("10025f6e", param_1.add(i), read_unaligned) as usize
                        as *mut i32;
                    if FUN_10025f6e(child, param_2, param_3, param_4, param_5, param_6) != 0 {
                        return mech_trace::rt!("10025f6e", 1);
                    }
                }
                mech_trace::wr!("10025f6e", param_6, write_unaligned, 0);
                1
            } else {
                let mut local_c = param_1.add(0xb);
                let mut local_10: i32 = 0;
                while local_10 < mech_trace::rd!("10025f6e", param_1.add(6), read_unaligned) {
                    let i_var2 = mech_trace::rd!("10025f6e", local_c, read_unaligned);
                    local_c = local_c.add(1);
                    if 0 < mech_trace::rd!(
                        "10025f6e",
                        (i_var2 as usize as *const i32).add(6),
                        read_unaligned
                    ) && mech_trace::cr!(
                        "10025f6e",
                        "100150df",
                        FUN_100150df(i_var2, param_2.wrapping_add(0x18), param_3, param_5)
                    ) != 0
                    {
                        mech_trace::cr!(
                            "10025f6e",
                            "100141fb",
                            FUN_100141fb(
                                i_var2,
                                param_2.wrapping_add(0x18),
                                param_3,
                                param_4,
                                param_5,
                                param_6,
                            )
                        );
                        return mech_trace::rt!("10025f6e", 1);
                    }
                    local_10 = local_10.wrapping_add(1);
                }
                mech_trace::wr!("10025f6e", param_6, write_unaligned, 0);
                1
            }
        )
    }
}
