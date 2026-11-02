// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32) -> u8;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10015b30(param_1: i32, param_2: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004a5ea(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
) {
    mech_trace::enter!("1004a5ea", param_1, param_2, param_3, param_4);
    let mut local_10: [u32; 3] = [0; 3];
    let base = param_1 as usize;
    // SAFETY: param_1 points at the game's object, the output pointers are the
    // caller's, and the callees and global are the game's own.
    unsafe {
        local_10[2] = mech_trace::rd!("1004a5ea", ((base + 0x14) as *const i32), read_unaligned)
            .wrapping_neg() as u32;
        local_10[1] = 0;
        local_10[0] = mech_trace::rd!("1004a5ea", ((base + 0xc) as *const u32), read_unaligned);
        let p = local_10.as_mut_ptr();
        mech_trace::cr!("1004a5ea", "1000fef4", FUN_1000fef4(p.add(2), p.add(1), p));

        let mut i_var1 = mech_trace::cr!(
            "1004a5ea",
            "10006b20",
            FUN_10006b20(mech_trace::rd!(
                "1004a5ea",
                ((base + 0x2c) as *const i32),
                read_unaligned
            ))
        );
        i_var1 = mech_trace::cr!(
            "1004a5ea",
            "10015b30",
            FUN_10015b30(local_10[2] as i32, i_var1)
        );
        mech_trace::wr!(
            "1004a5ea",
            param_2,
            write_unaligned,
            mech_trace::rd!("1004a5ea", param_2, read_unaligned).wrapping_add(i_var1 >> 9)
        );

        i_var1 = mech_trace::cr!(
            "1004a5ea",
            "10006b20",
            FUN_10006b20(mech_trace::rd!(
                "1004a5ea",
                ((base + 0x2c) as *const i32),
                read_unaligned
            ))
        );
        i_var1 = mech_trace::cr!(
            "1004a5ea",
            "10015b30",
            FUN_10015b30(local_10[0] as i32, i_var1)
        );
        mech_trace::wr!(
            "1004a5ea",
            param_4,
            write_unaligned,
            mech_trace::rd!("1004a5ea", param_4, read_unaligned).wrapping_add(i_var1 >> 9)
        );

        i_var1 = mech_trace::cr!(
            "1004a5ea",
            "10006b9a",
            FUN_10006b9a(mech_trace::rd!(
                "1004a5ea",
                ((base + 0x2c) as *const i32),
                read_unaligned
            ))
        );
        i_var1 = mech_trace::cr!(
            "1004a5ea",
            "10015b30",
            FUN_10015b30(local_10[0] as i32, i_var1)
        );
        mech_trace::wr!(
            "1004a5ea",
            param_3,
            write_unaligned,
            mech_trace::rd!("1004a5ea", param_3, read_unaligned).wrapping_add(i_var1 >> 10)
        );

        let q = (base + 0x2c) as *mut i32;
        let delta =
            mech_trace::gr!("1004a5ea", "100b3904", DAT_100b3904).wrapping_mul(0x3fa57) as i32;
        mech_trace::wr!(
            "1004a5ea",
            q,
            write_unaligned,
            mech_trace::rd!("1004a5ea", q, read_unaligned).wrapping_add(delta)
        );
        mech_trace::wr!(
            "1004a5ea",
            q,
            write_unaligned,
            mech_trace::rd!("1004a5ea", q, read_unaligned) % 0x1680000
        );
    }
}
