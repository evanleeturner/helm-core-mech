// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083808() -> i32;
    fn FUN_1002f28d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100837cb() -> u32 {
    mech_trace::enter!("100837cb");
    // SAFETY: the callees are the game's own functions; the value from the
    // first is passed to the second as the original does.
    unsafe {
        let i_var1 = mech_trace::cr!("100837cb", "10083808", FUN_10083808());
        mech_trace::rt!(
            "100837cb",
            if i_var1 == 0 {
                0
            } else {
                mech_trace::cr!("100837cb", "1002f28d", FUN_1002f28d(i_var1))
            }
        )
    }
}
