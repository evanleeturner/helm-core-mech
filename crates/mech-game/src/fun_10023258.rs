// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba0: u8;
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7be4: u8;
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004c610(param_1: i32, param_2: i32) -> u32;
    fn FUN_1006312b(param_1: i32, param_2: i32) -> u32;
    fn FUN_10062d5f(param_1: i32, param_2: i32) -> u8;
}

unsafe fn ri(a: i32) -> i32 {
    // SAFETY: the caller passes an address of the game's own memory.
    unsafe { mech_trace::rd!("10023258", (a as *const i32), read_unaligned) }
}

unsafe fn wi(a: i32, v: i32) {
    // SAFETY: the caller passes an address of the game's own memory.
    unsafe { mech_trace::wr!("10023258", (a as *mut i32), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10023258(param_1: i32, param_2: *mut u32) -> i32 {
    mech_trace::enter!("10023258", param_1, param_2);
    // SAFETY: all pointers and globals belong to the game; reads and writes
    // happen in the original's order.
    unsafe {
        let i_var2 = ri(param_1.wrapping_add(0x20));
        let mut local_8: i16 = 0;
        let mut local_24: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i16 = 0;
        loop {
            let mut finish = false;
            if ri(i_var2.wrapping_add(0x3dc)) <= local_14 as i32 {
                finish = true;
            } else {
                let i_var4 = (local_14 as i32)
                    .wrapping_mul(0x40)
                    .wrapping_add(ri(i_var2.wrapping_add(0x54)));
                let s_var3 = ri(i_var4.wrapping_add(4)) as i16;
                let i_var1 = (s_var3 as i32).wrapping_mul(0x68);
                wi(i_var4.wrapping_add(0xc), 0);
                let base_a = (&raw mut DAT_100a7ba0) as usize as i32;
                let base_b = (&raw mut DAT_100a7bc8) as usize as i32;
                let base_c = (&raw mut DAT_100a7be4) as usize as i32;
                let mut cond = false;
                if ri(param_1.wrapping_add(0xdc)) != -1
                    && (mech_trace::rd!(
                        "10023258",
                        (param_1.wrapping_add(0xdd) as *const u8),
                        read
                    ) & 0x10)
                        == 0
                    && mech_trace::cr!(
                        "10023258",
                        "1004c610",
                        FUN_1004c610(param_1, base_a.wrapping_add(i_var1))
                    ) == 2
                {
                    let f20 = ri(i_var4.wrapping_add(0x20));
                    if f20 == 9999
                        || (f20 < ri(i_var2.wrapping_add(0x60))
                            && ri(i_var2
                                .wrapping_add(0x66)
                                .wrapping_add(f20.wrapping_mul(0x3a)))
                                != 0)
                    {
                        if ri(i_var4.wrapping_add(8)) != -1 {
                            cond = true;
                        }
                    }
                }
                if cond {
                    let v_c = ri(base_c.wrapping_add(i_var1));
                    if (ri(param_1.wrapping_add(4)) as u32)
                        != mech_trace::gr!("10023258", "100adf38", DAT_100adf38)
                        && ri(param_1.wrapping_add(0x1b0))
                            <= ((v_c as u32)
                                .wrapping_add(ri(i_var2.wrapping_add(0x3cc)) as u32)
                                .wrapping_add(local_1c as u32)
                                as i32)
                                >> 0x10
                    {
                        local_24 = 0;
                        finish = true;
                    } else {
                        local_24 = 1;
                        wi(i_var4.wrapping_add(0xc), 1);
                        local_1c = local_1c.wrapping_add(ri(base_c.wrapping_add(i_var1)));
                        if local_8 == 0
                            && ri(base_b.wrapping_add(i_var1)) != 0
                            && ri(i_var4.wrapping_add(8)) == 1
                        {
                            local_8 = local_14.wrapping_add(1);
                        }
                        if s_var3 == 0 {
                            local_18 = 0x1e000;
                        }
                        if s_var3 == 4 {
                            local_18 = 0x3c000;
                        }
                    }
                }
                if !finish {
                    local_14 = local_14.wrapping_add(1);
                }
            }
            if finish {
                if local_24 == 0 {
                    mech_trace::cr!("10023258", "10062d5f", FUN_10062d5f(i_var2, 1));
                } else {
                    let p = ri(param_1.wrapping_add(0x4c));
                    wi(p, ri(p).wrapping_add(local_18));
                    if local_8 == 0 {
                        mech_trace::cr!("10023258", "1006312b", FUN_1006312b(i_var2, 1));
                    } else {
                        wi(
                            i_var2.wrapping_add(0x3e0),
                            (local_8 as i32).wrapping_add(-1),
                        );
                    }
                }
                if local_24 != 0 && local_8 != 0 {
                    mech_trace::wr!("10023258", param_2, write_unaligned, 1);
                }
                return mech_trace::rt!("10023258", local_24);
            }
        }
    }
}
