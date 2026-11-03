// SPDX-License-Identifier: MIT

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
        if mech_trace::rd!(
            "10064e00",
            bounds.byte_add(0xc).cast::<i32>(),
            read_unaligned
        ) <= second_value
            && second_value
                <= mech_trace::rd!("10064e00", bounds.byte_add(4).cast::<i32>(), read_unaligned)
            && mech_trace::rd!("10064e00", bounds.byte_add(8).cast::<i32>(), read_unaligned)
                <= first_value
            && first_value <= mech_trace::rd!("10064e00", bounds.cast::<i32>(), read_unaligned)
        {
            return mech_trace::rt!("10064e00", 1);
        }
    }
    mech_trace::rt!("10064e00", 0)
}
