// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069697(text: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10050b20(font_likely: *mut u8, text: *const i8) -> i32 {
    mech_trace::enter!("10050b20", font_likely, text);
    if text.is_null() {
        return mech_trace::rt!("10050b20", 0);
    }
    let mut text = text;
    let mut total: i32 = 0;
    // SAFETY: font_likely points at the font object whose first field is read;
    // text is a NUL-terminated string; the callee is the game's own.
    unsafe {
        let mut current = mech_trace::dr!("10050b20", text);
        while current != 0 {
            let font = mech_trace::rd!("10050b20", font_likely.cast::<i32>(), read_unaligned);
            let w = mech_trace::cr!(
                "10050b20",
                "10069697",
                FUN_10069697(font, mech_trace::dr!("10050b20", text) as i32)
            );
            total = total.wrapping_add(w as i32);
            current = mech_trace::dr!("10050b20", text.add(1));
            text = text.add(1);
        }
    }
    mech_trace::rt!("10050b20", total)
}
