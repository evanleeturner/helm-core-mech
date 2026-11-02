// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36dc: u32;
    static mut DAT_100b36e0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006065d(param_1: i32, param_2: u32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100609fa(param_1: i32, param_2: u32) {
    mech_trace::enter!("100609fa", param_1, param_2);
    // SAFETY: the callee and the globals are the game's own memory.
    unsafe {
        FUN_1006065d(param_1, param_2, 0);
        mech_trace::gw!("100609fa", "100b36dc", DAT_100b36dc, param_1 as u32);
        mech_trace::gw!("100609fa", "100b36e0", DAT_100b36e0, param_1 as u32);
    }
}
