// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3938: i32;
    static mut DAT_100b3910: i32;
    static mut DAT_100b3924: i32;
    static mut DAT_100b3920: i32;
    static mut DAT_100b3928: i32;
    static mut DAT_100b3930: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b3900: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn AIL_startup();
    fn AIL_register_timer(callback: unsafe extern "stdcall" fn(u32)) -> i32;
    fn AIL_set_timer_divisor(timer: i32, divisor: u32);
    fn AIL_start_timer(timer: i32);
    fn LAB_100369a0(user: u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100369c9(param_1: i32) -> i16;
    fn FUN_10036a72(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065a50() {
    // SAFETY: the callees and globals are the game's own memory; they are
    // read, written and called in the original's order.
    unsafe {
        if DAT_100b3938 == 0 {
            AIL_startup();
            let timer = AIL_register_timer(LAB_100369a0);
            DAT_100b3910 = timer;
            AIL_set_timer_divisor(timer, 0x199c);
            AIL_start_timer(DAT_100b3910);
            DAT_100b3924 = FUN_100369c9(0x100) as i32;
            DAT_100b3920 = FUN_100369c9(0x80) as i32;
            DAT_100b3928 = FUN_100369c9(0x80) as i32;
            FUN_10036a72(DAT_100b3924);
            FUN_10036a72(DAT_100b3928);
            FUN_10036a72(DAT_100b3920);
            DAT_100b3938 = 1;
        }
        DAT_100b3930 = 0;
        DAT_100b38fc = 0;
        DAT_100b3900 = 0;
    }
}
