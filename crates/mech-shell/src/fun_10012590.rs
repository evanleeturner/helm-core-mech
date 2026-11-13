// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10012590(
    bounds_owner_likely: *mut u8,
    rect_x_likely: i32,
    rect_y_likely: i32,
    rect_width_likely: i32,
    rect_height_likely: i32,
) -> u32 {
    // A hot port: its call is one `fn.reads` line at its return, its reads
    // counted, in place of its `fn.in`, a `mem.rd` a read, and its `fn.ret`.
    let mut reads: u32 = 0;
    let mut counted = |value: i32| {
        reads += 1;
        value
    };
    // SAFETY: the owner points at the game's own object; the four bounds
    // fields are read at their byte offsets, in the original's order.
    let inside = unsafe {
        rect_x_likely
            <= counted(
                bounds_owner_likely
                    .byte_add(0x8a)
                    .cast::<i32>()
                    .read_unaligned(),
            )
            && counted(
                bounds_owner_likely
                    .byte_add(0x82)
                    .cast::<i32>()
                    .read_unaligned(),
            ) <= rect_width_likely
                .wrapping_sub(1)
                .wrapping_add(rect_x_likely)
            && rect_y_likely
                <= counted(
                    bounds_owner_likely
                        .byte_add(0x8e)
                        .cast::<i32>()
                        .read_unaligned(),
                )
            && counted(
                bounds_owner_likely
                    .byte_add(0x86)
                    .cast::<i32>()
                    .read_unaligned(),
            ) <= rect_height_likely
                .wrapping_sub(1)
                .wrapping_add(rect_y_likely)
    };
    let answer = u32::from(inside);
    mech_trace::log::debug!(
        f = "10012590",
        a:% = mech_trace::Args([
            mech_trace::__shown!(bounds_owner_likely),
            mech_trace::__shown!(rect_x_likely),
            mech_trace::__shown!(rect_y_likely),
            mech_trace::__shown!(rect_width_likely),
            mech_trace::__shown!(rect_height_likely)
        ]),
        n:% = mech_trace::__shown!(reads), v:% = mech_trace::__shown!(answer); "fn.reads"
    );
    answer
}
