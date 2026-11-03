// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005e020(forwarded_string: u32, forwarded_value: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10036390(
    gate_object: *mut core::ffi::c_void,
    forwarded_string: *mut i8,
    forwarded_value: u32,
) -> i32 {
    mech_trace::enter!("10036390", gate_object, forwarded_string, forwarded_value);
    // SAFETY: gate_object points at the game's object whose first field is an i32;
    // the callee is the game's own function.
    unsafe {
        if mech_trace::rd!("10036390", gate_object.cast::<i32>(), read_unaligned) < 0 {
            return mech_trace::rt!("10036390", -1);
        }
        mech_trace::rt!(
            "10036390",
            mech_trace::cr!(
                "10036390",
                "1005e020",
                FUN_1005e020(forwarded_value, forwarded_string)
            )
        )
    }
}
