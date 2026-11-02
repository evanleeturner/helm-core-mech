// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn _rand();
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10056be0() -> i64 {
    mech_trace::enter!("10056be0");
    // SAFETY: both callees are the game's own runtime routines; they take no
    // arguments and pass their state through the machine's registers.
    unsafe {
        _rand();
        mech_trace::rt!("10056be0", mech_trace::cr!("10056be0", "__ftol", __ftol()))
    }
}
