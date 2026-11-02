// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab0c0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c255(param_1: u32) -> u32 {
    mech_trace::enter!("1001c255", param_1);
    // SAFETY: the global is the game's own memory; it is read then overwritten.
    unsafe {
        let u_var1 = mech_trace::gr!("1001c255", "100ab0c0", DAT_100ab0c0);
        mech_trace::gw!("1001c255", "100ab0c0", DAT_100ab0c0, param_1);
        mech_trace::rt!("1001c255", u_var1)
    }
}
