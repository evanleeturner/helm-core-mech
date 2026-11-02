// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008371c: i32;
    static mut DAT_10083720: i32;
    static mut DAT_10083724: i32;
    static mut DAT_10083728: i32;
    static mut DAT_1008372c: i32;
    static mut DAT_10083710: i32;
    static mut DAT_10084c58: i32;
    static mut DAT_10084c5c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100046f0() {
    mech_trace::enter!("100046f0");
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_10084c58/DAT_10084c5c is indexed by byte offset as the original does.
    unsafe {
        let i_var1 = (mech_trace::gr!("100046f0", "1008371c", DAT_1008371c) % 10000)
            .wrapping_mul(0x10) as isize;
        mech_trace::gw!(
            "100046f0",
            "10083724",
            DAT_10083724,
            mech_trace::rd!(
                "100046f0",
                (&raw mut DAT_10084c5c).byte_offset(i_var1),
                read_unaligned
            )
        );
        mech_trace::gw!(
            "100046f0",
            "10083720",
            DAT_10083720,
            mech_trace::rd!(
                "100046f0",
                (&raw mut DAT_10084c58).byte_offset(i_var1),
                read_unaligned
            )
        );
        if 9999 < mech_trace::gr!("100046f0", "1008371c", DAT_1008371c) {
            mech_trace::gw!(
                "100046f0",
                "10083724",
                DAT_10083724,
                mech_trace::gr!("100046f0", "10083724", DAT_10083724) / 2
            );
        }
        mech_trace::gw!(
            "100046f0",
            "10083728",
            DAT_10083728,
            ((mech_trace::gr!("100046f0", "1008371c", DAT_1008371c) < 10000) as i32)
                .wrapping_sub(1)
                & 7
        );
        mech_trace::gw!(
            "100046f0",
            "1008372c",
            DAT_1008372c,
            mech_trace::gr!("100046f0", "10083710", DAT_10083710).wrapping_sub(mech_trace::gr!(
                "100046f0",
                "10083724",
                DAT_10083724
            ))
        );
    }
}
