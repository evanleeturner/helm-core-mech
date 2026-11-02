// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: i32;
    static mut DAT_10160c00: [i32; 1];
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
    static mut DAT_100aa558: i32;
    static mut DAT_100aa55c: i32;
    static mut DAT_100aa560: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002634e(param_1: *mut i32) -> i32;
    fn FUN_100145d4(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32) -> u32;
    fn FUN_1003d2d3(param_1: i32, param_2: i32);
    fn FUN_1001bc2d(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_100267fd(param_1: *mut i32, param_2: *mut i32);
    fn FUN_10014b90(param_1: i32, param_2: *mut i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014768(param_1: *mut i32, param_2: *mut i32, param_3: u32) -> u32 {
    mech_trace::enter!("10014768", param_1, param_2, param_3);
    let mut local_88: [i32; 14] = [0; 14];
    let mut local_48: [i32; 14] = [0; 14];
    let mut local_50: i32 = 0;
    let mut local_8c: i32 = 0;
    let mut local_c: i32 = 0x7fffffff;
    // SAFETY: callees and globals are the game's own; param_1 and param_2
    // point at the caller's data, and list nodes are read at the original's
    // offsets.
    unsafe {
        let mut i_var1 = mech_trace::cr!("10014768", "1002634e", FUN_1002634e(param_1));
        if i_var1 < 1 {
            return mech_trace::rt!(
                "10014768",
                mech_trace::cr!(
                    "10014768",
                    "100145d4",
                    FUN_100145d4(
                        mech_trace::rd!("10014768", param_1, read),
                        mech_trace::rd!("10014768", param_1.add(1), read),
                        mech_trace::rd!("10014768", param_1.add(2), read),
                        param_2,
                    )
                )
            );
        }
        let mut local_10 = mech_trace::gr!("10014768", "100af098", DAT_100af098);
        if mech_trace::gr!("10014768", "100af098", DAT_100af098) == 0 {
            return mech_trace::rt!("10014768", 0);
        }
        FUN_1003d2d3(
            mech_trace::rd!("10014768", param_1, read),
            mech_trace::rd!("10014768", param_1.add(2), read),
        );
        local_10 = mech_trace::rd!("10014768", (&raw mut DAT_10160c00).cast::<i32>(), read);
        while local_10 != 0 {
            local_8c = local_8c.wrapping_add(1);
            let mut local_90 = mech_trace::rd!(
                "10014768",
                ((local_10.wrapping_add(0x10)) as *const i32),
                read_unaligned
            );
            while local_90 != 0 {
                let local_8 = mech_trace::rd!(
                    "10014768",
                    ((local_90.wrapping_add(2)) as *const u16),
                    read_unaligned
                );
                if ((local_8 & 0x100) == 0
                    || (u32::from(mech_trace::rd!(
                        "10014768",
                        ((local_90.wrapping_add(0x14)) as *const u16),
                        read_unaligned
                    )) != param_3
                        && (-1 < (param_3 as i32))))
                {
                    let local_4c =
                        mech_trace::cr!("10014768", "1001bc2d", FUN_1001bc2d(local_90, param_1));
                    if local_4c < local_c {
                        FUN_100267fd(local_88.as_mut_ptr(), param_1);
                        i_var1 = mech_trace::cr!(
                            "10014768",
                            "10014b90",
                            FUN_10014b90(local_90, local_88.as_mut_ptr(), local_4c)
                        );
                        if i_var1 != 0 {
                            i_var1 = mech_trace::cr!(
                                "10014768",
                                "1002634e",
                                FUN_1002634e(local_88.as_mut_ptr())
                            );
                            if i_var1 < local_c {
                                local_c = i_var1;
                                FUN_100267fd(local_48.as_mut_ptr(), local_88.as_mut_ptr());
                                local_50 = local_90;
                                mech_trace::gw!(
                                    "10014768",
                                    "100aa558",
                                    DAT_100aa558,
                                    mech_trace::gr!("10014768", "100aa540", DAT_100aa540)
                                );
                                mech_trace::gw!(
                                    "10014768",
                                    "100aa55c",
                                    DAT_100aa55c,
                                    mech_trace::gr!("10014768", "100aa544", DAT_100aa544)
                                );
                                mech_trace::gw!(
                                    "10014768",
                                    "100aa560",
                                    DAT_100aa560,
                                    mech_trace::gr!("10014768", "100aa548", DAT_100aa548)
                                );
                            }
                        }
                    }
                }
                local_90 = mech_trace::rd!(
                    "10014768",
                    ((local_90.wrapping_add(0x10)) as *const i32),
                    read_unaligned
                );
            }
            local_10 = mech_trace::rd!(
                "10014768",
                (&raw mut DAT_10160c00)
                    .cast::<i32>()
                    .offset(local_8c as isize),
                read
            );
        }
        mech_trace::wr!("10014768", param_2, write, local_50);
        mech_trace::rt!(
            "10014768",
            if local_50 == 0 {
                0
            } else {
                FUN_100267fd(param_1, local_48.as_mut_ptr());
                1
            }
        )
    }
}
