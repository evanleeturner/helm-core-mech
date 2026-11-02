// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10042730(priorCallResult: i32, forwardedPointer: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100364f0(priorCallResult: i32, forwardedPointer: *mut i32) {
    if priorCallResult != -1 && priorCallResult != -2 {
        // SAFETY: the callee is the game's own function; arguments are forwarded unchanged.
        unsafe {
            FUN_10042730(priorCallResult, forwardedPointer);
        }
    }
}
