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

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006984a(
    drawTarget_Likely: *mut i32,
    destX_Likely: i32,
    destY_Likely: i32,
    font_Likely: i32,
    text: *mut u8,
    param_6: i32,
) {
    let mut dest_x = destX_Likely;
    let mut text = text;
    loop {
        // SAFETY: the callee is the game's own; text points at a
        // NUL-terminated byte string supplied by the caller.
        unsafe {
            let advance_width = FUN_100696b7(
                drawTarget_Likely,
                dest_x,
                destY_Likely,
                font_Likely,
                i32::from(*text),
                param_6,
            );
            dest_x = dest_x.wrapping_add(advance_width);
            text = text.add(1);
            if *text == 0 {
                break;
            }
        }
    }
}
