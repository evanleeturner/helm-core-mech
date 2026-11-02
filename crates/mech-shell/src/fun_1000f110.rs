// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008da7c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f110(param_1: i32) -> u32 {
    mech_trace::enter!("1000f110", param_1);
    if param_1 > -1 && param_1 < 0x20 {
        // SAFETY: DAT_1008da7c is the game's own table; the index is bounds-checked
        // to 0..0x20 slots of 0x13 dwords each.
        unsafe {
            let base = &raw mut DAT_1008da7c;
            let p = base.offset((param_1 as isize) * 0x13);
            if (mech_trace::rd!("1000f110", p, read_unaligned) & 0x80000000) != 0
                && (mech_trace::rd!("1000f110", p, read_unaligned) & 1) == 0
            {
                return mech_trace::rt!("1000f110", 1);
            }
        }
    }
    mech_trace::rt!("1000f110", 0)
}
