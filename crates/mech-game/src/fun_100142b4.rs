// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
    static mut DAT_100aa54c: i32;
    static mut DAT_100aa550: i32;
    static mut DAT_100aa554: i32;
    static mut DAT_10160c00: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003d2d3(param_1: i32, param_3: i32) -> i32;
    fn FUN_10060161(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut i32,
    ) -> i32;
    fn FUN_1005fb3a(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100142b4(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut local_18: i32 = 0;
    let mut local_c: i32 = -0x7fffffff;
    let mut local_10: i32 = -0x7fffffff;
    let mut local_14: i32 = 0;
    // SAFETY: callees and globals are the game's own; list nodes are game
    // memory read at the same offsets as the original.
    unsafe {
        FUN_1003d2d3(param_1, param_3);
        let mut local_8: i32 = DAT_10160c00;
        loop {
            if local_8 == 0 {
                if local_c < -0x7ffffffe {
                    local_c = local_10;
                    if local_10 < -0x7ffffffe {
                        local_c = 0;
                    }
                }
                return local_c;
            }
            local_18 = local_18.wrapping_add(1);
            if local_8 == 0 {
                return 0;
            }
            let mut local_1c: i32 = ((local_8.wrapping_add(0x10)) as *const i32).read_unaligned();
            while local_1c != 0 {
                let kind = ((local_1c.wrapping_add(0x24)) as *const i32).read_unaligned();
                let local_20: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32) -> i32;
                if kind == 5 {
                    local_20 = FUN_10060161;
                } else if kind == 0 {
                    local_20 = FUN_1005fb3a;
                } else {
                    local_1c = ((local_1c.wrapping_add(0x10)) as *const i32).read_unaligned();
                    continue;
                }
                let i_var1 = local_20(local_1c, param_1, param_2, param_3, &raw mut local_14);
                if i_var1 != 0 {
                    if local_c == -0x7fffffff && local_14 < param_2 && local_10 < local_14 {
                        local_10 = local_14;
                        DAT_100aa54c = DAT_100aa540;
                        DAT_100aa550 = DAT_100aa544;
                        DAT_100aa554 = DAT_100aa548;
                    }
                    if param_2 < local_14.wrapping_add(1000)
                        && local_14.wrapping_add(-1000) < param_2
                        && local_c < local_14
                    {
                        local_c = local_14;
                        DAT_100aa54c = DAT_100aa540;
                        DAT_100aa550 = DAT_100aa544;
                        DAT_100aa554 = DAT_100aa548;
                    }
                }
                local_1c = ((local_1c.wrapping_add(0x10)) as *const i32).read_unaligned();
            }
            local_8 = (&raw mut DAT_10160c00)
                .wrapping_offset(local_18.wrapping_mul(4) as isize)
                .read_unaligned();
        }
    }
}
