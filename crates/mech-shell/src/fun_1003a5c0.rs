// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10064af0(this: *mut core::ffi::c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a5c0(param_1: *mut core::ffi::c_void) {
    mech_trace::enter!("1003a5c0", param_1);
    // SAFETY: the callee is the game's own method; param_1 is the object
    // the caller passes as its receiver.
    unsafe {
        mech_trace::cr!("1003a5c0", "10064af0", FUN_10064af0(param_1, 10));
        mech_trace::cr!("1003a5c0", "10064af0", FUN_10064af0(param_1, 0xb));
    }
}
