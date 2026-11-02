// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edfc: Option<unsafe extern "C" fn(i32)>;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10016cd0() {
    mech_trace::enter!("10016cd0");
    // SAFETY: the global is the game's own callback slot; when set, it is a
    // valid cdecl function taking one int.
    unsafe {
        if let Some(f) = mech_trace::gr!("10016cd0", "1008edfc", DAT_1008edfc) {
            mech_trace::cr!("10016cd0", "ind", f(0));
        }
    }
}
