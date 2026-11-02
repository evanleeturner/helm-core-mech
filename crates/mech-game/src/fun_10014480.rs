// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
    static mut DAT_100af098: i32;
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
    static mut DAT_100aa54c: i32;
    static mut DAT_100aa550: i32;
    static mut DAT_100aa554: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003d6bf(param_1: i32, param_3: i32) -> i32;
    fn FUN_10060161(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: *mut i32,
    ) -> i32;
    fn FUN_1005fb3a(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: *mut i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014480(param_1: i32, param_2: u32, param_3: i32) -> i32 {
    let mut local_10: i32 = 0;
    let local_8: i32;
    let mut local_c: i32;
    // SAFETY: globals and callees are the game's own; the list nodes are
    // game memory read at the same offsets as the original.
    unsafe {
        if DAT_100aefc0 == 0 {
            local_8 = DAT_100af098;
        } else {
            local_8 = FUN_1003d6bf(param_1, param_3);
        }
        if local_8 == 0 {
            local_c = 0;
        } else {
            local_c = -0x7fffffff;
            let mut local_14 = ((local_8 as u32).wrapping_add(0x10) as *const i32).read_unaligned();
            while local_14 != 0 {
                let kind = ((local_14 as u32).wrapping_add(0x24) as *const i32).read_unaligned();
                let f: Option<unsafe extern "C" fn(i32, i32, u32, i32, *mut i32) -> i32> =
                    if kind == 5 {
                        Some(FUN_10060161)
                    } else if kind == 0 {
                        Some(FUN_1005fb3a)
                    } else {
                        None
                    };
                if let Some(func) = f {
                    let iv = func(local_14, param_1, param_2, param_3, &raw mut local_10);
                    if iv != 0 && local_c < local_10 {
                        local_c = local_10;
                        DAT_100aa54c = DAT_100aa540;
                        DAT_100aa550 = DAT_100aa544;
                        DAT_100aa554 = DAT_100aa548;
                    }
                }
                local_14 = ((local_14 as u32).wrapping_add(0x10) as *const i32).read_unaligned();
            }
            if local_c < -0x7ffffffe {
                local_c = 0;
            }
        }
    }
    local_c
}
