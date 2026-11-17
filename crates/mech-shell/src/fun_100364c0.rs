// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10036390(this: *mut u8, param_1: *mut i8, param_2: u32) -> i32;
    fn FUN_10036460(this: *mut u8, param_1: u32, param_2: *mut i32, param_3: *mut i8) -> *mut u16;
}

#[unsafe(export_name = "FUN_100364c0")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FindEntryByName(
    this: *mut u8,
    param_1: *mut i8,
    param_2: u32,
    param_3: *mut i32,
) -> *mut u16 {
    mech_trace::enter!("100364c0", this, param_1, param_2, param_3);
    // SAFETY: the callees are the game's own; the original passes the extra
    // stack words (param_3 to the first call, param_1 to the second) that the
    // callees' stack cleanup leaves in place, and returns the second call's
    // eax untouched.
    unsafe {
        let u_var1 = mech_trace::cr!("100364c0", "10036390", FUN_10036390(this, param_1, param_2));
        let found = mech_trace::cr!(
            "100364c0",
            "10036460",
            FUN_10036460(this, u_var1 as u32, param_3, param_1)
        );
        mech_trace::rt!("100364c0", found)
    }
}
