// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100839fc: u32;
    static mut DAT_10083a00: [u32; 0];
    static mut DAT_10083a04: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a3f0(param_1: i32, param_2: u32) {
    mech_trace::enter!("1000a3f0", param_1, param_2);
    // SAFETY: the globals are the game's own memory; the index is bounded
    // by the check against 0x4e before the table writes.
    unsafe {
        if 0 < param_1 && mech_trace::gr!("1000a3f0", "100839fc", DAT_100839fc) < 0x4e {
            let idx =
                (mech_trace::gr!("1000a3f0", "100839fc", DAT_100839fc).wrapping_mul(2)) as usize;
            mech_trace::wr!(
                "1000a3f0",
                (&raw mut DAT_10083a00).cast::<u32>().add(idx),
                write_unaligned,
                param_1 as u32
            );
            mech_trace::wr!(
                "1000a3f0",
                (&raw mut DAT_10083a04).cast::<u32>().add(idx),
                write_unaligned,
                param_2
            );
            mech_trace::gw!(
                "1000a3f0",
                "100839fc",
                DAT_100839fc,
                mech_trace::gr!("1000a3f0", "100839fc", DAT_100839fc).wrapping_add(1)
            );
        }
    }
}
