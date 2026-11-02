// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100498c5(param_1: i32) -> i32;
    fn FUN_1004890b(param_1: i32) -> u8;
    fn FUN_100482cb(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004831f(param_1: i32) -> u32 {
    mech_trace::enter!("1004831f", param_1);
    let mut local_8: u32 = 0;
    // SAFETY: the callees are the game's own functions, called in the original's order.
    unsafe {
        let i_var1 = mech_trace::cr!("1004831f", "100498c5", FUN_100498c5(param_1));
        if i_var1 == 0 {
            mech_trace::cr!("1004831f", "100482cb", FUN_100482cb(param_1));
            local_8 = 1;
        } else if i_var1 == 1 {
            mech_trace::cr!("1004831f", "1004890b", FUN_1004890b(param_1));
        }
    }
    mech_trace::rt!("1004831f", local_8)
}
