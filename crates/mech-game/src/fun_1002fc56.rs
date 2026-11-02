// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fc56(param_1: i32) -> u32 {
    mech_trace::enter!("1002fc56", param_1);
    // SAFETY: param_1 is an address supplied by the caller; the u32 at
    // offset 8 is read as the original does.
    unsafe {
        mech_trace::rt!(
            "1002fc56",
            mech_trace::rd!(
                "1002fc56",
                ((param_1.wrapping_add(8)) as *const u32),
                read_unaligned
            )
        )
    }
}
