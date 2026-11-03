// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a62c4: i32;
    static mut DAT_100ab6e8: u8;
    static mut s_Error___Existing_BlockBegin_or_m_100a6310: u8;
    static mut DAT_10095a98: u32;
    static mut DAT_100c61b4: *mut u8;
    static mut DAT_100a50f0: i32;
    static mut DAT_100d0d40: u16;
    static mut DAT_100d0d50: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn FUN_1006fde0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e4f0(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
) {
    mech_trace::enter!(
        "1005e4f0", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the globals and callees are the game's own memory; DAT_100c61b4
    // points at a writable block buffer with room for the 0x20-byte record.
    unsafe {
        if mech_trace::gr!("1005e4f0", "100a62c4", DAT_100a62c4) != 0 {
            mech_trace::cr!(
                "1005e4f0",
                "_fwprintf",
                _fwprintf(
                    &raw mut DAT_100ab6e8,
                    &raw const s_Error___Existing_BlockBegin_or_m_100a6310,
                )
            );
            FUN_1006fde0(0);
        } else {
            mech_trace::gw!(
                "1005e4f0",
                "100a62c4",
                DAT_100a62c4,
                mech_trace::gr!("1005e4f0", "100a62c4", DAT_100a62c4).wrapping_add(1)
            );
        }
        let v = mech_trace::gr!("1005e4f0", "10095a98", DAT_10095a98);
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4).cast::<u32>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(4)
                .cast::<u32>(),
            write_unaligned,
            0x20
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(8)
                .cast::<u32>(),
            write_unaligned,
            param_1
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(0xc)
                .cast::<u32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(0x10)
                .cast::<u32>(),
            write_unaligned,
            param_3
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(0x14)
                .cast::<u32>(),
            write_unaligned,
            param_4
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(0x18)
                .cast::<u32>(),
            write_unaligned,
            param_5
        );
        mech_trace::wr!(
            "1005e4f0",
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4)
                .byte_add(0x1c)
                .cast::<u32>(),
            write_unaligned,
            param_6
        );
        mech_trace::gw!(
            "1005e4f0",
            "100c61b4",
            DAT_100c61b4,
            mech_trace::gr!("1005e4f0", "100c61b4", DAT_100c61b4).byte_add(0x20)
        );
        let mut e = mech_trace::gr!("1005e4f0", "100a50f0", DAT_100a50f0);
        if e <= 0x20 {
            e = 0x20;
        }
        mech_trace::gw!("1005e4f0", "100a50f0", DAT_100a50f0, e);
        let idx = mech_trace::gr!("1005e4f0", "100d0d40", DAT_100d0d40) as usize;
        mech_trace::wr!(
            "1005e4f0",
            (&raw mut DAT_100d0d50).add(idx),
            write_unaligned,
            0
        );
    }
}
