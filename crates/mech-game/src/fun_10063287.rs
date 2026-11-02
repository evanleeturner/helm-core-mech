// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063287(param_1: i32) -> u32 {
    mech_trace::enter!("10063287", param_1);
    let mut local_8: u32 = 0xffffffff;
    // SAFETY: param_1 points at the game's object; the fields read are the
    // original's own, at the same offsets and in the same order.
    unsafe {
        let idx = mech_trace::rd!(
            "10063287",
            ((param_1 as usize).wrapping_add(0x3e0) as *const i32),
            read_unaligned
        );
        if idx != -1 {
            let base = mech_trace::rd!(
                "10063287",
                ((param_1 as usize).wrapping_add(0x54) as *const i32),
                read_unaligned
            );
            let addr = idx.wrapping_mul(0x40).wrapping_add(8).wrapping_add(base);
            if mech_trace::rd!("10063287", (addr as usize as *const i32), read_unaligned) == 1 {
                local_8 = 1;
            } else {
                local_8 = 0;
            }
        }
    }
    mech_trace::rt!("10063287", local_8)
}
