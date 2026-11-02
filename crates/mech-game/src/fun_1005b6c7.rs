// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b6c7(param_1: i32) -> u32 {
    mech_trace::enter!("1005b6c7", param_1);
    // SAFETY: param_1 is an address the caller passes; the u32 at offset 4 is read.
    unsafe {
        mech_trace::rt!(
            "1005b6c7",
            mech_trace::rd!(
                "1005b6c7",
                ((param_1 as usize).wrapping_add(4) as *const u32),
                read_unaligned
            )
        )
    }
}
