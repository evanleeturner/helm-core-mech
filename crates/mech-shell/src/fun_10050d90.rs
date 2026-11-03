// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10013180(
        this: *mut c_void,
        param_2: i32,
        text: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: *mut u8,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10050d90(
    this: *mut c_void,
    param_1: i32,
    param_2: i32,
    text: *mut u8,
    param_4: *mut u8,
) {
    mech_trace::enter!("10050d90", this, param_1, param_2, text, param_4);
    // SAFETY: `this` points at the game's object; the inner pointer at
    // offset 0x410 and the first dword are read as the original does, and
    // the callee is the game's own.
    unsafe {
        let inner = mech_trace::rd!(
            "10050d90",
            this.byte_add(0x410).cast::<*mut c_void>(),
            read_unaligned
        );
        let first = mech_trace::rd!("10050d90", this.cast::<i32>(), read_unaligned);
        FUN_10013180(inner, param_1, param_2, first, text, param_4);
    }
}
