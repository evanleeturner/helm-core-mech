// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_100879ee(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("100879ee", param_1, param_2);
    let mut local_8: i32 = 0;
    loop {
        if param_1 <= local_8 {
            return mech_trace::rt!("100879ee", 1);
        }
        let base = param_2.wrapping_add(local_8.wrapping_mul(0xc));
        // SAFETY: param_2 is the address of an array of 12-byte entries (pointer, mask, mode)
        // supplied by the caller; the pointer in each entry points at a readable u32.
        unsafe {
            let ptr = mech_trace::rd!("100879ee", (base as usize as *const u32), read_unaligned);
            let mask = mech_trace::rd!(
                "100879ee",
                ((base.wrapping_add(4)) as usize as *const u32),
                read_unaligned
            );
            let mode = mech_trace::rd!(
                "100879ee",
                ((base.wrapping_add(8)) as usize as *const i32),
                read_unaligned
            );
            let value = mech_trace::rd!("100879ee", (ptr as usize as *const u32), read_unaligned);
            if mode == 1 {
                if (mask & value) != 0 {
                    return mech_trace::rt!("100879ee", 0);
                }
            } else if (mask & value) == 0 {
                return mech_trace::rt!("100879ee", 0);
            }
        }
        local_8 = local_8.wrapping_add(1);
    }
}
