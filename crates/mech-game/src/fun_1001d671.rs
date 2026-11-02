// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d5cb(param_1: i32) -> u32;
    fn FUN_1002f272(param_1: i32, param_2: u32) -> u8;
    fn FUN_1001d644(param_1: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d671(param_1: *mut u8) {
    mech_trace::enter!("1001d671", param_1);
    // SAFETY: the callees are the game's own functions; param_1 is passed
    // through unchanged as the original does.
    unsafe {
        let i_var1 = mech_trace::cr!("1001d671", "1001d5cb", FUN_1001d5cb(param_1 as i32)) as i32;
        if i_var1 != 0 {
            mech_trace::cr!("1001d671", "1002f272", FUN_1002f272(i_var1, 0));
        }
        mech_trace::cr!("1001d671", "1001d644", FUN_1001d644(param_1));
    }
}
