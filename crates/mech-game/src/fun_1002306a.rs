// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004c610(param_1: i32, param_2: i32) -> u32;
    fn FUN_10063287(param_1: i32) -> u32;
    fn FUN_10023898(param_1: i32, param_2: i32) -> u32;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10062d5f(param_1: i32, param_2: i32) -> u8;
}

#[inline(always)]
unsafe fn rd(a: i32) -> i32 {
    // SAFETY: caller guarantees the address is valid game memory.
    unsafe { mech_trace::rd!("1002306a", (a as usize as *const i32), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002306a(param_1: i32, param_2: *mut u32) -> i32 {
    mech_trace::enter!("1002306a", param_1, param_2);
    // SAFETY: all addresses are the game's own memory, accessed where the
    // original does; param_2 is the caller's out pointer.
    unsafe {
        let i_var1 = rd(param_1.wrapping_add(0x20));
        mech_trace::dw!("1002306a", param_2, 0);
        let mut local_14: i32 = 0;
        let mut local_8: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_c: i32 = 0;
        let mut local_1c: i32 = 0;
        while local_14 < rd(i_var1.wrapping_add(0x3dc)) && local_8 == 0 {
            local_8 = 0;
            if rd(i_var1.wrapping_add(0x3e0)) != -1 {
                local_18 = rd(i_var1.wrapping_add(0x3e0))
                    .wrapping_mul(0x40)
                    .wrapping_add(rd(i_var1.wrapping_add(0x54)));
                local_c = rd(local_18.wrapping_add(4));
                local_1c = ((&raw mut DAT_100a7ba0) as usize as i32)
                    .wrapping_add(local_c.wrapping_mul(0x68));
                let i_var2 =
                    mech_trace::cr!("1002306a", "1004c610", FUN_1004c610(param_1, local_1c)) as i32;
                if 0 < i_var2 && i_var2 < 3 {
                    local_8 = 1;
                }
            }
            if local_8 != 0 {
                local_8 = 0;
                let sum = (rd(i_var1.wrapping_add(0x3cc)) as u32)
                    .wrapping_add(rd(local_1c.wrapping_add(0x44)) as u32)
                    as i32;
                if (sum >> 0x10) < rd(param_1.wrapping_add(0x1b0))
                    && mech_trace::cr!("1002306a", "10063287", FUN_10063287(i_var1)) == 1
                    && mech_trace::cr!("1002306a", "10023898", FUN_10023898(i_var1, local_18)) != 0
                    && mech_trace::cr!(
                        "1002306a",
                        "10051f43",
                        FUN_10051f43((rd(local_1c.wrapping_add(0x54)) / 0x5a).wrapping_add(1))
                    ) == 0
                {
                    local_8 = 1;
                }
            }
            if local_8 == 0 {
                mech_trace::cr!("1002306a", "10062d5f", FUN_10062d5f(i_var1, 1));
            }
            local_14 = local_14.wrapping_add(1);
        }
        if local_8 != 0 {
            let p = rd(param_1.wrapping_add(0x4c)) as usize as *mut i32;
            if local_c == 0 {
                mech_trace::wr!(
                    "1002306a",
                    p,
                    write_unaligned,
                    mech_trace::rd!("1002306a", p, read_unaligned).wrapping_add(0x1e000)
                );
            } else if local_c == 4 {
                mech_trace::wr!(
                    "1002306a",
                    p,
                    write_unaligned,
                    mech_trace::rd!("1002306a", p, read_unaligned).wrapping_add(0x3c000)
                );
            }
            if rd(local_1c.wrapping_add(0x28)) != 0 {
                mech_trace::dw!("1002306a", param_2, 1);
            }
        }
        mech_trace::rt!("1002306a", local_8)
    }
}
