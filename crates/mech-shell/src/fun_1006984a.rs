// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100696b7(
        drawTarget_Likely: *mut i32,
        destX_Likely: i32,
        destY_Likely: i32,
        font_Likely: i32,
        text: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(export_name = "FUN_1006984a")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn DrawTextString(
    drawTarget_Likely: *mut i32,
    start_x: i32,
    y: i32,
    font: i32,
    text: *mut u8,
    remap_table: i32,
) {
    mech_trace::enter!(
        "1006984a",
        drawTarget_Likely,
        start_x,
        y,
        font,
        text,
        remap_table
    );
    let mut dest_x = start_x;
    let mut text = text;
    loop {
        // SAFETY: the callee is the game's own; text points at a
        // NUL-terminated byte string supplied by the caller.
        unsafe {
            let advance_width = mech_trace::cr!(
                "1006984a",
                "100696b7",
                FUN_100696b7(
                    drawTarget_Likely,
                    dest_x,
                    y,
                    font,
                    i32::from(mech_trace::dr!("1006984a", text)),
                    remap_table,
                )
            );
            dest_x = dest_x.wrapping_add(advance_width);
            text = text.add(1);
            if mech_trace::dr!("1006984a", text) == 0 {
                break;
            }
        }
    }
}
