// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_101686f0: u32;
    static mut DAT_100af0b8: u32;
    static mut DAT_100ad9b4: u8;
    static mut DAT_100ad9c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002e711(param_1: i32) {
    mech_trace::enter!("1002e711", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's object, read at the same offsets as the original.
    unsafe {
        if mech_trace::rd!(
            "1002e711",
            ((param_1.wrapping_add(4)) as *const i16),
            read_unaligned
        ) != 0
        {
            let idx = mech_trace::gr!("1002e711", "100adf38", DAT_100adf38) as usize;
            let e = mech_trace::rd!(
                "1002e711",
                (&raw mut DAT_100eef40).cast::<u32>().add(idx),
                read_unaligned
            );
            let p20 = mech_trace::rd!(
                "1002e711",
                (e.wrapping_add(0x20) as *const u32),
                read_unaligned
            );
            let i_var1 = mech_trace::rd!(
                "1002e711",
                (p20.wrapping_add(0x3f0) as *const i32),
                read_unaligned
            );
            let local_8: *mut u8;
            if i_var1 == 1 {
                local_8 = &raw mut DAT_100ad9b4;
            } else {
                if i_var1 != 2 {
                    return;
                }
                local_8 = &raw mut DAT_100ad9c0;
            }
            let pu_var2 = mech_trace::cr!(
                "1002e711",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1002e711", "100af17c", DAT_100af17c) as i32,
                    mech_trace::gr!("1002e711", "101686f0", DAT_101686f0).wrapping_add(1),
                    mech_trace::gr!("1002e711", "100af0b8", DAT_100af0b8) as *mut i32,
                )
            );
            if !pu_var2.is_null() {
                if mech_trace::rd!("1002e711", local_8, read) != 0 {
                    let a4 = mech_trace::rd!(
                        "1002e711",
                        (param_1.wrapping_add(0xa4) as *const u32),
                        read_unaligned
                    );
                    let a8 = mech_trace::rd!(
                        "1002e711",
                        (param_1.wrapping_add(0xa8) as *const u32),
                        read_unaligned
                    );
                    let v0 = mech_trace::rd!("1002e711", (a8 as *const i32), read_unaligned);
                    let v1 = mech_trace::rd!(
                        "1002e711",
                        (a8.wrapping_add(4) as *const i32),
                        read_unaligned
                    );
                    mech_trace::cr!(
                        "1002e711",
                        "1005acaa",
                        FUN_1005acaa(a4 as *mut i32, v0, v1, pu_var2 as i32, local_8, 0x100eebb0)
                    );
                }
                mech_trace::cr!(
                    "1002e711",
                    "1008664f",
                    FUN_1008664f(
                        mech_trace::gr!("1002e711", "101686f0", DAT_101686f0).wrapping_add(1),
                        mech_trace::gr!("1002e711", "100af0b8", DAT_100af0b8) as *mut i32
                    )
                );
            }
        }
    }
}
