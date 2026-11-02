// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002cd5d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100810c0() -> u32 {
    mech_trace::enter!("100810c0");
    // SAFETY: the callee is the game's own function; it takes a plain integer.
    unsafe {
        if mech_trace::cr!("100810c0", "1002cd5d", FUN_1002cd5d(0x100)) == 0
            && mech_trace::cr!("100810c0", "1002cd5d", FUN_1002cd5d(0x200)) == 0
        {
            return mech_trace::rt!("100810c0", 0);
        }
    }
    mech_trace::rt!("100810c0", 1)
}
