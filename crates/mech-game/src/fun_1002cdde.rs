// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafbc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002cdde(param_1: u32, enable: i32) {
    mech_trace::enter!("1002cdde", param_1, enable);
    let _ = param_1;
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("1002cdde", "100aafbc", DAT_100aafbc, (enable == 0) as u32);
    }
}
