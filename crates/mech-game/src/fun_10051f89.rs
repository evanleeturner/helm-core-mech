// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09e4: u32;
    static mut DAT_101151c0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10051f89() -> u32 {
    mech_trace::enter!("10051f89");
    // SAFETY: the globals are the game's own memory; the table is read at the
    // index held in DAT_100b09e4, which the function keeps below 0x7f.
    unsafe {
        let i_var1 = mech_trace::gr!("10051f89", "100b09e4", DAT_100b09e4).wrapping_mul(4);
        mech_trace::gw!(
            "10051f89",
            "100b09e4",
            DAT_100b09e4,
            mech_trace::gr!("10051f89", "100b09e4", DAT_100b09e4).wrapping_add(1)
        );
        if mech_trace::gr!("10051f89", "100b09e4", DAT_100b09e4) == 0x7f {
            mech_trace::gw!("10051f89", "100b09e4", DAT_100b09e4, 0);
        }
        mech_trace::rt!(
            "10051f89",
            mech_trace::rd!(
                "10051f89",
                (&raw mut DAT_101151c0)
                    .cast::<u8>()
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                read_unaligned
            )
        )
    }
}
