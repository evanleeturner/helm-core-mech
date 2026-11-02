// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b34d4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ClipCursor(param_1: *const core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ece8() -> u32 {
    mech_trace::enter!("1005ece8");
    // SAFETY: ClipCursor is the system API and the global is the game's own memory.
    unsafe {
        mech_trace::cr!("1005ece8", "ClipCursor", ClipCursor(core::ptr::null()));
        mech_trace::gw!("1005ece8", "100b34d4", DAT_100b34d4, 0);
    }
    mech_trace::rt!("1005ece8", 0)
}
