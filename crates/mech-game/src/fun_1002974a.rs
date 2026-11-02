// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002b08b(unused_arg1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002974a(_unused_arg1: u32, param_2: i32, param_3: i32) {
    mech_trace::enter!("1002974a", _unused_arg1, param_2, param_3);
    // SAFETY: the callee is the game's own function; the divisor is a nonzero
    // constant other than -1, so the division cannot fault.
    unsafe {
        FUN_1002b08b(param_2 / 0x2d00, (param_3 as u32).wrapping_mul(2) as i32, 1);
    }
}
