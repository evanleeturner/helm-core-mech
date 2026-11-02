// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003c111() -> u8;
    fn FUN_1002cdb5() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100811b9() -> u32 {
    mech_trace::enter!("100811b9");
    // SAFETY: the callees are the game's own functions and take no arguments.
    unsafe {
        if mech_trace::cr!("100811b9", "1003c111", FUN_1003c111()) == 0
            && mech_trace::cr!("100811b9", "1002cdb5", FUN_1002cdb5()) == 0
        {
            return mech_trace::rt!("100811b9", 0);
        }
    }
    mech_trace::rt!("100811b9", 1)
}
