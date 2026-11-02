// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adb00: u32;
    static mut DAT_100adb04: u32;
    static mut DAT_100adb08: u32;
    static mut DAT_100adb0c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_release_timer_handle_4(param_1: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006af90(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100338f2() {
    mech_trace::enter!("100338f2");
    // SAFETY: the globals and callees are the game's own; the handles are
    // released and cleared exactly where the original does.
    unsafe {
        if mech_trace::gr!("100338f2", "100adb00", DAT_100adb00) != 0xFFFF_FFFF {
            mech_trace::cr!(
                "100338f2",
                "_AIL_release_timer_handle_4",
                _AIL_release_timer_handle_4(mech_trace::gr!("100338f2", "100adb00", DAT_100adb00))
            );
            mech_trace::gw!("100338f2", "100adb00", DAT_100adb00, 0xFFFF_FFFF);
        }
        if mech_trace::gr!("100338f2", "100adb04", DAT_100adb04) != 0 {
            FUN_1006af90(mech_trace::gr!("100338f2", "100adb04", DAT_100adb04));
            mech_trace::gw!("100338f2", "100adb04", DAT_100adb04, 0);
        }
        if mech_trace::gr!("100338f2", "100adb08", DAT_100adb08) != 0 {
            FUN_1006af90(mech_trace::gr!("100338f2", "100adb08", DAT_100adb08));
            mech_trace::gw!("100338f2", "100adb08", DAT_100adb08, 0);
            mech_trace::gw!("100338f2", "100adb0c", DAT_100adb0c, 0);
        }
    }
}
