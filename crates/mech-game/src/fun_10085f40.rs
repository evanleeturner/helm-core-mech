// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9054: u32;
    static mut DAT_100c9058: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10085f40(param_1: i32) {
    mech_trace::enter!("10085f40", param_1);
    // SAFETY: param_1 points at the caller's node; the globals are the game's
    // own memory; reads and writes follow the original's order.
    unsafe {
        let p = param_1 as u32;
        if mech_trace::rd!(
            "10085f40",
            ((param_1 as usize + 2) as *const i16),
            read_unaligned
        ) != 0
        {
            mech_trace::wr!(
                "10085f40",
                ((param_1 as usize + 2) as *mut u16),
                write_unaligned,
                0
            );
            if mech_trace::gr!("10085f40", "100c9058", DAT_100c9058) != 0 {
                mech_trace::wr!(
                    "10085f40",
                    ((mech_trace::gr!("10085f40", "100c9058", DAT_100c9058) as usize + 0xc)
                        as *mut i32),
                    write_unaligned,
                    param_1
                );
            }
            mech_trace::wr!(
                "10085f40",
                ((param_1 as usize + 0x10) as *mut u32),
                write_unaligned,
                mech_trace::gr!("10085f40", "100c9058", DAT_100c9058)
            );
            mech_trace::gw!("10085f40", "100c9058", DAT_100c9058, p);
            mech_trace::wr!(
                "10085f40",
                ((param_1 as usize + 0xc) as *mut u32),
                write_unaligned,
                0
            );
            if mech_trace::gr!("10085f40", "100c9054", DAT_100c9054) == 0 {
                mech_trace::gw!("10085f40", "100c9054", DAT_100c9054, p);
            }
        }
    }
}
