// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049844(param_1: i32) -> bool {
    mech_trace::enter!("10049844", param_1);
    // SAFETY: param_1 is an address of a game structure; the i32 at offset 0xc is read.
    unsafe {
        mech_trace::rt!(
            "10049844",
            mech_trace::rd!(
                "10049844",
                ((param_1 as usize).wrapping_add(0xc) as *const i32),
                read_unaligned
            ) == 0
        )
    }
}
