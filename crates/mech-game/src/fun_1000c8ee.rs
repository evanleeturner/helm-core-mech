// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f5518: i32;
    static mut DAT_100a7574: i32;
    static mut DAT_100a7570: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c8ee(param_1: i32) -> u32 {
    mech_trace::enter!("1000c8ee", param_1);
    let mut local_8: u32 = 0xffffffff;
    if -1 < param_1 {
        let mut local_c: u32 = 0;
        // SAFETY: the globals are the game's own; the tables they address
        // hold at least DAT_101f5518 entries each.
        unsafe {
            while (local_c as i32) < mech_trace::gr!("1000c8ee", "101f5518", DAT_101f5518) {
                let a = (mech_trace::gr!("1000c8ee", "100a7574", DAT_100a7574) as u32)
                    .wrapping_add(local_c.wrapping_mul(4));
                if mech_trace::rd!("1000c8ee", (a as *const i32), read_unaligned) == param_1 {
                    let b = (mech_trace::gr!("1000c8ee", "100a7570", DAT_100a7570) as u32)
                        .wrapping_add(local_c.wrapping_mul(4));
                    local_8 = mech_trace::rd!("1000c8ee", (b as *const u32), read_unaligned);
                }
                local_c = local_c.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("1000c8ee", local_8)
}
