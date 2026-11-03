// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017f30(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1001e620(this: *mut core::ffi::c_void, param_1: i32) {
    mech_trace::enter!("1001e620", this, param_1);
    // SAFETY: `this` points at the caller's object whose first field is an i32;
    // the callees are the game's own functions.
    unsafe {
        let first_call_result = mech_trace::cr!(
            "1001e620",
            "10017f30",
            FUN_10017f30(
                mech_trace::rd!("1001e620", this.cast::<i32>(), read_unaligned),
                param_1
            )
        );
        if (first_call_result as i32) >= 0 {
            FUN_10017ec0(
                mech_trace::rd!("1001e620", this.cast::<i32>(), read_unaligned),
                param_1,
                0,
            );
        }
    }
}
