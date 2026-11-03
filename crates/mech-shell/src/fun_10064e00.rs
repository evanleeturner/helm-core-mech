// SPDX-License-Identifier: MIT

/// Whether the point (`first_value`, `second_value`) lies inside `bounds`,
/// a rectangle of four `i32`s (left, top, right, bottom), edges included.
///
/// Fixed against the original's code, HC6 wall 4: mercs' rebuilt C (and the
/// port taken from it) reversed all four comparisons, `right <= x <= left`
/// and `bottom <= y <= top`, so no real rectangle could hold a point and the
/// main menu's click found no button. The original (`cmp edx,eax` with
/// `jl`/`jg` to the miss) is `left <= x <= right` and `top <= y <= bottom`.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10064e00(
    bounds: *const u8,
    first_value: i32,
    second_value: i32,
) -> i32 {
    mech_trace::enter!("10064e00", bounds, first_value, second_value);
    // SAFETY: bounds points at the caller's structure of at least 16 bytes.
    unsafe {
        if second_value
            <= mech_trace::rd!(
                "10064e00",
                bounds.byte_add(0xc).cast::<i32>(),
                read_unaligned
            )
            && mech_trace::rd!("10064e00", bounds.byte_add(4).cast::<i32>(), read_unaligned)
                <= second_value
            && first_value
                <= mech_trace::rd!("10064e00", bounds.byte_add(8).cast::<i32>(), read_unaligned)
            && mech_trace::rd!("10064e00", bounds.cast::<i32>(), read_unaligned) <= first_value
        {
            return mech_trace::rt!("10064e00", 1);
        }
    }
    mech_trace::rt!("10064e00", 0)
}
