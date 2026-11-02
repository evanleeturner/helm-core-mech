// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ca78(param_1: i32) -> u16 {
    mech_trace::enter!("1001ca78", param_1);
    // SAFETY: param_1 is an address the caller supplies; the u16 at offset 2
    // is read where the original reads it.
    unsafe {
        mech_trace::rt!(
            "1001ca78",
            mech_trace::rd!(
                "1001ca78",
                ((param_1 as usize).wrapping_add(2) as *const u16),
                read_unaligned
            )
        )
    }
}
