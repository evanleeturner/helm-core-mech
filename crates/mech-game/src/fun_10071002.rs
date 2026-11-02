// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071002(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("10071002", param_1, param_2);
    // SAFETY: param_1 is the caller's structure address; reads happen at the
    // same offsets as the original.
    unsafe {
        mech_trace::rt!(
            "10071002",
            if param_2 < 0
                || mech_trace::rd!(
                    "10071002",
                    ((param_1.wrapping_add(0x4b4)) as *const i32),
                    read_unaligned
                ) <= param_2
            {
                0xffffffff
            } else {
                mech_trace::rd!(
                    "10071002",
                    ((param_1
                        .wrapping_add(0xc)
                        .wrapping_add(param_2.wrapping_mul(0xc)))
                        as *const u32),
                    read_unaligned
                )
            }
        )
    }
}
