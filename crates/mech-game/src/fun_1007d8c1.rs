// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafa4: u32;
    static mut DAT_100aafb4: i32;
    static mut DAT_100b6814: i32;
    static mut DAT_100b6818: u32;
    static mut DAT_100b681c: *mut u32;
    static mut DAT_100b6824: u32;
    static mut DAT_100b6828: u32;
    static mut DAT_100c96b0: u32;
    static mut DAT_100c96bc: u32;
    static mut DAT_100c9ec0: u32;
    static mut DAT_100c9ec4: u32;
    static mut DAT_100c9ec8: u32;
    static mut PTR_DAT_100aaf68: *const u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008072a(param_1: i32, param_2: i32) -> i32;
    fn FUN_1007d2bc(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d8c1(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    mech_trace::enter!("1007d8c1", param_1, param_2, param_3, param_4);
    // SAFETY: the globals and callees are the game's own memory; the pointer
    // and list nodes are read where the original reads them.
    unsafe {
        let u_var5 = mech_trace::gr!("1007d8c1", "100c9ec8", DAT_100c9ec8);
        let u_var4 = mech_trace::gr!("1007d8c1", "100c9ec4", DAT_100c9ec4);
        let u_var3 = mech_trace::gr!("1007d8c1", "100c9ec0", DAT_100c9ec0);
        let mut local_8: i32 = mech_trace::gr!("1007d8c1", "100c96bc", DAT_100c96bc) as i32;
        let u_var2 = mech_trace::gr!("1007d8c1", "100c96b0", DAT_100c96b0);
        let mut b_var1 = false;
        let b_var7 = mech_trace::gr!("1007d8c1", "100aafa4", DAT_100aafa4) == 3;
        if b_var7 || mech_trace::gr!("1007d8c1", "100b6828", DAT_100b6828) != 0 {
            let p = mech_trace::gr!("1007d8c1", "100aaf68", PTR_DAT_100aaf68);
            mech_trace::gw!(
                "1007d8c1",
                "100c9ec4",
                DAT_100c9ec4,
                mech_trace::rd!("1007d8c1", p.cast::<u32>(), read_unaligned)
            );
            mech_trace::gw!(
                "1007d8c1",
                "100c9ec8",
                DAT_100c9ec8,
                mech_trace::rd!("1007d8c1", p.byte_add(4).cast::<u32>(), read_unaligned)
            );
            mech_trace::gw!(
                "1007d8c1",
                "100c9ec0",
                DAT_100c9ec0,
                mech_trace::rd!("1007d8c1", p.byte_add(8).cast::<u32>(), read_unaligned)
            );
            mech_trace::gw!("1007d8c1", "100c96b0", DAT_100c96b0, 0);
        }
        if mech_trace::gr!("1007d8c1", "100b6818", DAT_100b6818) != 0
            || b_var7
            || mech_trace::gr!("1007d8c1", "100b6828", DAT_100b6828) != 0
        {
            let i = mech_trace::cr!("1007d8c1", "1008072a", FUN_1008072a(param_1, param_2));
            let t = i.wrapping_mul(mech_trace::gr!("1007d8c1", "100b6814", DAT_100b6814));
            let i_var6 = t.wrapping_add((t >> 0x1f) & 0x7f) >> 7;
            if 0 < i_var6 {
                local_8 = local_8.wrapping_add(i_var6);
            }
        }
        mech_trace::gw!("1007d8c1", "100c96b0", DAT_100c96b0, 0);
        if 0 < mech_trace::gr!("1007d8c1", "100aafb4", DAT_100aafb4) && !b_var7 {
            let q =
                param_4.wrapping_shl(0xb) / mech_trace::gr!("1007d8c1", "100aafb4", DAT_100aafb4);
            local_8 = local_8.wrapping_sub(q >> 4);
            if local_8 < 0 {
                local_8 = 0;
            }
        }
        if mech_trace::gr!("1007d8c1", "100b6824", DAT_100b6824) != 0 && local_8 < 0x80 && !b_var7 {
            let mut local_28 = mech_trace::gr!("1007d8c1", "100b681c", DAT_100b681c);
            while !local_28.is_null() && !b_var1 {
                let r = mech_trace::cr!(
                    "1007d8c1",
                    "1007d2bc",
                    FUN_1007d2bc(param_1, param_2, local_28 as i32)
                );
                if r != 0 {
                    mech_trace::gw!(
                        "1007d8c1",
                        "100c9ec4",
                        DAT_100c9ec4,
                        mech_trace::rd!("1007d8c1", local_28.add(5), read_unaligned)
                    );
                    mech_trace::gw!(
                        "1007d8c1",
                        "100c9ec8",
                        DAT_100c9ec8,
                        mech_trace::rd!("1007d8c1", local_28.add(6), read_unaligned)
                    );
                    mech_trace::gw!(
                        "1007d8c1",
                        "100c9ec0",
                        DAT_100c9ec0,
                        mech_trace::rd!("1007d8c1", local_28.add(7), read_unaligned)
                    );
                    let i = mech_trace::cr!("1007d8c1", "1008072a", FUN_1008072a(param_1, param_2));
                    if 0 < i {
                        local_8 = local_8.wrapping_add(i);
                        if 0x7f < local_8 {
                            b_var1 = true;
                        }
                    }
                }
                local_28 =
                    mech_trace::rd!("1007d8c1", local_28, read_unaligned) as usize as *mut u32;
            }
        }
        mech_trace::gw!("1007d8c1", "100c9ec4", DAT_100c9ec4, u_var4);
        mech_trace::gw!("1007d8c1", "100c9ec8", DAT_100c9ec8, u_var5);
        mech_trace::gw!("1007d8c1", "100c9ec0", DAT_100c9ec0, u_var3);
        mech_trace::gw!("1007d8c1", "100c96b0", DAT_100c96b0, u_var2);
        local_8 = (param_3 >> 1).wrapping_mul(local_8) / 0x440;
        if local_8 < 1 {
            local_8 = 0;
        } else if 0xf < local_8 {
            local_8 = 0xf;
        }
        mech_trace::rt!("1007d8c1", local_8)
    }
}
