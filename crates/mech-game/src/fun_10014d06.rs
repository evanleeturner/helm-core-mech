// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa540: u32;
    static mut DAT_100aa544: u32;
    static mut DAT_100aa548: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002650e(param_1: *mut i32, param_2: i32);
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014d06(param_1: i32, param_2: *mut i32, param_3: i32) {
    mech_trace::enter!("10014d06", param_1, param_2, param_3);
    // SAFETY: the callees and globals are the game's own; param_1 is the
    // caller's object address and param_2 points at the caller's ints.
    unsafe {
        if 0 < param_3 {
            FUN_1002650e(param_2, param_3);
        }
        let b0 = mech_trace::rd!(
            "10014d06",
            (param_1.wrapping_add(0x34) as *const i32),
            read_unaligned
        );
        let b1 = mech_trace::rd!(
            "10014d06",
            (param_1.wrapping_add(0x38) as *const i32),
            read_unaligned
        );
        let b2 = mech_trace::rd!(
            "10014d06",
            (param_1.wrapping_add(0x3c) as *const i32),
            read_unaligned
        );
        if param_3 == 10 {
            mech_trace::gw!(
                "10014d06",
                "100aa540",
                DAT_100aa540,
                mech_trace::rd!("10014d06", param_2, read_unaligned).wrapping_sub(b0) as u32
            );
            mech_trace::gw!(
                "10014d06",
                "100aa544",
                DAT_100aa544,
                mech_trace::rd!("10014d06", param_2.add(1), read_unaligned).wrapping_sub(b1) as u32
            );
            mech_trace::gw!(
                "10014d06",
                "100aa548",
                DAT_100aa548,
                mech_trace::rd!("10014d06", param_2.add(2), read_unaligned).wrapping_sub(b2) as u32
            );
        } else {
            mech_trace::gw!(
                "10014d06",
                "100aa540",
                DAT_100aa540,
                mech_trace::rd!("10014d06", param_2.add(3), read_unaligned).wrapping_sub(b0) as u32
            );
            mech_trace::gw!(
                "10014d06",
                "100aa544",
                DAT_100aa544,
                mech_trace::rd!("10014d06", param_2.add(4), read_unaligned).wrapping_sub(b1) as u32
            );
            mech_trace::gw!(
                "10014d06",
                "100aa548",
                DAT_100aa548,
                mech_trace::rd!("10014d06", param_2.add(5), read_unaligned).wrapping_sub(b2) as u32
            );
        }
        FUN_1000fef4(
            &raw mut DAT_100aa540,
            &raw mut DAT_100aa544,
            &raw mut DAT_100aa548,
        );
    }
}
