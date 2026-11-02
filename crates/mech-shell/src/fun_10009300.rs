// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082d6c: i32;
    static mut DAT_100ae180: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000bfd0() -> u32;
    fn FUN_10008330();
    fn FUN_10008680();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10009300() {
    mech_trace::enter!("10009300");
    // SAFETY: the globals and callees are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        let i_var2 = mech_trace::gr!("10009300", "10082d6c", DAT_10082d6c).wrapping_add(1);
        mech_trace::gw!("10009300", "10082d6c", DAT_10082d6c, i_var2);
        let i_var1 = mech_trace::cr!("10009300", "1000bfd0", FUN_1000bfd0()) as i32;
        if i_var1 <= i_var2 {
            mech_trace::gw!("10009300", "10082d6c", DAT_10082d6c, 0);
        }
        mech_trace::gw!("10009300", "100ae180", DAT_100ae180, 0);
        FUN_10008330();
        FUN_10008680();
    }
}
