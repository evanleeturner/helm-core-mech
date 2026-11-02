// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36cc: i32;
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001bdc6(param_1: *mut u16);
    fn FUN_10026815(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut i32,
        param_6: *mut i32,
    ) -> i32;
    fn FUN_1002634e(param_1: i32) -> i32;
    fn FUN_1002650e(param_1: *mut i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005fc8a(param_1: *mut u16, param_2: *mut i32) -> u32 {
    mech_trace::enter!("1005fc8a", param_1, param_2);
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32;
    let mut local_8: i32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 and param_2
    // point at the caller's structures, read at the original's offsets.
    unsafe {
        FUN_1001bdc6(param_1);
        let local_14: *mut i32 = mech_trace::rd!(
            "1005fc8a",
            param_1.add(0x26).cast::<*mut i32>(),
            read_unaligned
        );
        if local_14.is_null() {
            return mech_trace::rt!("1005fc8a", 0);
        }
        let rd = |p: *mut i32, i: usize| mech_trace::rd!("1005fc8a", p.add(i), read_unaligned);
        let iv = mech_trace::cr!(
            "1005fc8a",
            "10026815",
            FUN_10026815(
                rd(param_2, 0),
                rd(param_2, 9),
                rd(local_14, 0),
                rd(local_14, 1),
                &raw mut local_8,
                &raw mut local_18,
            )
        );
        if iv != 0 {
            return mech_trace::rt!("1005fc8a", 0);
        }
        local_c = 0;
        let iv = mech_trace::cr!(
            "1005fc8a",
            "10026815",
            FUN_10026815(
                rd(param_2, 1),
                rd(param_2, 10),
                rd(local_14, 2),
                rd(local_14, 3),
                &raw mut local_10,
                &raw mut local_1c,
            )
        );
        if iv != 0 {
            return mech_trace::rt!("1005fc8a", 0);
        }
        if local_8 < local_10 {
            local_c = 1;
            local_8 = local_10;
        }
        if local_1c < local_18 {
            local_18 = local_1c;
        }
        let iv = mech_trace::cr!(
            "1005fc8a",
            "10026815",
            FUN_10026815(
                rd(param_2, 2),
                rd(param_2, 0xb),
                rd(local_14, 4),
                rd(local_14, 5),
                &raw mut local_10,
                &raw mut local_1c,
            )
        );
        if iv != 0 {
            return mech_trace::rt!("1005fc8a", 0);
        }
        if local_8 < local_10 {
            local_8 = local_10;
            local_c = 2;
        }
        if local_1c < local_18 {
            local_18 = local_1c;
        }
        if local_18 < local_8 {
            return mech_trace::rt!("1005fc8a", 0);
        }
        if local_8 < 0 {
            mech_trace::gw!("1005fc8a", "100b36cc", DAT_100b36cc, local_8);
            local_8 = 0;
        }
        local_10 = mech_trace::cr!(
            "1005fc8a",
            "1002634e",
            FUN_1002634e(param_2 as usize as i32)
        );
        if local_10 < local_18 {
            local_18 = local_10;
        }
        if local_18 < local_8 {
            return mech_trace::rt!("1005fc8a", 0);
        }
        FUN_1002650e(param_2, local_8);
        if local_c == 0 {
            mech_trace::gw!("1005fc8a", "100aa548", DAT_100aa548, 0);
            mech_trace::gw!("1005fc8a", "100aa544", DAT_100aa544, 0);
            if rd(param_2, 6) < 1 {
                mech_trace::gw!("1005fc8a", "100aa540", DAT_100aa540, 0x10000);
            } else {
                mech_trace::gw!("1005fc8a", "100aa540", DAT_100aa540, 0xffff0000u32 as i32);
            }
        } else if local_c == 1 {
            mech_trace::gw!("1005fc8a", "100aa548", DAT_100aa548, 0);
            mech_trace::gw!("1005fc8a", "100aa540", DAT_100aa540, 0);
            if rd(param_2, 7) < 1 {
                mech_trace::gw!("1005fc8a", "100aa544", DAT_100aa544, 0x10000);
            } else {
                mech_trace::gw!("1005fc8a", "100aa544", DAT_100aa544, 0xffff0000u32 as i32);
            }
        } else if local_c == 2 {
            mech_trace::gw!("1005fc8a", "100aa544", DAT_100aa544, 0);
            mech_trace::gw!("1005fc8a", "100aa540", DAT_100aa540, 0);
            if rd(param_2, 8) < 1 {
                mech_trace::gw!("1005fc8a", "100aa548", DAT_100aa548, 0x10000);
            } else {
                mech_trace::gw!("1005fc8a", "100aa548", DAT_100aa548, 0xffff0000u32 as i32);
            }
        }
        mech_trace::rt!("1005fc8a", 1)
    }
}
