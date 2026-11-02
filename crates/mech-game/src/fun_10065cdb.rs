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
    // SAFETY: the globals and callees are the game's own; calls and
    // writes happen in the original's order.
    unsafe {
        if DAT_100b3938 != 0 {
            FUN_10036aeb(DAT_100b3924);
            FUN_10036aeb(DAT_100b3928);
            FUN_10036aeb(DAT_100b3920);
            AIL_release_timer_handle(DAT_100b3910);
            DAT_100b3910 = 0xffffffff;
            AIL_shutdown();
            DAT_100b3938 = 0;
        }
    }
}
