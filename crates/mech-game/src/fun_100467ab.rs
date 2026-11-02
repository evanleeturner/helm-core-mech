// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100463c4();
    fn FUN_10046563() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100467ab() {
    mech_trace::enter!("100467ab");
    // SAFETY: the global and the callees are the game's own.
    unsafe {
        if mech_trace::gr!("100467ab", "100b04c4", DAT_100b04c4) != 0 {
            loop {
                FUN_100463c4();
                let i_var1 = mech_trace::cr!("100467ab", "10046563", FUN_10046563()) as i32;
                if i_var1 != 3 {
                    break;
                }
            }
        }
    }
}
