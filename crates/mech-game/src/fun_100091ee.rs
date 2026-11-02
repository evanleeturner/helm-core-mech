// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100091ee(param_1: i32, param_2: u8) -> u32 {
    mech_trace::enter!("100091ee", param_1, param_2);
    // SAFETY: param_1 is the address of a caller-owned structure; the two
    // fields are read where the original reads them.
    unsafe {
        let flags = mech_trace::rd!(
            "100091ee",
            ((param_1.wrapping_add(0x24)) as *const u16),
            read_unaligned
        );
        let val = mech_trace::rd!(
            "100091ee",
            ((param_1.wrapping_add(0x26)) as *const i16),
            read_unaligned
        );
        mech_trace::rt!(
            "100091ee",
            if (flags & 0x20) == 0 || (val == 0 && 1u32.wrapping_shl((param_2 & 0x1f) as u32) == 0)
            {
                0
            } else {
                1
            }
        )
    }
}
