// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046f04(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004633a() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004652f() {
    mech_trace::enter!("1004652f");
    // SAFETY: the global and the callees are the game's own; they are
    // called and written in the original's order.
    unsafe {
        if mech_trace::gr!("1004652f", "100b04c4", DAT_100b04c4) != 0 {
            mech_trace::cr!("1004652f", "10046f04", FUN_10046f04(0x100b04b8));
            mech_trace::cr!("1004652f", "1004633a", FUN_1004633a());
            mech_trace::gw!("1004652f", "100b04c4", DAT_100b04c4, 0);
        }
    }
}
