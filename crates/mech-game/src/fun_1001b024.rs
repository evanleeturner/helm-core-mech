// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab318: i32;
    static mut DAT_101eb454: i32;
    static mut DAT_100ba9ac: i32;
    static mut DAT_101eb3a4: i32;
    static mut DAT_100ba9a8: i32;
    static mut DAT_100ab090: i32;
    static mut DAT_101eb450: i32;
    static mut DAT_101eb3a0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a8a3(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
    fn FUN_10067af1(param_1: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b024(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
    param_6: *mut i32,
) {
    mech_trace::enter!(
        "1001b024", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the callees and globals are the game's own; the pointer
    // parameters are the caller's and are read and written where the
    // original does.
    unsafe {
        FUN_1001a8a3(param_1, param_2, param_3, param_4, param_5, param_6);
        if mech_trace::gr!("1001b024", "100ab318", DAT_100ab318) == 0 {
            mech_trace::gw!(
                "1001b024",
                "101eb454",
                DAT_101eb454,
                mech_trace::gr!("1001b024", "100ba9ac", DAT_100ba9ac)
            );
            let i_var1 =
                mech_trace::cr!("1001b024", "10067af1", FUN_10067af1(&raw mut DAT_101eb450));
            mech_trace::gw!(
                "1001b024",
                "101eb3a4",
                DAT_101eb3a4,
                mech_trace::gr!("1001b024", "100ba9a8", DAT_100ba9a8)
            );
            let i_var2 =
                mech_trace::cr!("1001b024", "10067af1", FUN_10067af1(&raw mut DAT_101eb3a0));
            mech_trace::wr!(
                "1001b024",
                param_2,
                write_unaligned,
                mech_trace::rd!("1001b024", param_2, read_unaligned).wrapping_add(i_var1)
            );
            mech_trace::wr!(
                "1001b024",
                param_1,
                write_unaligned,
                mech_trace::rd!("1001b024", param_1, read_unaligned).wrapping_add(i_var2)
            );
        }
        mech_trace::gw!("1001b024", "100ab090", DAT_100ab090, 1);
    }
}
