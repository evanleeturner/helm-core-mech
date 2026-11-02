// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100988dc: *mut c_void;
    static mut DAT_100988e0: *mut c_void;
    static mut DAT_100988e4: *mut c_void;
    static mut DAT_100988e8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f7d0(this: *mut c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100430b0(param_1: u32) {
    mech_trace::enter!("100430b0", param_1);
    // SAFETY: the globals and the callee are the game's own; each global is
    // read by value just before the call that uses it.
    unsafe {
        match param_1 {
            0 => {
                mech_trace::cr!(
                    "100430b0",
                    "1005f7d0",
                    FUN_1005f7d0(mech_trace::gr!("100430b0", "100988dc", DAT_100988dc), 0)
                );
            }
            1 => {
                mech_trace::cr!(
                    "100430b0",
                    "1005f7d0",
                    FUN_1005f7d0(mech_trace::gr!("100430b0", "100988e8", DAT_100988e8), 0)
                );
            }
            2 => {
                mech_trace::cr!(
                    "100430b0",
                    "1005f7d0",
                    FUN_1005f7d0(mech_trace::gr!("100430b0", "100988e4", DAT_100988e4), 0)
                );
            }
            3 => {
                mech_trace::cr!(
                    "100430b0",
                    "1005f7d0",
                    FUN_1005f7d0(mech_trace::gr!("100430b0", "100988e0", DAT_100988e0), 0)
                );
            }
            _ => {}
        }
    }
}
