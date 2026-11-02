// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100a6798: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000bfd0() -> u32 {
    mech_trace::enter!("1000bfd0");
    // SAFETY: the globals are the game's own; they hold addresses of game
    // structures that are read at the offsets the original uses.
    unsafe {
        if mech_trace::rd!(
            "1000bfd0",
            ((mech_trace::gr!("1000bfd0", "10083508", DAT_10083508).wrapping_add(0x23bb))
                as *const i32),
            read_unaligned
        ) == 2
        {
            return mech_trace::rt!("1000bfd0", 0x3e);
        }
        mech_trace::rt!(
            "1000bfd0",
            mech_trace::rd!(
                "1000bfd0",
                (mech_trace::gr!("1000bfd0", "100a6798", DAT_100a6798).wrapping_add(4)
                    as *const u32),
                read_unaligned
            )
        )
    }
}
