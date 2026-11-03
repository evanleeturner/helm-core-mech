// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100425b0(param_1: i32, param_2: u32, param_3: *mut i32, param_4: u32) -> *mut u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10036460(
    this: *mut u8,
    param_1: u32,
    param_2: u32,
    _param_3: u32,
) -> *mut u16 {
    mech_trace::enter!("10036460", this, param_1, param_2, _param_3);
    if param_1 == 0xffff_ffff {
        return mech_trace::rt!("10036460", core::ptr::null_mut());
    }
    if param_1 == 0xffff_fffe {
        return mech_trace::rt!("10036460", core::ptr::null_mut());
    }
    // SAFETY: `this` points at the caller's object whose first field is read;
    // the callee is the game's own function.
    unsafe {
        let first = mech_trace::rd!("10036460", this.cast::<i32>(), read_unaligned);
        mech_trace::rt!(
            "10036460",
            mech_trace::cr!(
                "10036460",
                "100425b0",
                FUN_100425b0(first, param_1, param_2 as *mut i32, 0)
            )
        )
    }
}
