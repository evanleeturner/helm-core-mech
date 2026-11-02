// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10091fcc: *mut c_void;
    static mut DAT_10091fd0: *mut c_void;
    static mut DAT_10091fd4: *mut c_void;
    static mut DAT_10091fd8: *mut c_void;
    static mut DAT_10091fdc: *mut c_void;
}

// The original is __thiscall (this in ECX, one stack argument, callee pops).
// "fastcall" with a dummy EDX argument produces the same register and stack layout.
#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005f7d0(this: *mut c_void, unused_edx: i32, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035e10() {
    mech_trace::enter!("10035e10");
    // SAFETY: the globals and the callee are the game's own; each global is
    // read just before it is passed as the `this` pointer.
    unsafe {
        mech_trace::cr!(
            "10035e10",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10035e10", "10091fcc", DAT_10091fcc), 0, 0)
        );
        mech_trace::cr!(
            "10035e10",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10035e10", "10091fd0", DAT_10091fd0), 0, 0)
        );
        mech_trace::cr!(
            "10035e10",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10035e10", "10091fd4", DAT_10091fd4), 0, 0)
        );
        mech_trace::cr!(
            "10035e10",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10035e10", "10091fd8", DAT_10091fd8), 0, 0)
        );
        mech_trace::cr!(
            "10035e10",
            "1005f7d0",
            FUN_1005f7d0(mech_trace::gr!("10035e10", "10091fdc", DAT_10091fdc), 0, 0)
        );
    }
}
