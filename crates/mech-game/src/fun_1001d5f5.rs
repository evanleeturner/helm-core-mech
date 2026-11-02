// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d5f5(param_1: *mut u16) -> u16 {
    mech_trace::enter!("1001d5f5", param_1);
    // SAFETY: param_1 points at the caller's u16, read where the original does.
    unsafe {
        mech_trace::rt!(
            "1001d5f5",
            mech_trace::rd!("1001d5f5", param_1, read_unaligned) & 0x10f
        )
    }
}
