// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7fb8: u32;
    static mut DAT_100c7fbc: u32;
    static mut DAT_100c7fc0: u32;
    static mut DAT_100c7fc4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d32b(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> u8;
    fn FUN_1001d6b2(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069ef1(param_1: i32) {
    mech_trace::enter!("10069ef1", param_1);
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    // SAFETY: the callees and the globals are the game's own; the locals
    // are valid for the callee to write through.
    unsafe {
        mech_trace::cr!(
            "10069ef1",
            "1001d32b",
            FUN_1001d32b(param_1, &raw mut local_8, &raw mut local_c)
        );
        mech_trace::gw!(
            "10069ef1",
            "100c7fc0",
            DAT_100c7fc0,
            mech_trace::gr!("10069ef1", "100c7fc0", DAT_100c7fc0).wrapping_add(1)
        );
        mech_trace::gw!(
            "10069ef1",
            "100c7fb8",
            DAT_100c7fb8,
            mech_trace::gr!("10069ef1", "100c7fb8", DAT_100c7fb8).wrapping_add(local_8 as u32)
        );
        mech_trace::gw!(
            "10069ef1",
            "100c7fc4",
            DAT_100c7fc4,
            mech_trace::gr!("10069ef1", "100c7fc4", DAT_100c7fc4).wrapping_add(local_c as u32)
        );
        let i_var1 = mech_trace::cr!("10069ef1", "1001d6b2", FUN_1001d6b2(param_1));
        mech_trace::gw!(
            "10069ef1",
            "100c7fbc",
            DAT_100c7fbc,
            mech_trace::gr!("10069ef1", "100c7fbc", DAT_100c7fbc).wrapping_add(i_var1 as u32)
        );
    }
}
