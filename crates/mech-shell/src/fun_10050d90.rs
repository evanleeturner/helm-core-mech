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

#[unsafe(export_name = "FUN_10050d90")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn DrawTextInContext(
    text_context: *mut c_void,
    x: i32,
    y: i32,
    text: *mut u8,
    remap_table: *mut u8,
) {
    mech_trace::enter!("10050d90", text_context, x, y, text, remap_table);
    // SAFETY: `text_context` points at the game's object; the inner pointer at
    // offset 0x410 and the first dword are read as the original does, and
    // the callee is the game's own.
    unsafe {
        let surface = mech_trace::rd!(
            "10050d90",
            text_context.byte_add(0x410).cast::<*mut c_void>(),
            read_unaligned
        );
        let font = mech_trace::rd!("10050d90", text_context.cast::<i32>(), read_unaligned);
        FUN_10013180(surface, x, y, font, text, remap_table);
    }
}
