// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_10030433(param_1: i32, param_2: *mut i32, param_3: *mut i32) {
    mech_trace::enter!("10030433", param_1, param_2, param_3);
    // SAFETY: param_1 is a node address in the game's own list, and param_2 and
    // param_3 point at the list head and tail slots; all accesses mirror the original.
    unsafe {
        if param_1 != 0
            && mech_trace::rd!("10030433", param_2, read_unaligned) != 0
            && mech_trace::rd!("10030433", param_3, read_unaligned) != 0
        {
            if mech_trace::rd!("10030433", param_2, read_unaligned) == param_1 {
                if mech_trace::rd!("10030433", param_3, read_unaligned) == param_1 {
                    mech_trace::wr!("10030433", param_2, write_unaligned, 0);
                    mech_trace::wr!("10030433", param_3, write_unaligned, 0);
                } else {
                    mech_trace::wr!(
                        "10030433",
                        param_2,
                        write_unaligned,
                        mech_trace::rd!(
                            "10030433",
                            ((param_1 as usize).wrapping_add(0x10) as *const i32),
                            read_unaligned
                        )
                    );
                    mech_trace::wr!(
                        "10030433",
                        ((mech_trace::rd!("10030433", param_2, read_unaligned) as usize)
                            .wrapping_add(0xc) as *mut i32),
                        write_unaligned,
                        0
                    );
                }
            } else if mech_trace::rd!("10030433", param_3, read_unaligned) == param_1 {
                mech_trace::wr!(
                    "10030433",
                    param_3,
                    write_unaligned,
                    mech_trace::rd!(
                        "10030433",
                        ((param_1 as usize).wrapping_add(0xc) as *const i32),
                        read_unaligned
                    )
                );
                mech_trace::wr!(
                    "10030433",
                    ((mech_trace::rd!("10030433", param_3, read_unaligned) as usize)
                        .wrapping_add(0x10) as *mut i32),
                    write_unaligned,
                    0
                );
            } else {
                let i_var1 = mech_trace::rd!(
                    "10030433",
                    ((param_1 as usize).wrapping_add(0xc) as *const i32),
                    read_unaligned
                );
                let i_var2 = mech_trace::rd!(
                    "10030433",
                    ((param_1 as usize).wrapping_add(0x10) as *const i32),
                    read_unaligned
                );
                if i_var1 != 0 {
                    mech_trace::wr!(
                        "10030433",
                        ((i_var1 as usize).wrapping_add(0x10) as *mut i32),
                        write_unaligned,
                        i_var2
                    );
                }
                if i_var2 != 0 {
                    mech_trace::wr!(
                        "10030433",
                        ((i_var2 as usize).wrapping_add(0xc) as *mut i32),
                        write_unaligned,
                        i_var1
                    );
                }
            }
            mech_trace::wr!(
                "10030433",
                ((param_1 as usize).wrapping_add(0xc) as *mut i32),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10030433",
                ((param_1 as usize).wrapping_add(0x10) as *mut i32),
                write_unaligned,
                0
            );
        }
    }
}
