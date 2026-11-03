// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10012620(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10055d60(param_1: i32) {
    mech_trace::enter!("10055d60", param_1);
    // SAFETY: param_1 points at the caller's object; its field at offset 4 is
    // read and passed to the game's own callee, which the original tail-calls.
    unsafe {
        let v = mech_trace::rd!(
            "10055d60",
            ((param_1 as usize + 4) as *const i32),
            read_unaligned
        );
        FUN_10012620(v);
    }
}
