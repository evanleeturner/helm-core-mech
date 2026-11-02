// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa588: i32;
    static mut DAT_100aa58c: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100eef40: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067d51(param_1: *mut i32) -> i32;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10016cdd(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016651(param_1: i32) {
    mech_trace::enter!("10016651", param_1);
    // SAFETY: param_1 is the caller's object pointer as an integer; the globals
    // and callees are the game's own, and all reads follow the original.
    unsafe {
        let rd = |off: i32| -> i32 {
            mech_trace::rd!(
                "10016651",
                ((param_1 as u32).wrapping_add(off as u32) as *const i32),
                read_unaligned
            )
        };
        let pi_var1 = rd(0xa4) as *mut i32;
        let mut local_10: i32 = 0;
        let local_14: i32;
        let local_c: i32;
        let mut local_8: i32;

        let entry = mech_trace::rd!(
            "10016651",
            (&raw mut DAT_100eef40)
                .cast::<i32>()
                .offset(mech_trace::gr!("10016651", "100adf38", DAT_100adf38) as isize),
            read_unaligned
        );
        let inner = mech_trace::rd!(
            "10016651",
            ((entry as u32).wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        let a = mech_trace::rd!(
            "10016651",
            ((inner as u32).wrapping_add(0x3c4) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "10016651",
            ((inner as u32).wrapping_add(0x3d0) as *const i32),
            read_unaligned
        );
        let v = (a as u32).wrapping_sub((b as u32).wrapping_mul(mech_trace::gr!(
            "10016651",
            "100b3904",
            DAT_100b3904
        ) as u32)) as i32;
        mech_trace::gw!("10016651", "100aa58c", DAT_100aa58c, v >> 6);

        local_8 = mech_trace::cr!("10016651", "10067d51", FUN_10067d51(&raw mut DAT_100aa588));
        if local_8 < 1 {
            local_14 = 7;
            local_c = 7;
            local_8 = 0;
        } else if local_8 < 0x300 {
            local_14 = 3;
            local_c = 7;
        } else {
            local_14 = 0xb;
            local_c = 3;
            local_8 = (local_8 as u32).wrapping_sub(0x300) as i32;
        }
        if 0 < local_8 {
            local_10 = mech_trace::cr!(
                "10016651",
                "10011e60",
                FUN_10011e60(rd(0x9c), local_8, 0x300)
            );
            if local_10 < 0 {
                local_10 = 0;
            } else if rd(0x9c) < local_10 {
                local_10 = rd(0x9c);
            }
        }
        if 0 < local_10 {
            FUN_10016cdd(
                pi_var1,
                rd(0x94),
                rd(0x98),
                local_10,
                (rd(0xa0) as u32).wrapping_sub(1) as i32,
                local_14,
            );
        }
        if local_10 < rd(0x9c) {
            FUN_10016cdd(
                pi_var1,
                (rd(0x94) as u32).wrapping_add(local_10 as u32) as i32,
                rd(0x98),
                (rd(0x9c) as u32).wrapping_sub(local_10 as u32) as i32,
                (rd(0xa0) as u32).wrapping_sub(1) as i32,
                local_c,
            );
        }
    }
}
