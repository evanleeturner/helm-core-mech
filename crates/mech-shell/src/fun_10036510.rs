// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10036390(this: *mut u8, param_1: *mut i8, param_2: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100364f0(param_1: i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10036510(
    this: *mut u8,
    param_1: *mut i8,
    param_2: u32,
    param_3: *mut i32,
) {
    mech_trace::enter!("10036510", this, param_1, param_2, param_3);
    // SAFETY: the callees are the game's own functions; arguments are forwarded unchanged.
    unsafe {
        let first_call_result =
            mech_trace::cr!("10036510", "10036390", FUN_10036390(this, param_1, param_2));
        FUN_100364f0(first_call_result, param_3);
    }
}
