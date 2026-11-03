// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10055b10(param_1: i32) -> u32 {
    mech_trace::enter!("10055b10", param_1);
    // SAFETY: param_1 is the address of a caller-owned object with a
    // readable u32 at offset 0x10.
    unsafe {
        mech_trace::rt!(
            "10055b10",
            mech_trace::rd!(
                "10055b10",
                ((param_1 as usize).wrapping_add(0x10) as *const u32),
                read_unaligned
            )
        )
    }
}
