// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada68: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003106f(param_1: i32) -> i32 {
    mech_trace::enter!("1003106f", param_1);
    let mut param_1 = param_1;
    // SAFETY: the global is the game's own memory, read and written where the original does.
    unsafe {
        if param_1 == -1 {
            param_1 = mech_trace::gr!("1003106f", "100ada68", DAT_100ada68);
        }
        mech_trace::gw!("1003106f", "100ada68", DAT_100ada68, param_1);
    }
    mech_trace::rt!("1003106f", param_1)
}
