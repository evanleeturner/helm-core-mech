// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c07c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006f200(param_1: u32) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10040590(
        this: *mut core::ffi::c_void,
        param_1: *mut i8,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> *mut i32;
    fn FUN_10013330(this: *mut core::ffi::c_void, param_1: *mut i32, param_2: i32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100402b0(param_1: *mut i32);
}

#[unsafe(export_name = "FUN_10050c30")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn CreateTextItem(
    text_context: *mut core::ffi::c_void,
    x: i32,
    y: i32,
    text: *mut i8,
    remap_table: i32,
) -> *mut i32 {
    mech_trace::enter!("10050c30", text_context, x, y, text, remap_table);
    // SAFETY: callees and globals are the game's own; `text_context` points at the
    // caller's object, whose field at 0x410 is read as the original does.
    unsafe {
        let mut param_3 = text;
        if param_3.is_null() {
            param_3 = &raw mut DAT_1009c07c;
        }
        let item_block = mech_trace::cr!("10050c30", "1006f200", FUN_1006f200(0x3e));
        let item: *mut i32 = if item_block.is_null() {
            core::ptr::null_mut()
        } else {
            mech_trace::cr!(
                "10050c30",
                "10040590",
                FUN_10040590(item_block, param_3, x, y, remap_table, text_context as i32)
            )
        };
        let surface = mech_trace::rd!(
            "10050c30",
            text_context
                .byte_add(0x410)
                .cast::<*mut core::ffi::c_void>(),
            read_unaligned
        );
        FUN_10013330(surface, item, 1);
        FUN_100402b0(item);
        mech_trace::rt!("10050c30", item)
    }
}
