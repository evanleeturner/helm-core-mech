// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100eee60: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002590(param_1: u32) -> u32;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003c41d() -> u32;
    fn FUN_1004643b() -> u32;
    fn FUN_1001d7a0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ace6() -> u32 {
    mech_trace::enter!("1006ace6");
    // SAFETY: the callees and the globals are the game's own memory; calls
    // and accesses happen in the original's order.
    unsafe {
        mech_trace::cr!("1006ace6", "10002590", FUN_10002590(8));
        mech_trace::cr!("1006ace6", "1003c41d", FUN_1003c41d());
        let i_var1 = mech_trace::cr!("1006ace6", "1004643b", FUN_1004643b()) as i32;
        if i_var1 == 0 {
            FUN_100108a0(0x53, core::ptr::null_mut());
        }
        FUN_1001d7a0();
        mech_trace::gw!(
            "1006ace6",
            "100eee60",
            DAT_100eee60,
            mech_trace::gr!("1006ace6", "100b38fc", DAT_100b38fc).wrapping_add(0x389)
        );
        mech_trace::rt!("1006ace6", 1)
    }
}
