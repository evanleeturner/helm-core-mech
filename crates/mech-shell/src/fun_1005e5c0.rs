// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a62c4: i32;
    static mut DAT_100c61b4: *mut u8;
    static mut DAT_10095aa0: u32;
    static mut DAT_100a50f0: i32;
    static mut DAT_100d0d40: u16;
    static mut DAT_100d0d50: u32;
    static mut DAT_100ab6e8: u8;
    static mut s_Error___Missing_BlockBegin__100a6344: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn FUN_1006fde0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005e5c0() {
    mech_trace::enter!("1005e5c0");
    // SAFETY: the globals and callees are the game's own memory and code;
    // DAT_100c61b4 points at a writable buffer with room for 8 bytes.
    unsafe {
        if mech_trace::gr!("1005e5c0", "100a62c4", DAT_100a62c4) == 0 {
            mech_trace::cr!(
                "1005e5c0",
                "_fwprintf",
                _fwprintf(
                    &raw mut DAT_100ab6e8,
                    &raw const s_Error___Missing_BlockBegin__100a6344,
                )
            );
            FUN_1006fde0(0);
        } else {
            mech_trace::gw!(
                "1005e5c0",
                "100a62c4",
                DAT_100a62c4,
                mech_trace::gr!("1005e5c0", "100a62c4", DAT_100a62c4).wrapping_sub(1)
            );
        }
        mech_trace::wr!(
            "1005e5c0",
            mech_trace::gr!("1005e5c0", "100c61b4", DAT_100c61b4).cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1005e5c0", "10095aa0", DAT_10095aa0)
        );
        mech_trace::wr!(
            "1005e5c0",
            mech_trace::gr!("1005e5c0", "100c61b4", DAT_100c61b4)
                .byte_add(4)
                .cast::<u32>(),
            write_unaligned,
            8
        );
        mech_trace::gw!(
            "1005e5c0",
            "100c61b4",
            DAT_100c61b4,
            mech_trace::gr!("1005e5c0", "100c61b4", DAT_100c61b4).byte_add(8)
        );
        let mut eax = mech_trace::gr!("1005e5c0", "100a50f0", DAT_100a50f0);
        if eax <= 8 {
            eax = 8;
        }
        let was_zero = mech_trace::gr!("1005e5c0", "100d0d40", DAT_100d0d40) == 0;
        mech_trace::gw!("1005e5c0", "100a50f0", DAT_100a50f0, eax);
        if was_zero {
            return;
        }
        mech_trace::gw!(
            "1005e5c0",
            "100d0d40",
            DAT_100d0d40,
            mech_trace::gr!("1005e5c0", "100d0d40", DAT_100d0d40).wrapping_sub(1)
        );
        if mech_trace::gr!("1005e5c0", "100d0d40", DAT_100d0d40) == 0 {
            return;
        }
        loop {
            let idx = mech_trace::gr!("1005e5c0", "100d0d40", DAT_100d0d40) as usize;
            let v = mech_trace::rd!(
                "1005e5c0",
                (&raw const DAT_100d0d50).add(idx),
                read_unaligned
            );
            if v == 0 {
                return;
            }
            mech_trace::gw!(
                "1005e5c0",
                "100d0d40",
                DAT_100d0d40,
                mech_trace::gr!("1005e5c0", "100d0d40", DAT_100d0d40).wrapping_sub(1)
            );
            if mech_trace::gr!("1005e5c0", "100d0d40", DAT_100d0d40) == 0 {
                return;
            }
        }
    }
}
