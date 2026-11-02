// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_101600e4: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1002c39a() -> u32 {
    mech_trace::enter!("1002c39a");
    // SAFETY: the globals are the game's own memory; the table entry is an
    // address of a record, and the result indexes the game's 0x3c-byte array.
    unsafe {
        let idx = mech_trace::gr!("1002c39a", "100adf38", DAT_100adf38) as usize;
        let entry = mech_trace::rd!(
            "1002c39a",
            (&raw mut DAT_100eef40).cast::<u32>().add(idx),
            read_unaligned
        );
        let n = mech_trace::rd!(
            "1002c39a",
            ((entry as usize).wrapping_add(8) as *const i32),
            read_unaligned
        );
        mech_trace::rt!(
            "1002c39a",
            mech_trace::rd!(
                "1002c39a",
                (&raw mut DAT_101600e4)
                    .cast::<u8>()
                    .byte_add((n as usize).wrapping_mul(0x3c))
                    .cast::<u32>(),
                read_unaligned
            )
        )
    }
}
