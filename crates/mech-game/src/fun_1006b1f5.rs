// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1006065d(param_1: i32, param_2: u32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b1f5(param_1: u32) {
    mech_trace::enter!("1006b1f5", param_1);
    // SAFETY: the callees are the game's own functions, called in the original's order.
    unsafe {
        let p = if param_1 < 0x10 {
            mech_trace::cr!("1006b1f5", "10002570", FUN_10002570(param_1, 0xf))
        } else {
            0x10000
        };
        let u_var1 = mech_trace::cr!("1006b1f5", "10003d60", FUN_10003d60(0x16a, p as i32));
        mech_trace::cr!("1006b1f5", "1006065d", FUN_1006065d(0x11, u_var1 as u32, 1));
    }
}
