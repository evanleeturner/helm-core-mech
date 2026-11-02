// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab37c: *mut i32;
    static mut DAT_100ab380: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100239a4(param_1: *mut i32) {
    mech_trace::enter!("100239a4", param_1);
    // SAFETY: the globals are the game's own; param_1 and the linked nodes
    // are the game's list nodes, read and written where the original does.
    unsafe {
        if !param_1.is_null()
            && !mech_trace::gr!("100239a4", "100ab37c", DAT_100ab37c).is_null()
            && !mech_trace::gr!("100239a4", "100ab380", DAT_100ab380).is_null()
        {
            if mech_trace::gr!("100239a4", "100ab37c", DAT_100ab37c) == param_1 {
                if mech_trace::gr!("100239a4", "100ab380", DAT_100ab380) == param_1 {
                    mech_trace::gw!("100239a4", "100ab37c", DAT_100ab37c, core::ptr::null_mut());
                    mech_trace::gw!("100239a4", "100ab380", DAT_100ab380, core::ptr::null_mut());
                } else {
                    mech_trace::gw!(
                        "100239a4",
                        "100ab37c",
                        DAT_100ab37c,
                        mech_trace::rd!(
                            "100239a4",
                            param_1.byte_add(4).cast::<i32>(),
                            read_unaligned
                        ) as *mut i32
                    );
                    mech_trace::wr!(
                        "100239a4",
                        mech_trace::gr!("100239a4", "100ab37c", DAT_100ab37c).cast::<i32>(),
                        write_unaligned,
                        0
                    );
                }
            } else if mech_trace::gr!("100239a4", "100ab380", DAT_100ab380) == param_1 {
                mech_trace::gw!(
                    "100239a4",
                    "100ab380",
                    DAT_100ab380,
                    mech_trace::rd!("100239a4", param_1.cast::<i32>(), read_unaligned) as *mut i32
                );
                mech_trace::wr!(
                    "100239a4",
                    mech_trace::gr!("100239a4", "100ab380", DAT_100ab380)
                        .byte_add(4)
                        .cast::<i32>(),
                    write_unaligned,
                    0
                );
            } else {
                let i_var1 = mech_trace::rd!("100239a4", param_1.cast::<i32>(), read_unaligned);
                let pi_var2 = mech_trace::rd!(
                    "100239a4",
                    param_1.byte_add(4).cast::<i32>(),
                    read_unaligned
                ) as *mut i32;
                (i_var1 as usize + 4).wrapping_add(0) as *mut i32;
                mech_trace::wr!(
                    "100239a4",
                    ((i_var1 as u32).wrapping_add(4) as *mut i32),
                    write_unaligned,
                    pi_var2 as i32
                );
                mech_trace::wr!("100239a4", pi_var2, write_unaligned, i_var1);
            }
            mech_trace::wr!("100239a4", param_1, write_unaligned, 0);
            mech_trace::wr!(
                "100239a4",
                param_1.byte_add(4).cast::<i32>(),
                write_unaligned,
                0
            );
        }
    }
}
