// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100ab0a4: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100ab0a0: i32;
    static mut DAT_100ab07c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a7d4(param_1: i32, param_2: i32) {
    mech_trace::enter!("1001a7d4", param_1, param_2);
    // SAFETY: the globals are the game's own memory; the table index and the
    // entry pointer are read exactly as the original does.
    unsafe {
        let mut local_8: i32 = -1;
        if mech_trace::gr!("1001a7d4", "100b38d8", DAT_100b38d8) != 0 {
            if param_2 == 0 {
                if param_1 != 0 {
                    local_8 = 1;
                }
            } else {
                mech_trace::gw!(
                    "1001a7d4",
                    "100ab0a4",
                    DAT_100ab0a4,
                    mech_trace::gr!("1001a7d4", "100adf38", DAT_100adf38)
                );
                local_8 = 0;
            }
            mech_trace::gw!(
                "1001a7d4",
                "100ab0a4",
                DAT_100ab0a4,
                mech_trace::gr!("1001a7d4", "100ab0a4", DAT_100ab0a4).wrapping_add(local_8)
            );
            if mech_trace::gr!("1001a7d4", "100ab0a4", DAT_100ab0a4) < 0 {
                mech_trace::gw!(
                    "1001a7d4",
                    "100ab0a4",
                    DAT_100ab0a4,
                    mech_trace::gr!("1001a7d4", "100b38d8", DAT_100b38d8)
                        .wrapping_add(0xffff_ffffu32 as i32)
                );
            } else if mech_trace::gr!("1001a7d4", "100b38d8", DAT_100b38d8)
                <= mech_trace::gr!("1001a7d4", "100ab0a4", DAT_100ab0a4)
            {
                mech_trace::gw!("1001a7d4", "100ab0a4", DAT_100ab0a4, 0);
            }
            let base = (&raw mut DAT_100eef40).cast::<i32>();
            mech_trace::gw!(
                "1001a7d4",
                "100ab0a0",
                DAT_100ab0a0,
                mech_trace::rd!(
                    "1001a7d4",
                    base.offset(mech_trace::gr!("1001a7d4", "100ab0a4", DAT_100ab0a4) as isize),
                    read_unaligned
                )
            );
            let flags = mech_trace::rd!(
                "1001a7d4",
                ((mech_trace::gr!("1001a7d4", "100ab0a0", DAT_100ab0a0) as u32).wrapping_add(0x14)
                    as *const u16),
                read_unaligned
            );
            if (flags & 0x4800) != 0 {
                FUN_1001a7d4(param_1, param_2);
            }
            mech_trace::gw!("1001a7d4", "100ab07c", DAT_100ab07c, 0xffff_ffffu32);
        }
    }
}
