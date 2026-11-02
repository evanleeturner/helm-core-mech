// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f34(param_1: i32) -> u32 {
    mech_trace::enter!("10063f34", param_1);
    // SAFETY: param_1 is the address of a caller-owned structure; the u32 at
    // offset 0xa0 is read where the original does.
    unsafe {
        mech_trace::rt!(
            "10063f34",
            mech_trace::rd!(
                "10063f34",
                ((param_1.wrapping_add(0xa0)) as *const u32),
                read_unaligned
            )
        )
    }
}
