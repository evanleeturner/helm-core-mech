// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160ef0: i32;
    static mut DAT_10160ebc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b8f6(param_1: i32, param_2: i32) {
    mech_trace::enter!("1003b8f6", param_1, param_2);
    // SAFETY: the globals are the game's own; param_1 is an address of a
    // caller-owned structure, written at the offsets the original uses.
    unsafe {
        mech_trace::gw!("1003b8f6", "10160ef0", DAT_10160ef0, param_2);
        mech_trace::wr!(
            "1003b8f6",
            ((param_1 as usize + 0x40) as *mut i32),
            write_unaligned,
            param_2
        );
        if param_2 < 0x1fffffff {
            mech_trace::gw!(
                "1003b8f6",
                "10160ebc",
                DAT_10160ebc,
                (param_2 as u32).wrapping_shl(2)
            );
            mech_trace::wr!(
                "1003b8f6",
                ((param_1 as usize + 0xb4) as *mut i32),
                write_unaligned,
                param_2
            );
        } else {
            mech_trace::gw!("1003b8f6", "10160ebc", DAT_10160ebc, 0x7fffffff);
            mech_trace::wr!(
                "1003b8f6",
                ((param_1 as usize + 0xb4) as *mut i32),
                write_unaligned,
                0x7fffffff
            );
        }
    }
}
