// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae750: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002cd5d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1008113f() -> u32 {
    mech_trace::enter!("1008113f");
    // SAFETY: the callee and the global are the game's own.
    unsafe {
        if mech_trace::cr!("1008113f", "1002cd5d", FUN_1002cd5d(0x800)) == 0
            && mech_trace::gr!("1008113f", "100ae750", DAT_100ae750) == 0
        {
            return mech_trace::rt!("1008113f", 0);
        }
    }
    mech_trace::rt!("1008113f", 1)
}
