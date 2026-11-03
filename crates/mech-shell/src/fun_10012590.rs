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
    mech_trace::enter!(
        "10012590",
        bounds_owner_likely,
        rect_x_likely,
        rect_y_likely,
        rect_width_likely,
        rect_height_likely
    );
    // SAFETY: the owner points at the game's own object; the four bounds
    // fields are read at their byte offsets, in the original's order.
    unsafe {
        if rect_x_likely
            <= mech_trace::rd!(
                "10012590",
                bounds_owner_likely.byte_add(0x8a).cast::<i32>(),
                read_unaligned
            )
            && mech_trace::rd!(
                "10012590",
                bounds_owner_likely.byte_add(0x82).cast::<i32>(),
                read_unaligned
            ) <= rect_width_likely
                .wrapping_sub(1)
                .wrapping_add(rect_x_likely)
            && rect_y_likely
                <= mech_trace::rd!(
                    "10012590",
                    bounds_owner_likely.byte_add(0x8e).cast::<i32>(),
                    read_unaligned
                )
            && mech_trace::rd!(
                "10012590",
                bounds_owner_likely.byte_add(0x86).cast::<i32>(),
                read_unaligned
            ) <= rect_height_likely
                .wrapping_sub(1)
                .wrapping_add(rect_y_likely)
        {
            return mech_trace::rt!("10012590", 1);
        }
    }
    mech_trace::rt!("10012590", 0)
}
