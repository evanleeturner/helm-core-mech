// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098bac: *mut c_void;
    static mut DAT_10098bb0: *mut c_void;
    static mut DAT_10098bb4: *mut c_void;
    static mut DAT_10098bb8: *mut c_void;
    static mut DAT_10098bbc: *mut c_void;
    static mut DAT_10098bcc: *mut c_void;
    static mut DAT_10098bd0: *mut c_void;
    static mut DAT_10098bd4: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    // Really __thiscall (this in ECX, one stack argument popped by the callee);
    // it is only ever called through `call_thiscall` below.
    fn FUN_1005f7d0();
}

#[inline(always)]
unsafe fn call_thiscall(this: *mut c_void) {
    // SAFETY: the callee is the game's __thiscall function taking `this` in ECX
    // and one stack argument (0), which it pops itself.
    unsafe {
        mech_trace::log::debug!(
            f = "10043fb0", b = "1005f7d0", a:% = mech_trace::Args([
                mech_trace::__shown!(this),
            ]); "asm.in"
        );
        core::arch::asm!(
            "push 0",
            "call {f}",
            f = sym FUN_1005f7d0,
            in("ecx") this,
            clobber_abi("C"),
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043fb0(param_1: u32) {
    mech_trace::enter!("10043fb0", param_1);
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        match param_1 {
            0 => call_thiscall(mech_trace::gr!("10043fb0", "10098bb0", DAT_10098bb0)),
            1 => call_thiscall(mech_trace::gr!("10043fb0", "10098bb4", DAT_10098bb4)),
            2 => call_thiscall(mech_trace::gr!("10043fb0", "10098bb8", DAT_10098bb8)),
            3 => call_thiscall(mech_trace::gr!("10043fb0", "10098bac", DAT_10098bac)),
            4 => call_thiscall(mech_trace::gr!("10043fb0", "10098bbc", DAT_10098bbc)),
            5 => call_thiscall(mech_trace::gr!("10043fb0", "10098bd4", DAT_10098bd4)),
            6 => call_thiscall(mech_trace::gr!("10043fb0", "10098bcc", DAT_10098bcc)),
            7 => call_thiscall(mech_trace::gr!("10043fb0", "10098bd0", DAT_10098bd0)),
            _ => {}
        }
    }
}
