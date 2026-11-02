// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092014: *mut u32;
    static mut DAT_10092018: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063f40(param_1: *mut i32) -> *mut i32;
    fn FUN_10063ea0(param_1: *mut i32);
    fn FUN_1006e790(param_1: *mut c_void);
    fn FUN_10033f40(param_1: i32) -> *mut u32;
    fn FUN_10063d30(param_1: *mut i32);
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10033e80(param_1: i32) {
    mech_trace::enter!("10033e80", param_1);
    if param_1 == 0 {
        return;
    }
    // SAFETY: the globals and callees are the game's own; the pointers walk
    // the game's record array exactly as the original does.
    unsafe {
        let mut local_4: i32 = 0;
        if !mech_trace::gr!("10033e80", "10092014", DAT_10092014).is_null() {
            let p = mech_trace::cr!(
                "10033e80",
                "10063f40",
                FUN_10063f40(mech_trace::gr!("10033e80", "10092014", DAT_10092014) as *mut i32)
            );
            if !p.is_null() {
                local_4 = mech_trace::rd!("10033e80", p.add(9), read_unaligned);
            }
        }
        FUN_10063ea0(mech_trace::gr!("10033e80", "10092014", DAT_10092014) as *mut i32);
        if !mech_trace::gr!("10033e80", "10092014", DAT_10092014).is_null() {
            FUN_1006e790(mech_trace::gr!("10033e80", "10092014", DAT_10092014) as *mut c_void);
        }
        mech_trace::gw!("10033e80", "10092018", DAT_10092018, param_1 as u32);
        mech_trace::gw!(
            "10033e80",
            "10092014",
            DAT_10092014,
            mech_trace::cr!("10033e80", "10033f40", FUN_10033f40(param_1))
        );
        FUN_10063d30(mech_trace::gr!("10033e80", "10092014", DAT_10092014) as *mut i32);
        let mut pi2 = mech_trace::gr!("10033e80", "10092014", DAT_10092014) as *mut i32;
        if mech_trace::rd!(
            "10033e80",
            mech_trace::gr!("10033e80", "10092014", DAT_10092014),
            read_unaligned
        ) != 0xFFFF_FFFF
        {
            while mech_trace::rd!("10033e80", pi2.add(9), read_unaligned) != local_4 {
                let pi1 = pi2.add(0xe);
                pi2 = pi2.add(0xe);
                if mech_trace::rd!("10033e80", pi1, read_unaligned) == -1 {
                    return;
                }
            }
            mech_trace::wr!("10033e80", pi2.add(0xb), write_unaligned, 1);
            mech_trace::wr!("10033e80", pi2.add(0x19), write_unaligned, 1);
            mech_trace::wr!("10033e80", pi2.add(0x27), write_unaligned, 1);
            FUN_10063de0(mech_trace::gr!("10033e80", "10092014", DAT_10092014) as *mut i32);
        }
    }
}
