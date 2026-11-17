// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008da7c: u32;
}

#[unsafe(export_name = "FUN_1000f170")]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn IsModalMovieOpen() -> u32 {
    mech_trace::enter!("1000f170");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        if (mech_trace::gr!("1000f170", "1008da7c", DAT_1008da7c) & 0x80000000) != 0 {
            if (mech_trace::gr!("1000f170", "1008da7c", DAT_1008da7c) & 0x1000) != 0 {
                return mech_trace::rt!("1000f170", 1);
            }
        }
    }
    mech_trace::rt!("1000f170", 0)
}
