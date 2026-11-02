// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100a1048: f64;
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_10215658: i32;
    static mut PTR_DAT_100af0b8: *mut u8;
    static mut s__s_3_1lf_s_100a7318: [i8; 0];
    static mut s__s_3_1lf_100a7324: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_10002570(param_1: u32, param_2: i32) -> i32;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    );
    fn FUN_1008664f(param_1: i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100099a1(param_1: *mut u32) {
    mech_trace::enter!("100099a1", param_1);
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's structure of u32 fields, accessed where the original does.
    unsafe {
        let piv1 = mech_trace::rd!("100099a1", param_1, read_unaligned) as *mut i32;
        let mut i_var4 = mech_trace::rd!(
            "100099a1",
            (&raw mut DAT_100eef40)
                .cast::<i32>()
                .offset(mech_trace::gr!("100099a1", "100adf38", DAT_100adf38) as isize),
            read_unaligned
        );
        let pu_var2 = mech_trace::cr!(
            "100099a1",
            "1008668b",
            FUN_1008668b(
                mech_trace::gr!("100099a1", "100af17c", DAT_100af17c),
                (mech_trace::rd!("100099a1", param_1.add(0xc), read_unaligned) as i32)
                    .wrapping_add(mech_trace::gr!("100099a1", "101686f0", DAT_101686f0)),
                mech_trace::gr!("100099a1", "PTR_DAT_100af0b8", PTR_DAT_100af0b8).cast::<i32>(),
            )
        );
        if !pu_var2.is_null() {
            if mech_trace::rd!("100099a1", param_1.add(6), read_unaligned)
                != mech_trace::rd!("100099a1", param_1.add(7), read_unaligned)
            {
                let u_var3 = ((mech_trace::rd!("100099a1", param_1.add(6), read_unaligned) as i32)
                    / 2) as u32;
                let local_14: i32;
                let local_10: u32;
                if (u_var3 as i32) < 100000 {
                    local_14 = mech_trace::cr!("100099a1", "10002570", FUN_10002570(u_var3, 100));
                    local_10 = mech_trace::rd!("100099a1", param_1.add(0x14), read_unaligned);
                } else {
                    local_14 =
                        mech_trace::cr!("100099a1", "10002570", FUN_10002570(u_var3, 100000));
                    local_10 = mech_trace::rd!("100099a1", param_1.add(0x15), read_unaligned);
                }
                mech_trace::cr!(
                    "100099a1",
                    "_sprintf",
                    _sprintf(
                        mech_trace::rd!("100099a1", param_1.add(0x10), read_unaligned) as *mut i8,
                        (&raw const s__s_3_1lf_s_100a7318).cast::<i8>(),
                        mech_trace::rd!("100099a1", param_1.add(0xf), read_unaligned),
                        (local_14 as f64)
                            / mech_trace::gr!("100099a1", "_DAT_100a1048", _DAT_100a1048),
                        local_10,
                    )
                );
                mech_trace::wr!(
                    "100099a1",
                    param_1.add(7),
                    write_unaligned,
                    mech_trace::rd!("100099a1", param_1.add(6), read_unaligned)
                );
            }
            FUN_1005acaa(
                piv1,
                mech_trace::rd!("100099a1", param_1.add(0x18), read_unaligned),
                mech_trace::rd!("100099a1", param_1.add(0x19), read_unaligned),
                pu_var2 as i32,
                mech_trace::rd!("100099a1", param_1.add(0x10), read_unaligned) as *mut u8,
                0x100eebb0,
            );
            if mech_trace::gr!("100099a1", "10215658", DAT_10215658) == 4 {
                let v = mech_trace::rd!(
                    "100099a1",
                    ((i_var4 as usize + 0x60) as *const i32),
                    read_unaligned
                );
                i_var4 = (v % 0x1680000).wrapping_add(0x1680000) % 0x1680000;
                if mech_trace::rd!("100099a1", param_1.add(0x13), read_unaligned) as i32 != i_var4 {
                    mech_trace::cr!(
                        "100099a1",
                        "_sprintf",
                        _sprintf(
                            mech_trace::rd!("100099a1", param_1.add(0x12), read_unaligned)
                                as *mut i8,
                            (&raw const s__s_3_1lf_100a7324).cast::<i8>(),
                            mech_trace::rd!("100099a1", param_1.add(0x11), read_unaligned),
                            (i_var4 as f64)
                                / mech_trace::gr!("100099a1", "_DAT_100a1048", _DAT_100a1048),
                        )
                    );
                    mech_trace::wr!(
                        "100099a1",
                        param_1.add(0x13),
                        write_unaligned,
                        i_var4 as u32
                    );
                }
                FUN_1005acaa(
                    piv1,
                    mech_trace::rd!("100099a1", param_1.add(0x1a), read_unaligned),
                    mech_trace::rd!("100099a1", param_1.add(0x1b), read_unaligned),
                    pu_var2 as i32,
                    mech_trace::rd!("100099a1", param_1.add(0x12), read_unaligned) as *mut u8,
                    0x100eebb0,
                );
            }
            FUN_1008664f(
                (mech_trace::rd!("100099a1", param_1.add(0xc), read_unaligned) as i32)
                    .wrapping_add(mech_trace::gr!("100099a1", "101686f0", DAT_101686f0)),
                mech_trace::gr!("100099a1", "PTR_DAT_100af0b8", PTR_DAT_100af0b8).cast::<i32>(),
            );
        }
    }
}
