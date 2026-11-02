// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f28d(param_1: i32) -> u32 {
    mech_trace::enter!("1002f28d", param_1);
    // SAFETY: param_1 is an address the caller guarantees points at readable
    // game memory; the field at offset 0x6c is read as the original does.
    unsafe {
        mech_trace::rt!(
            "1002f28d",
            mech_trace::rd!(
                "1002f28d",
                ((param_1.wrapping_add(0x6c)) as *const u32),
                read_unaligned
            )
        )
    }
}
