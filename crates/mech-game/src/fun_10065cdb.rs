// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3910: u32;
    static mut DAT_100b3920: u32;
    static mut DAT_100b3924: u32;
    static mut DAT_100b3928: u32;
    static mut DAT_100b3938: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036aeb(param_1: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn AIL_release_timer_handle(param_1: u32) -> u32;
    fn AIL_shutdown() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065cdb() {
    mech_trace::enter!("10065cdb");
    // SAFETY: the globals and callees are the game's own; calls and
    // writes happen in the original's order.
    unsafe {
        if mech_trace::gr!("10065cdb", "100b3938", DAT_100b3938) != 0 {
            mech_trace::cr!(
                "10065cdb",
                "10036aeb",
                FUN_10036aeb(mech_trace::gr!("10065cdb", "100b3924", DAT_100b3924))
            );
            mech_trace::cr!(
                "10065cdb",
                "10036aeb",
                FUN_10036aeb(mech_trace::gr!("10065cdb", "100b3928", DAT_100b3928))
            );
            mech_trace::cr!(
                "10065cdb",
                "10036aeb",
                FUN_10036aeb(mech_trace::gr!("10065cdb", "100b3920", DAT_100b3920))
            );
            mech_trace::cr!(
                "10065cdb",
                "AIL_release_timer_handle",
                AIL_release_timer_handle(mech_trace::gr!("10065cdb", "100b3910", DAT_100b3910))
            );
            mech_trace::gw!("10065cdb", "100b3910", DAT_100b3910, 0xffffffff);
            mech_trace::cr!("10065cdb", "AIL_shutdown", AIL_shutdown());
            mech_trace::gw!("10065cdb", "100b3938", DAT_100b3938, 0);
        }
    }
}
