// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022b24(param_1: i32) -> u32 {
    mech_trace::enter!("10022b24", param_1);
    // SAFETY: param_1 is an address of a game object whose field at 0x20 holds
    // a pointer to a structure readable at offset 0x400.
    unsafe {
        let p = mech_trace::rd!(
            "10022b24",
            ((param_1 as usize).wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        mech_trace::rt!(
            "10022b24",
            mech_trace::rd!(
                "10022b24",
                ((p as usize).wrapping_add(0x400) as *const u32),
                read_unaligned
            )
        )
    }
}
