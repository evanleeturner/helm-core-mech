// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_101604d0: [u8; 1];
    static mut DAT_101604f0: [u8; 1];
    static mut DAT_10160510: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> u32;
    fn FUN_1002fc9d(param_1: i32) -> i32;
    fn FUN_10089060(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d34a(
    param_1: u32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut u32,
) -> u32 {
    mech_trace::enter!("1004d34a", param_1, param_2, param_3, param_4);
    // SAFETY: the globals and callees are the game's own; the pointers are
    // the caller's and are read and written where the original does.
    unsafe {
        let mut local_18: i32 = 0;
        mech_trace::rt!(
            "1004d34a",
            if param_1 < mech_trace::gr!("1004d34a", "100b38d8", DAT_100b38d8) {
                let tab = (&raw mut DAT_100eef40).cast::<u32>();
                let i_var1 = mech_trace::rd!(
                    "1004d34a",
                    ((mech_trace::rd!("1004d34a", tab.add(param_1 as usize), read) as usize)
                        .wrapping_add(8) as *const i32),
                    read_unaligned
                );
                let local_c = mech_trace::cr!("1004d34a", "1004d47f", FUN_1004d47f(i_var1));
                let local_8 = mech_trace::rd!(
                    "1004d34a",
                    ((mech_trace::rd!("1004d34a", tab.add(param_1 as usize), read) as usize)
                        .wrapping_add(0xc) as *const i32),
                    read_unaligned
                );
                if param_1 == local_c {
                    let base =
                        mech_trace::rd!("1004d34a", tab.add(local_c as usize), read) as usize;
                    mech_trace::wr!(
                        "1004d34a",
                        param_2,
                        write_unaligned,
                        mech_trace::rd!(
                            "1004d34a",
                            (base.wrapping_add(0x50) as *const i32),
                            read_unaligned
                        )
                    );
                    let base =
                        mech_trace::rd!("1004d34a", tab.add(local_c as usize), read) as usize;
                    mech_trace::wr!(
                        "1004d34a",
                        param_3,
                        write_unaligned,
                        mech_trace::rd!(
                            "1004d34a",
                            (base.wrapping_add(0x58) as *const i32),
                            read_unaligned
                        )
                    );
                    let base =
                        mech_trace::rd!("1004d34a", tab.add(local_c as usize), read) as usize;
                    mech_trace::wr!(
                        "1004d34a",
                        param_4,
                        write_unaligned,
                        mech_trace::rd!(
                            "1004d34a",
                            (base.wrapping_add(0x60) as *const u32),
                            read_unaligned
                        )
                    );
                } else {
                    let off = local_8
                        .wrapping_mul(4)
                        .wrapping_add(i_var1.wrapping_mul(0x70))
                        as usize;
                    mech_trace::wr!(
                        "1004d34a",
                        param_2,
                        write_unaligned,
                        mech_trace::rd!(
                            "1004d34a",
                            ((&raw mut DAT_101604d0 as usize).wrapping_add(off) as *const i32),
                            read_unaligned
                        )
                    );
                    mech_trace::wr!(
                        "1004d34a",
                        param_3,
                        write_unaligned,
                        mech_trace::rd!(
                            "1004d34a",
                            ((&raw mut DAT_101604f0 as usize).wrapping_add(off) as *const i32),
                            read_unaligned
                        )
                    );
                    mech_trace::wr!(
                        "1004d34a",
                        param_4,
                        write_unaligned,
                        mech_trace::rd!(
                            "1004d34a",
                            ((&raw mut DAT_10160510 as usize).wrapping_add(off) as *const u32),
                            read_unaligned
                        )
                    );
                    let base =
                        mech_trace::rd!("1004d34a", tab.add(local_c as usize), read) as usize;
                    let local_14 = mech_trace::rd!(
                        "1004d34a",
                        (base.wrapping_add(0x40) as *const i32),
                        read_unaligned
                    );
                    let local_10 = mech_trace::cr!("1004d34a", "1002fc9d", FUN_1002fc9d(local_14))
                        as usize as *mut i32;
                    mech_trace::cr!(
                        "1004d34a",
                        "10089060",
                        FUN_10089060(local_10, param_2, &raw mut local_18, param_3)
                    );
                }
                1
            } else {
                0
            }
        )
    }
}
