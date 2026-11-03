// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10036390(this: *mut u8, param_1: *mut i8, param_2: u32) -> i32;
    fn FUN_10036460(this: *mut u8, param_1: u32, param_2: *mut i32, param_3: *mut i8) -> *mut u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_100364c0(
    this: *mut u8,
    param_1: *mut i8,
    param_2: u32,
    param_3: *mut i32,
) {
    mech_trace::enter!("100364c0", this, param_1, param_2, param_3);
    // SAFETY: the callees are the game's own; the original passes the extra
    // stack words (param_3 to the first call, param_1 to the second) that the
    // callees' stack cleanup leaves in place.
    unsafe {
        let u_var1 = mech_trace::cr!("100364c0", "10036390", FUN_10036390(this, param_1, param_2));
        mech_trace::cr!(
            "100364c0",
            "10036460",
            FUN_10036460(this, u_var1 as u32, param_3, param_1)
        );
    }
}
