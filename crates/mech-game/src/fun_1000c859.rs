// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215620: i32;
    static mut DAT_100a7564: i32;
    static mut DAT_100a7560: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c859(param_1: i32) -> u32 {
    mech_trace::enter!("1000c859", param_1);
    let mut local_8: u32 = 0xffff_ffff;
    let mut local_c: u32 = 0;
    // SAFETY: the globals are the game's own; the two tables they point at
    // hold at least DAT_10215620 entries, read where the original reads.
    unsafe {
        while (local_c as i32) < mech_trace::gr!("1000c859", "10215620", DAT_10215620) {
            let a = (mech_trace::gr!("1000c859", "100a7564", DAT_100a7564) as u32)
                .wrapping_add(local_c.wrapping_mul(4));
            if mech_trace::rd!("1000c859", (a as *const i32), read_unaligned) == param_1 {
                let b = (mech_trace::gr!("1000c859", "100a7560", DAT_100a7560) as u32)
                    .wrapping_add(local_c.wrapping_mul(4));
                local_8 = mech_trace::rd!("1000c859", (b as *const u32), read_unaligned);
            }
            local_c = local_c.wrapping_add(1);
        }
    }
    mech_trace::rt!("1000c859", local_8)
}
