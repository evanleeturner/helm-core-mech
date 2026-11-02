// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b38d8: u32;
    static mut DAT_100b38dc: u32;
    static mut DAT_100b9f60: u32;
    static mut DAT_100b9f68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10082adc(param_1: u32, param_2: i32, param_3: u32) -> u32;
    fn FUN_10082d2f(param_1: i32, param_2: i32, param_3: u32) -> u32;
    fn FUN_1008304a(param_1: i32, param_2: i32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008276d(param_1: i32, mut param_2: i32, param_3: u32) {
    mech_trace::enter!("1008276d", param_1, param_2, param_3);
    // SAFETY: param_1 points at the game's object; the globals and callees
    // are the game's own, accessed in the original's order.
    unsafe {
        let mut b_var2 = false;
        let p1 = param_1 as usize;
        if mech_trace::rd!("1008276d", ((p1 + 4) as *const u32), read_unaligned)
            == mech_trace::gr!("1008276d", "100adf38", DAT_100adf38)
        {
            mech_trace::gw!("1008276d", "100b9f68", DAT_100b9f68, 0);
        }
        let u_var1 = mech_trace::rd!("1008276d", ((p1 + 0xdc) as *const u32), read_unaligned);
        let mut local_20: u32 = u_var1 & 0xf00;
        let mut local_14: u32;
        if param_2 == 0 || (u_var1 & 0x1000) != 0 {
            local_20 = 0x200;
            local_14 = 0;
            param_2 = 1;
        } else {
            local_14 = (u_var1 & 0xff).wrapping_add(param_2 as u32);
        }
        let i_var3: i32 = (mech_trace::gr!("1008276d", "100b9f60", DAT_100b9f60) as i32)
            .wrapping_add(mech_trace::gr!("1008276d", "100b38dc", DAT_100b38dc) as i32)
            .wrapping_add(mech_trace::gr!("1008276d", "100b38d8", DAT_100b38d8) as i32);
        let mut local_1c: i32 = 0;
        while local_1c < i_var3.wrapping_add(3) && !b_var2 {
            local_1c = local_1c.wrapping_add(1);
            let f4 = mech_trace::rd!("1008276d", ((p1 + 4) as *const u32), read_unaligned);
            if local_20 == 0x100 {
                let r = mech_trace::cr!(
                    "1008276d",
                    "10082adc",
                    FUN_10082adc(f4, local_14 as i32, param_3)
                ) as i32;
                if r == -2 {
                    local_14 = 0;
                    local_20 = 0x200;
                } else if r == -1 {
                    local_14 =
                        mech_trace::gr!("1008276d", "100b38dc", DAT_100b38dc).wrapping_sub(1);
                    local_20 = 0x400;
                } else if r == 1 {
                    b_var2 = true;
                } else {
                    local_14 = local_14.wrapping_add(param_2 as u32);
                }
            } else if local_20 == 0x200 {
                let r = mech_trace::cr!(
                    "1008276d",
                    "10082d2f",
                    FUN_10082d2f(f4 as i32, local_14 as i32, param_3)
                ) as i32;
                if r == -2 {
                    local_14 = 0;
                    local_20 = 0x400;
                } else if r == -1 {
                    local_14 =
                        mech_trace::gr!("1008276d", "100b9f60", DAT_100b9f60).wrapping_sub(1);
                    local_20 = 0x100;
                } else if r == 1 {
                    b_var2 = true;
                } else {
                    local_14 = local_14.wrapping_add(param_2 as u32);
                }
            } else if local_20 == 0x400 {
                let r = mech_trace::cr!(
                    "1008276d",
                    "1008304a",
                    FUN_1008304a(f4 as i32, local_14 as i32, param_3)
                ) as i32;
                if r == -2 {
                    local_14 = 0;
                    local_20 = 0x100;
                } else if r == -1 {
                    local_14 =
                        mech_trace::gr!("1008276d", "100b38d8", DAT_100b38d8).wrapping_sub(1);
                    local_20 = 0x200;
                } else if r == 1 {
                    b_var2 = true;
                } else {
                    local_14 = local_14.wrapping_add(param_2 as u32);
                }
            } else {
                local_14 = 0;
                local_20 = 0x200;
            }
        }
        if b_var2 {
            mech_trace::wr!(
                "1008276d",
                ((p1 + 0xdc) as *mut u32),
                write_unaligned,
                local_14 | local_20
            );
            if local_20 == 0x100 {
                mech_trace::wr!(
                    "1008276d",
                    ((p1 + 0x104) as *mut u32),
                    write_unaligned,
                    local_14 | 0x100
                );
            }
        } else {
            let v = mech_trace::rd!("1008276d", ((p1 + 0xdc) as *const u32), read_unaligned);
            mech_trace::wr!(
                "1008276d",
                ((p1 + 0xdc) as *mut u32),
                write_unaligned,
                v | 0x1000
            );
        }
        let b105 = mech_trace::rd!("1008276d", ((p1 + 0x105) as *const u8), read);
        let p20 = mech_trace::rd!("1008276d", ((p1 + 0x20) as *const u32), read_unaligned) as usize;
        if (b105 & 1) == 0
            && mech_trace::rd!("1008276d", ((p20 + 0x3f0) as *const i32), read_unaligned) != 0
        {
            let p4c =
                mech_trace::rd!("1008276d", ((p1 + 0x4c) as *const u32), read_unaligned) as usize;
            mech_trace::wr!("1008276d", ((p4c + 0x43) as *mut u8), write, 1);
        }
    }
}
