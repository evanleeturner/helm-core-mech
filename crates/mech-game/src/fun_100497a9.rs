// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100497a9(param_1: i32) -> u32 {
    mech_trace::enter!("100497a9", param_1);
    let mut local_8: u32 = 0;
    // SAFETY: param_1 is the address of a caller-owned structure whose fields at
    // offsets 8 and 0xc are read as the original does.
    unsafe {
        let count = mech_trace::rd!(
            "100497a9",
            ((param_1 as u32).wrapping_add(0xc) as *const i32),
            read_unaligned
        );
        if 0 < count {
            let base = mech_trace::rd!(
                "100497a9",
                ((param_1 as u32).wrapping_add(8) as *const i32),
                read_unaligned
            );
            let addr = base.wrapping_sub(4).wrapping_add(count.wrapping_mul(4));
            local_8 = mech_trace::rd!("100497a9", (addr as u32 as *const u32), read_unaligned);
        }
    }
    mech_trace::rt!("100497a9", local_8)
}
