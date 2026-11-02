// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001cac9(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
) -> u32 {
    mech_trace::enter!("1001cac9", param_1, param_2, param_3, param_4);
    // SAFETY: param_1 is an address of the caller's structure; the output
    // pointers are written only when non-null, as in the original.
    unsafe {
        if !param_2.is_null() {
            mech_trace::wr!(
                "1001cac9",
                param_2,
                write_unaligned,
                mech_trace::rd!(
                    "1001cac9",
                    ((param_1.wrapping_add(0x34)) as *const u32),
                    read_unaligned
                )
            );
        }
        if !param_3.is_null() {
            mech_trace::wr!(
                "1001cac9",
                param_3,
                write_unaligned,
                mech_trace::rd!(
                    "1001cac9",
                    ((param_1.wrapping_add(0x38)) as *const u32),
                    read_unaligned
                )
            );
        }
        if !param_4.is_null() {
            mech_trace::wr!(
                "1001cac9",
                param_4,
                write_unaligned,
                mech_trace::rd!(
                    "1001cac9",
                    ((param_1.wrapping_add(0x3c)) as *const u32),
                    read_unaligned
                )
            );
        }
        mech_trace::rt!(
            "1001cac9",
            mech_trace::rd!(
                "1001cac9",
                ((param_1.wrapping_add(0x40)) as *const u32),
                read_unaligned
            )
        )
    }
}
