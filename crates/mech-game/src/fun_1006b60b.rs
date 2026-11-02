// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006b6af(param_1: u32, param_2: u32, param_3: u32) -> u8;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b60b(param_1: *mut i32, param_2: i32) {
    mech_trace::enter!("1006b60b", param_1, param_2);
    // SAFETY: param_1 points at the game's object whose first field is a
    // pointer to another game struct; the globals and callees are the game's own.
    unsafe {
        let base = mech_trace::rd!("1006b60b", param_1, read_unaligned) as u32 as usize;
        if mech_trace::rd!("1006b60b", ((base + 4) as *const i32), read_unaligned) as u32
            == mech_trace::gr!("1006b60b", "100adf38", DAT_100adf38)
        {
            mech_trace::cr!(
                "1006b60b",
                "1006b6af",
                FUN_1006b6af(0, param_2.wrapping_neg() as u32, 0)
            );
        }
        let local_8: u32 = if param_2 < -0x102762 { 0xfa } else { 0xfb };

        let a1 = mech_trace::rd!(
            "1006b60b",
            (((mech_trace::rd!("1006b60b", param_1, read_unaligned) as u32 as usize) + 0x50)
                as *const i32),
            read_unaligned
        )
        .wrapping_sub(mech_trace::rd!(
            "1006b60b",
            (mech_trace::gr!("1006b60b", "100aaf68", DAT_100aaf68) as usize as *const i32),
            read_unaligned
        ));
        let a2 = mech_trace::rd!(
            "1006b60b",
            (((mech_trace::rd!("1006b60b", param_1, read_unaligned) as u32 as usize) + 0x54)
                as *const i32),
            read_unaligned
        )
        .wrapping_sub(mech_trace::rd!(
            "1006b60b",
            param_1.byte_add(0x400),
            read_unaligned
        ))
        .wrapping_sub(mech_trace::rd!(
            "1006b60b",
            ((mech_trace::gr!("1006b60b", "100aaf68", DAT_100aaf68) as usize + 4) as *const i32),
            read_unaligned
        ));
        let a3 = mech_trace::rd!(
            "1006b60b",
            (((mech_trace::rd!("1006b60b", param_1, read_unaligned) as u32 as usize) + 0x58)
                as *const i32),
            read_unaligned
        )
        .wrapping_sub(mech_trace::rd!(
            "1006b60b",
            ((mech_trace::gr!("1006b60b", "100aaf68", DAT_100aaf68) as usize + 8) as *const i32),
            read_unaligned
        ));
        mech_trace::cr!(
            "1006b60b",
            "100037f0",
            FUN_100037f0(
                a1 as u32,
                a2 as u32,
                a3 as u32,
                local_8,
                mech_trace::gr!("1006b60b", "100ab094", DAT_100ab094) as i32,
            )
        );
    }
}
