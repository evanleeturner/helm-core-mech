// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d5cb(param_1: i32) -> u32 {
    mech_trace::enter!("1001d5cb", param_1);
    // SAFETY: param_1 is an address supplied by the caller; the u32 at offset 0x18 is read where the original does.
    unsafe {
        mech_trace::rt!(
            "1001d5cb",
            mech_trace::rd!(
                "1001d5cb",
                ((param_1.wrapping_add(0x18)) as *const u32),
                read_unaligned
            )
        )
    }
}
