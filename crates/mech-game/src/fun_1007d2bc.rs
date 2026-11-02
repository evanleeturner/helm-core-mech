// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d2bc(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    mech_trace::enter!("1007d2bc", param_1, param_2, param_3);
    let mut local_c: i32 = 0;
    // SAFETY: the caller passes addresses of the game's own structures;
    // reads happen in the original's order.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1007d2bc",
            ((param_3 as u32).wrapping_add(0x30) as *const i32),
            read_unaligned
        );
        if param_1 != 0 && param_2 != 0 {
            let mut local_24: i32 = 0;
            while local_24
                < mech_trace::rd!(
                    "1007d2bc",
                    ((param_1 as u32).wrapping_add(2) as *const u16),
                    read_unaligned
                ) as i32
                && local_c == 0
            {
                let off = mech_trace::rd!(
                    "1007d2bc",
                    ((param_1 as u32).wrapping_add(4) as *const i32),
                    read_unaligned
                );
                let idx = mech_trace::rd!(
                    "1007d2bc",
                    (off.wrapping_add(local_24).wrapping_add(param_1) as u32 as *const u8),
                    read
                );
                let i_var2 = (idx as i32).wrapping_mul(0x44).wrapping_add(param_2);
                let p2 = |o: i32| {
                    mech_trace::rd!(
                        "1007d2bc",
                        (i_var2.wrapping_add(o) as u32 as *const i32),
                        read_unaligned
                    )
                };
                let p3 = |o: i32| {
                    mech_trace::rd!(
                        "1007d2bc",
                        (param_3.wrapping_add(o) as u32 as *const i32),
                        read_unaligned
                    )
                };
                if p2(0xc) <= i_var1.wrapping_add(p3(0x14))
                    && p3(0x14).wrapping_sub(i_var1) <= p2(0xc)
                    && p2(0x10) <= i_var1.wrapping_add(p3(0x18))
                    && p3(0x18).wrapping_sub(i_var1) <= p2(0x10)
                    && p2(0x14) <= p3(0x1c).wrapping_add(i_var1)
                    && p3(0x1c).wrapping_sub(i_var1) <= p2(0x14)
                {
                    local_c = 1;
                }
                local_24 = local_24.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("1007d2bc", local_c)
}
