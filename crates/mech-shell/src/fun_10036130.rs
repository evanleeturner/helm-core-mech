// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093b2c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036130(param_1: i32) -> u32 {
    mech_trace::enter!("10036130", param_1);
    // SAFETY: the global holds the game's own table pointer; the reads are
    // at the offsets the original uses.
    unsafe {
        let mut p = param_1;
        if p < 0 {
            p = mech_trace::rd!(
                "10036130",
                (mech_trace::gr!("10036130", "10093b2c", DAT_10093b2c).wrapping_add(4)
                    as *const i32),
                read_unaligned
            );
        }
        if p < mech_trace::rd!(
            "10036130",
            (mech_trace::gr!("10036130", "10093b2c", DAT_10093b2c).wrapping_add(8) as *const i32),
            read_unaligned
        ) && p < mech_trace::rd!(
            "10036130",
            (mech_trace::gr!("10036130", "10093b2c", DAT_10093b2c).wrapping_add(0xc) as *const i32),
            read_unaligned
        ) {
            let addr = mech_trace::gr!("10036130", "10093b2c", DAT_10093b2c)
                .wrapping_add((p.wrapping_mul(0x40)) as u32)
                .wrapping_add(0x14);
            return mech_trace::rt!(
                "10036130",
                mech_trace::rd!("10036130", (addr as *const u32), read_unaligned)
            );
        }
        mech_trace::rt!("10036130", 0xffff_ffff)
    }
}
