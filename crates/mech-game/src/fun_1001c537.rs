// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c537(param_1: i32) -> u32 {
    mech_trace::enter!("1001c537", param_1);
    // SAFETY: param_1 is the address of a game structure; the pointer at
    // offset 0x20 is read, and dereferenced only when non-null.
    unsafe {
        let p = mech_trace::rd!(
            "1001c537",
            ((param_1 as usize).wrapping_add(0x20) as *const *const u32),
            read_unaligned
        );
        mech_trace::rt!(
            "1001c537",
            if p.is_null() {
                0
            } else {
                mech_trace::rd!("1001c537", p, read_unaligned)
            }
        )
    }
}
