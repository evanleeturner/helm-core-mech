// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081234() -> u32 {
    mech_trace::enter!("10081234");
    // SAFETY: the global is the game's own; its value is an address of a
    // structure whose field at offset 0x20 is read.
    unsafe {
        mech_trace::rt!(
            "10081234",
            mech_trace::rd!(
                "10081234",
                ((mech_trace::gr!("10081234", "100b3c9c", DAT_100b3c9c).wrapping_add(0x20))
                    as *const u32),
                read_unaligned
            )
        )
    }
}
